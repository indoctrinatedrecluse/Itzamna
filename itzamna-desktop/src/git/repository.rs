//! Git Sanctuary — Real repository operations powered by git2-rs

use git2::{
    BlameOptions, BranchType, Commit, IndexAddOption, Oid, Repository, ResetType, Signature,
    StatusOptions,
};
use std::path::Path;

use crate::git::detailed_commit::DetailedCommit;

fn command_report(stdout: &[u8], stderr: &[u8], fallback: &str) -> String {
    let stdout = String::from_utf8_lossy(stdout)
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .collect::<String>();
    let stderr = String::from_utf8_lossy(stderr)
        .replace('\r', "\n")
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .collect::<String>();
    let combined = [stdout.trim().to_string(), stderr.trim().to_string()]
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    if combined.is_empty() {
        fallback.to_string()
    } else {
        combined
    }
}

#[derive(Debug, Clone)]
pub struct BlameLineInfo {
    pub line_no: usize,
    pub commit_sha: String,
    pub short_sha: String,
    pub author_name: String,
    pub author_email: String,
    pub time_secs: i64,
    pub relative_time: String,
    pub code_content: String,
    pub age_heat: f32, // 0.0 (aged/ancient) to 1.0 (recent/radiant)
}

#[derive(Debug, Clone)]
pub struct ReflogEntryInfo {
    pub index: usize,
    pub old_sha: String,
    pub new_sha: String,
    pub full_sha: String,
    pub committer_name: String,
    pub message: String,
    pub action_type: &'static str,
}

#[derive(Debug, Clone)]
pub struct StashEntryDetail {
    pub index: usize,
    pub message: String,
    pub commit_sha: String,
    pub full_sha: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeStrategy {
    FastForward,
    NoFastForward,
    Squash,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RebaseAction {
    Pick,
    Reword,
    Squash,
    Drop,
}

#[derive(Debug, Clone)]
pub struct RebasePlanStep {
    pub sha: String,
    pub full_sha: String,
    pub original_message: String,
    pub current_message: String,
    pub action: RebaseAction,
}

#[derive(Debug, Clone)]
pub enum RebaseError {
    Conflict {
        conflicted_files: Vec<String>,
        step_index: usize,
        message: String,
    },
    Fatal(String),
}

#[derive(Debug, Clone)]
pub struct GitCommitNode {
    pub sha: String,
    pub short_sha: String,
    pub summary: String,
    pub author_name: String,
    pub author_email: String,
    pub time_secs: i64,
    pub relative_time: String,
    pub parents: Vec<String>,
    pub branch_lane: usize,
    pub is_head: bool,
    pub tags: Vec<String>,
    pub branches: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FileStatus {
    pub path: String,
    pub status_code: &'static str,
    pub is_staged: bool,
}

pub struct GitSanctuary {
    repo: Repository,
}

impl GitSanctuary {
    pub fn open(path: &str) -> Result<Self, String> {
        let repo = Repository::discover(path).map_err(|e| e.to_string())?;
        Ok(Self { repo })
    }

    /// Compute repository telemetry from real commits, diffs, release tags, and authors.
    pub fn calculate_local_telemetry(
        &self,
    ) -> Result<crate::git::telemetry::RepoHealthOverview, String> {
        use std::collections::HashMap;
        let mut walk = self.repo.revwalk().map_err(|e| e.to_string())?;
        walk.push_head().map_err(|e| e.to_string())?;
        walk.set_sorting(git2::Sort::TIME | git2::Sort::TOPOLOGICAL)
            .map_err(|e| e.to_string())?;
        let now = chrono::Utc::now().timestamp();
        let mut author_counts: HashMap<(String, String), usize> = HashMap::new();
        let mut commit_count = 0usize;
        let mut added = 0u64;
        let mut deleted = 0u64;
        let mut failures = 0usize;
        let mut merge_lead_hours = Vec::new();
        for oid in walk.take(500) {
            let commit = self
                .repo
                .find_commit(oid.map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
            let message = commit.summary().unwrap_or("").to_lowercase();
            if message.starts_with("fix")
                || message.starts_with("revert")
                || message.contains("bugfix")
            {
                failures += 1;
            }
            let author = commit.author();
            *author_counts
                .entry((
                    author.name().unwrap_or("Unknown").to_string(),
                    author.email().unwrap_or("").to_string(),
                ))
                .or_default() += 1;
            if commit.parent_count() > 1 {
                if let Ok(parent) = commit.parent(1) {
                    merge_lead_hours.push(
                        ((commit.time().seconds() - parent.time().seconds()).max(0) as f64)
                            / 3600.0,
                    );
                }
            }
            let tree = commit.tree().map_err(|e| e.to_string())?;
            let diff = if commit.parent_count() > 0 {
                let parent = commit.parent(0).map_err(|e| e.to_string())?;
                self.repo
                    .diff_tree_to_tree(
                        Some(&parent.tree().map_err(|e| e.to_string())?),
                        Some(&tree),
                        None,
                    )
                    .map_err(|e| e.to_string())?
            } else {
                self.repo
                    .diff_tree_to_tree(None, Some(&tree), None)
                    .map_err(|e| e.to_string())?
            };
            let stats = diff.stats().map_err(|e| e.to_string())?;
            added += stats.insertions() as u64;
            deleted += stats.deletions() as u64;
            commit_count += 1;
        }
        let mut recent_release_tags = 0usize;
        for tag in self
            .repo
            .tag_names(None)
            .map_err(|e| e.to_string())?
            .iter()
            .flatten()
        {
            if let Ok(obj) = self.repo.revparse_single(tag) {
                if let Ok(target) = obj.peel_to_commit() {
                    let age_seconds = now - target.time().seconds();
                    if (0..=90 * 24 * 60 * 60).contains(&age_seconds) {
                        recent_release_tags += 1;
                    }
                }
            }
        }
        let release_tags = recent_release_tags;
        let total_commits = commit_count.max(1);
        let mut contributors: Vec<_> = author_counts
            .into_iter()
            .map(
                |((name, email), count)| crate::git::telemetry::ContributorInsight {
                    name,
                    email,
                    commit_count: count,
                    ownership_percent: count as f64 * 100.0 / total_commits as f64,
                },
            )
            .collect();
        contributors.sort_by_key(|c| std::cmp::Reverse(c.commit_count));
        let mut bus_factor = 0usize;
        let mut covered = 0usize;
        for contributor in &contributors {
            bus_factor += 1;
            covered += contributor.commit_count;
            if covered * 2 >= total_commits {
                break;
            }
        }
        let failure_rate = failures as f64 * 100.0 / total_commits as f64;
        let lead_time = if merge_lead_hours.is_empty() {
            0.0
        } else {
            merge_lead_hours.iter().sum::<f64>() / merge_lead_hours.len() as f64
        };
        let health = if commit_count == 0 {
            0
        } else {
            (100.0 - failure_rate).round().clamp(0.0, 100.0) as u32
        };
        let tier = if release_tags >= 90 {
            "Elite"
        } else if release_tags >= 30 {
            "High"
        } else if release_tags >= 6 {
            "Medium"
        } else if release_tags == 0 {
            "No deployments"
        } else {
            "Low"
        };
        let risk = if bus_factor == 0 {
            "Unknown"
        } else if bus_factor == 1 {
            "High"
        } else if bus_factor <= 3 {
            "Moderate"
        } else {
            "Low"
        };
        Ok(crate::git::telemetry::RepoHealthOverview {
            health_score: health,
            dora: crate::git::telemetry::DoraMetrics {
                deployment_frequency: release_tags as f64 / 90.0,
                lead_time_hours: lead_time,
                change_failure_rate: failure_rate,
                mttr_minutes: 0.0,
                tier: tier.to_string(),
            },
            churn: crate::git::telemetry::ChurnMetrics {
                lines_added: added,
                lines_deleted: deleted,
                lines_modified: added.min(deleted),
                net_growth: added as i64 - deleted as i64,
            },
            top_contributors: contributors,
            bus_factor: crate::git::telemetry::BusFactor {
                score: bus_factor,
                risk_level: risk.to_string(),
            },
        })
    }

    pub fn get_head_info(&self) -> (String, String) {
        let branch = match self.repo.head() {
            Ok(head) => head.shorthand().unwrap_or("detached HEAD").to_string(),
            Err(_) => "empty codex".to_string(),
        };

        let sha = match self.repo.head().and_then(|h| h.peel_to_commit()) {
            Ok(c) => c.id().to_string()[..7].to_string(),
            Err(_) => "0000000".to_string(),
        };

        (branch, sha)
    }

    pub fn get_branches(&self) -> (Vec<String>, Vec<String>) {
        let mut local = Vec::new();
        let mut remote = Vec::new();

        if let Ok(branches) = self.repo.branches(Some(BranchType::Local)) {
            for item in branches.flatten() {
                if let Ok(Some(name)) = item.0.name() {
                    local.push(name.to_string());
                }
            }
        }

        if let Ok(branches) = self.repo.branches(Some(BranchType::Remote)) {
            for item in branches.flatten() {
                if let Ok(Some(name)) = item.0.name() {
                    remote.push(name.to_string());
                }
            }
        }

        (local, remote)
    }

    pub fn get_tags(&self) -> Vec<String> {
        let mut tags = Vec::new();
        if let Ok(tag_names) = self.repo.tag_names(None) {
            for tag in tag_names.iter().flatten() {
                tags.push(tag.to_string());
            }
        }
        tags
    }

    pub fn get_stashes(&mut self) -> Vec<String> {
        let mut stashes = Vec::new();
        let _ = self.repo.stash_foreach(|idx, name, _oid| {
            stashes.push(format!("cist@{}: {}", idx, name));
            true
        });
        stashes
    }

    pub fn get_status_files(&self) -> (Vec<FileStatus>, Vec<FileStatus>) {
        let mut unstaged = Vec::new();
        let mut staged = Vec::new();

        let mut opts = StatusOptions::new();
        opts.include_untracked(true).recurse_untracked_dirs(true);

        if let Ok(statuses) = self.repo.statuses(Some(&mut opts)) {
            for entry in statuses.iter() {
                let path = entry.path().unwrap_or("unknown").to_string();
                let status = entry.status();

                // Staged checks
                if status.is_index_new() {
                    staged.push(FileStatus {
                        path: path.clone(),
                        status_code: "A",
                        is_staged: true,
                    });
                } else if status.is_index_modified() {
                    staged.push(FileStatus {
                        path: path.clone(),
                        status_code: "M",
                        is_staged: true,
                    });
                } else if status.is_index_deleted() {
                    staged.push(FileStatus {
                        path: path.clone(),
                        status_code: "D",
                        is_staged: true,
                    });
                }

                // Unstaged checks
                if status.is_wt_new() {
                    unstaged.push(FileStatus {
                        path,
                        status_code: "?",
                        is_staged: false,
                    });
                } else if status.is_wt_modified() {
                    unstaged.push(FileStatus {
                        path,
                        status_code: "M",
                        is_staged: false,
                    });
                } else if status.is_wt_deleted() {
                    unstaged.push(FileStatus {
                        path,
                        status_code: "D",
                        is_staged: false,
                    });
                }
            }
        }

        (unstaged, staged)
    }

    pub fn load_commit_dag(&self, max_commits: usize) -> Vec<GitCommitNode> {
        let mut nodes = Vec::new();

        let mut revwalk = match self.repo.revwalk() {
            Ok(w) => w,
            Err(_) => return nodes,
        };

        // Push HEAD
        let _ = revwalk.push_head();

        // Push all local branch tips
        if let Ok(branches) = self.repo.branches(Some(BranchType::Local)) {
            for b in branches.flatten() {
                if let Some(target) = b.0.get().target() {
                    let _ = revwalk.push(target);
                }
            }
        }

        // Push all remote branch tips
        if let Ok(branches) = self.repo.branches(Some(BranchType::Remote)) {
            for b in branches.flatten() {
                if let Some(target) = b.0.get().target() {
                    let _ = revwalk.push(target);
                }
            }
        }

        // Push all tags
        if let Ok(tag_names) = self.repo.tag_names(None) {
            for tag in tag_names.iter().flatten() {
                if let Ok(obj) = self.repo.revparse_single(tag) {
                    let _ = revwalk.push(obj.id());
                }
            }
        }

        let _ = revwalk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::TIME);

        // Pre-build Branch & Tag Maps
        let head_oid = self.repo.head().ok().and_then(|h| h.target());
        let head_branch_name = self
            .repo
            .head()
            .ok()
            .and_then(|h| h.shorthand().map(|s| s.to_string()));

        let mut branch_map: std::collections::HashMap<Oid, Vec<String>> =
            std::collections::HashMap::new();
        if let Ok(branches) = self.repo.branches(Some(BranchType::Local)) {
            for b in branches.flatten() {
                if let (Some(target), Ok(Some(name))) = (b.0.get().target(), b.0.name()) {
                    let is_active_head = head_branch_name.as_deref() == Some(name);
                    let display_name = if is_active_head {
                        format!("HEAD -> {}", name)
                    } else {
                        name.to_string()
                    };
                    branch_map.entry(target).or_default().push(display_name);
                }
            }
        }

        if let Ok(branches) = self.repo.branches(Some(BranchType::Remote)) {
            for b in branches.flatten() {
                if let (Some(target), Ok(Some(name))) = (b.0.get().target(), b.0.name()) {
                    branch_map.entry(target).or_default().push(name.to_string());
                }
            }
        }

        let mut tag_map: std::collections::HashMap<Oid, Vec<String>> =
            std::collections::HashMap::new();
        if let Ok(tag_names) = self.repo.tag_names(None) {
            for tag in tag_names.iter().flatten() {
                if let Ok(obj) = self.repo.revparse_single(tag) {
                    if let Ok(commit) = obj.peel_to_commit() {
                        tag_map
                            .entry(commit.id())
                            .or_default()
                            .push(tag.to_string());
                    } else {
                        tag_map.entry(obj.id()).or_default().push(tag.to_string());
                    }
                }
            }
        }

        let now = chrono::Utc::now().timestamp();

        for oid_res in revwalk.take(max_commits) {
            let oid = match oid_res {
                Ok(id) => id,
                Err(_) => continue,
            };

            let commit = match self.repo.find_commit(oid) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let sha = oid.to_string();
            let short_sha = sha[..sha.len().min(7)].to_string();
            let summary = commit.summary().unwrap_or("Sacred Inscription").to_string();
            let author = commit.author();
            let author_name = author.name().unwrap_or("Ahau").to_string();
            let author_email = author.email().unwrap_or("").to_string();
            let time_secs = commit.time().seconds();

            let parents = commit
                .parent_ids()
                .map(|p| p.to_string())
                .collect::<Vec<_>>();

            let is_head = head_oid.map(|h| h == oid).unwrap_or(false);

            let diff_mins = ((now - time_secs) / 60).max(1);
            let relative_time = if diff_mins < 60 {
                format!("{} min ago", diff_mins)
            } else if diff_mins < 1440 {
                format!("{} hours ago", diff_mins / 60)
            } else {
                format!("{} days ago", diff_mins / 1440)
            };

            let tags = tag_map.get(&oid).cloned().unwrap_or_default();
            let branches = branch_map.get(&oid).cloned().unwrap_or_default();

            nodes.push(GitCommitNode {
                sha,
                short_sha,
                summary,
                author_name,
                author_email,
                time_secs,
                relative_time,
                parents,
                branch_lane: 0,
                is_head,
                tags,
                branches,
            });
        }

        // Apply topological lane packing algorithm
        Self::assign_topological_lanes(&mut nodes);

        nodes
    }

    /// Topological Lane Packing Algorithm for Git commit graphs
    pub fn assign_topological_lanes(nodes: &mut [GitCommitNode]) {
        let mut active_lanes: Vec<Option<String>> = Vec::new();

        for node in nodes.iter_mut() {
            // Check if this commit is expected in an already active lane
            let existing_lane = active_lanes.iter().position(|lane| {
                if let Some(target) = lane {
                    target == &node.sha || target == &node.short_sha
                } else {
                    false
                }
            });

            let assigned_lane = match existing_lane {
                Some(lane) => lane,
                None => {
                    // Reuse first empty slot or expand
                    if let Some(free_slot) = active_lanes.iter().position(|l| l.is_none()) {
                        free_slot
                    } else {
                        active_lanes.push(None);
                        active_lanes.len() - 1
                    }
                }
            };

            node.branch_lane = assigned_lane;
            for (lane_index, lane) in active_lanes.iter_mut().enumerate() {
                if lane_index != assigned_lane
                    && lane
                        .as_deref()
                        .is_some_and(|sha| sha == node.sha || sha == node.short_sha)
                {
                    *lane = None;
                }
            }

            // Route parent commits
            if node.parents.is_empty() {
                // Root commit finishes this lane
                active_lanes[assigned_lane] = None;
            } else {
                // First parent inherits this lane
                active_lanes[assigned_lane] = Some(node.parents[0].clone());

                // Additional parents (merges) get allocated or reuse existing lanes
                for parent_sha in node.parents.iter().skip(1) {
                    let already_assigned =
                        active_lanes.iter().any(|l| l.as_ref() == Some(parent_sha));
                    if !already_assigned {
                        if let Some(free_slot) = active_lanes.iter().position(|l| l.is_none()) {
                            active_lanes[free_slot] = Some(parent_sha.clone());
                        } else {
                            active_lanes.push(Some(parent_sha.clone()));
                        }
                    }
                }
            }

            // Trim trailing inactive lanes
            while active_lanes.last() == Some(&None) {
                active_lanes.pop();
            }
        }
    }

    pub fn find_detailed_commit(&self, short_or_full_sha: &str) -> Option<DetailedCommit> {
        let oid = if short_or_full_sha.len() == 40 {
            Oid::from_str(short_or_full_sha).ok()?
        } else {
            let mut found = None;
            if let Ok(mut revwalk) = self.repo.revwalk() {
                let _ = revwalk.push_head();
                for id in revwalk.flatten() {
                    if id.to_string().starts_with(short_or_full_sha) {
                        found = Some(id);
                        break;
                    }
                }
            }
            found?
        };

        let commit = self.repo.find_commit(oid).ok()?;
        Some(DetailedCommit::from_git2_commit(&self.repo, &commit))
    }

    // --- REAL STAGING & UNSTAGING OPERATIONS ---

    pub fn stage_path(&self, path: &str) -> Result<(), String> {
        let mut index = self.repo.index().map_err(|e| e.to_string())?;
        index.add_path(Path::new(path)).map_err(|e| e.to_string())?;
        index.write().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn unstage_path(&self, path: &str) -> Result<(), String> {
        let head = self.repo.head().map_err(|e| e.to_string())?;
        let commit = head.peel_to_commit().map_err(|e| e.to_string())?;
        self.repo
            .reset_default(Some(&commit.into_object()), [Path::new(path)])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn stage_all(&self) -> Result<(), String> {
        let mut index = self.repo.index().map_err(|e| e.to_string())?;
        index
            .add_all(["*"], IndexAddOption::DEFAULT, None)
            .map_err(|e| e.to_string())?;
        index.write().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn unstage_all(&self) -> Result<(), String> {
        let head = self.repo.head().map_err(|e| e.to_string())?;
        let commit = head.peel_to_commit().map_err(|e| e.to_string())?;
        self.repo
            .reset(commit.as_object(), ResetType::Mixed, None)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn discard_changes(&self, path: &str) -> Result<(), String> {
        let mut checkout = git2::build::CheckoutBuilder::new();
        checkout.path(path).force();
        self.repo
            .checkout_head(Some(&mut checkout))
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn discard_all(&self) -> Result<(), String> {
        let mut checkout = git2::build::CheckoutBuilder::new();
        checkout.force();
        self.repo
            .checkout_head(Some(&mut checkout))
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_working_diff_counts(
        &self,
        is_staged: bool,
    ) -> std::collections::HashMap<String, (usize, usize)> {
        let mut map = std::collections::HashMap::new();
        let mut opts = git2::DiffOptions::new();

        let diff_res = if is_staged {
            let head_tree = self
                .repo
                .head()
                .ok()
                .and_then(|h| h.peel_to_commit().ok())
                .and_then(|c| c.tree().ok());
            let index = self.repo.index().ok();
            index.and_then(|idx| {
                self.repo
                    .diff_tree_to_index(head_tree.as_ref(), Some(&idx), Some(&mut opts))
                    .ok()
            })
        } else {
            let index = self.repo.index().ok();
            self.repo
                .diff_index_to_workdir(index.as_ref(), Some(&mut opts))
                .ok()
        };

        if let Some(diff) = diff_res {
            let _ = diff.print(git2::DiffFormat::Patch, |delta, _hunk, line| {
                let path = delta
                    .new_file()
                    .path()
                    .or_else(|| delta.old_file().path())
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();
                let entry = map.entry(path).or_insert((0, 0));
                match line.origin() {
                    '+' => entry.0 += 1,
                    '-' => entry.1 += 1,
                    _ => {}
                }
                true
            });
        }
        map
    }

    pub fn apply_patch_to_index(&self, patch: &str) -> Result<(), String> {
        let mut child = std::process::Command::new("git")
            .current_dir(self.repo.workdir().unwrap_or(std::path::Path::new(".")))
            .args(["apply", "--cached", "--unidiff-zero", "-"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;

        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin
                .write_all(patch.as_bytes())
                .map_err(|e| e.to_string())?;
        }

        let output = child.wait_with_output().map_err(|e| e.to_string())?;
        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    pub fn unapply_patch_from_index(&self, patch: &str) -> Result<(), String> {
        let mut child = std::process::Command::new("git")
            .current_dir(self.repo.workdir().unwrap_or(std::path::Path::new(".")))
            .args(["apply", "--cached", "--reverse", "--unidiff-zero", "-"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;

        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin
                .write_all(patch.as_bytes())
                .map_err(|e| e.to_string())?;
        }

        let output = child.wait_with_output().map_err(|e| e.to_string())?;
        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    pub fn discard_patch_from_workdir(&self, patch: &str) -> Result<(), String> {
        let mut child = std::process::Command::new("git")
            .current_dir(self.repo.workdir().unwrap_or(std::path::Path::new(".")))
            .args(["apply", "--reverse", "--unidiff-zero", "-"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;

        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin
                .write_all(patch.as_bytes())
                .map_err(|e| e.to_string())?;
        }

        let output = child.wait_with_output().map_err(|e| e.to_string())?;
        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    // --- REAL COMMIT CREATION ---

    /// Returns the active author/committer signature according to Git hierarchy:
    /// 1. Repository configured signature (checks local .git/config, global ~/.gitconfig, system config)
    /// 2. Sanctuary credentials from local .git/config (user.name / user.email)
    /// 3. Environment variables (GIT_AUTHOR_NAME / GIT_AUTHOR_EMAIL, USERNAME, USER)
    /// 4. Graceful generic fallback ("Itzamna Scribe", "scribe@sanctuary.local")
    pub fn sanctuary_signature(&self) -> Result<Signature<'static>, String> {
        if let Ok(sig) = self.repo.signature() {
            let name = sig.name().unwrap_or("Itzamna Scribe").to_string();
            let email = sig.email().unwrap_or("scribe@sanctuary.local").to_string();
            return Signature::now(&name, &email).map_err(|e| e.to_string());
        }

        let (name, email, _) = self.get_local_credentials();
        if !name.trim().is_empty() && !email.trim().is_empty() {
            return Signature::now(name.trim(), email.trim()).map_err(|e| e.to_string());
        }

        let env_name = std::env::var("GIT_AUTHOR_NAME")
            .or_else(|_| std::env::var("GIT_COMMITTER_NAME"))
            .or_else(|_| std::env::var("USERNAME"))
            .or_else(|_| std::env::var("USER"))
            .unwrap_or_else(|_| "Itzamna Scribe".to_string());

        let env_email = std::env::var("GIT_AUTHOR_EMAIL")
            .or_else(|_| std::env::var("GIT_COMMITTER_EMAIL"))
            .unwrap_or_else(|_| {
                let clean_name = env_name.to_lowercase().replace(' ', ".");
                format!("{}@sanctuary.local", clean_name)
            });

        Signature::now(&env_name, &env_email).map_err(|e| e.to_string())
    }

    pub fn create_commit(&self, summary: &str, description: &str) -> Result<String, String> {
        let mut index = self.repo.index().map_err(|e| e.to_string())?;
        let tree_oid = index.write_tree().map_err(|e| e.to_string())?;
        let tree = self.repo.find_tree(tree_oid).map_err(|e| e.to_string())?;

        let sig = self.sanctuary_signature()?;

        let full_msg = if description.is_empty() {
            summary.to_string()
        } else {
            format!("{}\n\n{}", summary, description)
        };

        let parent_commit = match self.repo.head() {
            Ok(head) => head.peel_to_commit().ok(),
            Err(_) => None,
        };

        let parents: Vec<&Commit> = match &parent_commit {
            Some(c) => vec![c],
            None => vec![],
        };

        let commit_oid = self
            .repo
            .commit(Some("HEAD"), &sig, &sig, &full_msg, &tree, &parents)
            .map_err(|e| e.to_string())?;

        Ok(commit_oid.to_string())
    }

    pub fn amend_commit(&self, summary: &str, description: &str) -> Result<String, String> {
        let head = self.repo.head().map_err(|e| e.to_string())?;
        let commit = head.peel_to_commit().map_err(|e| e.to_string())?;

        let mut index = self.repo.index().map_err(|e| e.to_string())?;
        let tree_oid = index.write_tree().map_err(|e| e.to_string())?;
        let tree = self.repo.find_tree(tree_oid).map_err(|e| e.to_string())?;

        let sig = self.sanctuary_signature()?;

        let full_msg = if description.is_empty() {
            summary.to_string()
        } else {
            format!("{}\n\n{}", summary, description)
        };

        let new_oid = commit
            .amend(
                Some("HEAD"),
                Some(&sig),
                Some(&sig),
                None,
                Some(&full_msg),
                Some(&tree),
            )
            .map_err(|e| e.to_string())?;

        Ok(new_oid.to_string())
    }

    pub fn get_local_credentials(&self) -> (String, String, String) {
        let config = self.repo.config().ok();
        let name = config
            .as_ref()
            .and_then(|c| c.get_string("user.name").ok())
            .unwrap_or_default();
        let email = config
            .as_ref()
            .and_then(|c| c.get_string("user.email").ok())
            .unwrap_or_default();
        let remote_url = self
            .repo
            .find_remote("origin")
            .ok()
            .and_then(|r| r.url().map(|s| s.to_string()))
            .unwrap_or_default();
        (name, email, remote_url)
    }

    pub fn set_local_credentials(
        &self,
        name: &str,
        email: &str,
        remote_url: &str,
    ) -> Result<(), String> {
        let mut config = self.repo.config().map_err(|e| e.to_string())?;
        config
            .set_str("user.name", name)
            .map_err(|e| e.to_string())?;
        config
            .set_str("user.email", email)
            .map_err(|e| e.to_string())?;

        if !remote_url.is_empty() {
            if self.repo.find_remote("origin").is_ok() {
                self.repo
                    .remote_set_url("origin", remote_url)
                    .map_err(|e| e.to_string())?;
            } else {
                self.repo
                    .remote("origin", remote_url)
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub fn push_origin(&self, branch_name: &str) -> Result<String, String> {
        self.push_origin_with_lease(branch_name, false)
    }

    pub fn push_origin_with_lease(
        &self,
        branch_name: &str,
        force_with_lease: bool,
    ) -> Result<String, String> {
        self.push_origin_with_progress(branch_name, force_with_lease, |_| {})
    }

    pub fn push_origin_with_progress(
        &self,
        branch_name: &str,
        force_with_lease: bool,
        mut progress: impl FnMut(String),
    ) -> Result<String, String> {
        let branch = if branch_name.is_empty() || branch_name == "detached HEAD" {
            "HEAD"
        } else {
            branch_name
        };
        let refspec = if branch == "HEAD" {
            "HEAD".to_string()
        } else {
            format!("HEAD:refs/heads/{}", branch)
        };
        let mut args = vec!["push".to_string(), "--progress".to_string()];
        if force_with_lease {
            args.push("--force-with-lease".to_string());
        }
        args.extend(["origin".to_string(), refspec]);
        self.run_git_with_progress(&args, &mut progress, "Pushed cleanly to origin")
    }

    pub fn pull_rebase(&self, branch_name: &str) -> Result<String, String> {
        self.pull_rebase_with_progress(branch_name, |_| {})
    }

    pub fn pull_rebase_with_progress(
        &self,
        branch_name: &str,
        mut progress: impl FnMut(String),
    ) -> Result<String, String> {
        let branch = if branch_name.is_empty() || branch_name == "detached HEAD" {
            "main"
        } else {
            branch_name
        };
        let refspec = format!("refs/heads/{}", branch);
        let args = vec![
            "pull".into(),
            "--progress".into(),
            "--rebase".into(),
            "origin".into(),
            refspec,
        ];
        self.run_git_with_progress(
            &args,
            &mut progress,
            "Pulled and rebased cleanly from origin",
        )
    }

    pub fn fetch_prune(&self) -> Result<String, String> {
        self.fetch_prune_with_progress(|_| {})
    }

    pub fn fetch_prune_with_progress(
        &self,
        mut progress: impl FnMut(String),
    ) -> Result<String, String> {
        self.run_git_with_progress(
            &["fetch".into(), "--progress".into(), "--prune".into()],
            &mut progress,
            "Fetched and pruned remote tracking refs",
        )
    }

    fn run_git_with_progress(
        &self,
        args: &[String],
        progress: &mut dyn FnMut(String),
        fallback: &str,
    ) -> Result<String, String> {
        use std::io::Read;
        let mut child = std::process::Command::new("git")
            .current_dir(self.repo.workdir().unwrap_or(std::path::Path::new(".")))
            .args(args)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;
        let mut stdout = child
            .stdout
            .take()
            .ok_or_else(|| "Could not read Git output".to_string())?;
        let stdout_reader = std::thread::spawn(move || {
            let mut bytes = Vec::new();
            let _ = stdout.read_to_end(&mut bytes);
            bytes
        });
        let mut stderr = child
            .stderr
            .take()
            .ok_or_else(|| "Could not read Git progress".to_string())?;
        let mut stderr_bytes = Vec::new();
        let mut chunk = [0u8; 2048];
        let mut pending = Vec::new();
        loop {
            let count = stderr.read(&mut chunk).map_err(|e| e.to_string())?;
            if count == 0 {
                break;
            }
            stderr_bytes.extend_from_slice(&chunk[..count]);
            for byte in &chunk[..count] {
                if *byte == b'\r' || *byte == b'\n' {
                    if !pending.is_empty() {
                        let line = String::from_utf8_lossy(&pending)
                            .chars()
                            .filter(|c| !c.is_control() || *c == '\t')
                            .collect::<String>();
                        if !line.trim().is_empty() {
                            progress(line);
                        }
                        pending.clear();
                    }
                } else {
                    pending.push(*byte);
                }
            }
        }
        if !pending.is_empty() {
            let line = String::from_utf8_lossy(&pending)
                .chars()
                .filter(|c| !c.is_control() || *c == '\t')
                .collect::<String>();
            if !line.trim().is_empty() {
                progress(line);
            }
        }
        let status = child.wait().map_err(|e| e.to_string())?;
        let stdout = stdout_reader.join().unwrap_or_default();
        if status.success() {
            Ok(command_report(&stdout, &stderr_bytes, fallback))
        } else {
            Err(command_report(
                &stdout,
                &stderr_bytes,
                "Git remote operation failed",
            ))
        }
    }

    // --- REAL BRANCH OPERATIONS ---

    pub fn switch_branch(&self, branch_name: &str) -> Result<(), String> {
        let ref_name = format!("refs/heads/{}", branch_name);
        let obj = self
            .repo
            .revparse_single(&ref_name)
            .map_err(|e| e.to_string())?;

        let mut checkout = git2::build::CheckoutBuilder::new();
        checkout.safe();

        self.repo
            .checkout_tree(&obj, Some(&mut checkout))
            .map_err(|e| e.to_string())?;

        self.repo.set_head(&ref_name).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn create_branch(&self, branch_name: &str) -> Result<(), String> {
        let head = self.repo.head().map_err(|e| e.to_string())?;
        let commit = head.peel_to_commit().map_err(|e| e.to_string())?;
        self.repo
            .branch(branch_name, &commit, false)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn list_workspace_files(&self) -> Vec<String> {
        let mut files = Vec::new();
        let workdir = match self.repo.workdir() {
            Some(w) => w,
            None => return files,
        };

        fn scan_dir(dir: &std::path::Path, base: &std::path::Path, out: &mut Vec<String>) {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    if entry
                        .file_type()
                        .map(|kind| kind.is_symlink())
                        .unwrap_or(true)
                    {
                        continue;
                    }
                    let path = entry.path();
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name == ".git" || name == "target" || name == "node_modules" {
                        continue;
                    }
                    if path.is_file() {
                        if let Ok(rel) = path.strip_prefix(base) {
                            out.push(rel.to_string_lossy().replace('\\', "/"));
                        }
                    } else if path.is_dir() {
                        scan_dir(&path, base, out);
                    }
                }
            }
        }

        scan_dir(workdir, workdir, &mut files);
        files.sort();
        files
    }

    pub fn create_tag(&self, tag_name: &str) -> Result<(), String> {
        let head = self.repo.head().map_err(|e| e.to_string())?;
        let obj = head
            .peel_to_commit()
            .map_err(|e| e.to_string())?
            .into_object();
        let sig = self.sanctuary_signature()?;

        self.repo
            .tag(tag_name, &obj, &sig, tag_name, false)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn create_stash(&mut self, message: &str) -> Result<(), String> {
        let sig = self.sanctuary_signature()?;

        self.repo
            .stash_save(&sig, message, None)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn pop_stash(&mut self, index: usize) -> Result<(), String> {
        self.repo
            .stash_pop(index, None)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    // --- PHASE 6: PANTHEON OF RITUALS SUITE ---

    /// 1. The Ahau Lineage: Interactive Git Blame & Heatmap
    pub fn blame_file(&self, file_path: &str) -> Result<Vec<BlameLineInfo>, String> {
        let mut opts = BlameOptions::new();
        let blame = self
            .repo
            .blame_file(Path::new(file_path), Some(&mut opts))
            .map_err(|e| e.to_string())?;

        let file_content = std::fs::read_to_string(file_path).unwrap_or_default();
        let lines: Vec<&str> = file_content.lines().collect();

        let now = chrono::Utc::now().timestamp();
        let mut result = Vec::new();

        for (idx, line_str) in lines.iter().enumerate() {
            let line_no = idx + 1;
            if let Some(hunk) = blame.get_line(line_no) {
                let sig = hunk.final_signature();
                let commit_oid = hunk.final_commit_id();
                let sha = commit_oid.to_string();
                let short_sha = sha[..7.min(sha.len())].to_string();

                let author_name = sig.name().unwrap_or("Sacred Architect").to_string();
                let author_email = sig.email().unwrap_or("architect@itzamna.dev").to_string();
                let time_secs = sig.when().seconds();
                let age_days = ((now - time_secs) / 86400).max(0);

                let relative_time = if age_days == 0 {
                    "Today".to_string()
                } else if age_days == 1 {
                    "Yesterday".to_string()
                } else if age_days < 30 {
                    format!("{}d ago", age_days)
                } else if age_days < 365 {
                    format!("{}mo ago", age_days / 30)
                } else {
                    format!("{}y ago", age_days / 365)
                };

                // Age heat: 1.0 (recent, <7 days) to 0.0 (ancient, >365 days)
                let age_heat = (1.0 - (age_days as f32 / 365.0)).clamp(0.05, 1.0);

                result.push(BlameLineInfo {
                    line_no,
                    commit_sha: sha,
                    short_sha,
                    author_name,
                    author_email,
                    time_secs,
                    relative_time,
                    code_content: line_str.to_string(),
                    age_heat,
                });
            } else {
                result.push(BlameLineInfo {
                    line_no,
                    commit_sha: "0000000".to_string(),
                    short_sha: "0000000".to_string(),
                    author_name: "Uncommitted".to_string(),
                    author_email: "".to_string(),
                    time_secs: now,
                    relative_time: "Now".to_string(),
                    code_content: line_str.to_string(),
                    age_heat: 1.0,
                });
            }
        }

        Ok(result)
    }

    /// 2. The Xibalba Mirror: Visual Reflog & Time Travel
    pub fn get_reflog(&self) -> Result<Vec<ReflogEntryInfo>, String> {
        let reflog = self.repo.reflog("HEAD").map_err(|e| e.to_string())?;
        let mut entries = Vec::new();

        for (idx, entry) in reflog.iter().enumerate() {
            let msg = entry.message().unwrap_or("State shift").to_string();
            let action_type = if msg.starts_with("checkout:") {
                "checkout"
            } else if msg.starts_with("commit:") || msg.starts_with("commit (initial):") {
                "commit"
            } else if msg.starts_with("rebase:") {
                "rebase"
            } else if msg.starts_with("reset:") {
                "reset"
            } else if msg.starts_with("merge:") {
                "merge"
            } else {
                "ritual"
            };

            let old_id = entry.id_old().to_string();
            let new_id = entry.id_new().to_string();
            let old_short = if old_id.len() >= 7 {
                old_id[..7].to_string()
            } else {
                old_id.clone()
            };
            let new_short = if new_id.len() >= 7 {
                new_id[..7].to_string()
            } else {
                new_id.clone()
            };

            entries.push(ReflogEntryInfo {
                index: idx,
                old_sha: old_short,
                new_sha: new_short,
                full_sha: new_id,
                committer_name: entry.committer().name().unwrap_or("Architect").to_string(),
                message: msg,
                action_type,
            });
        }

        Ok(entries)
    }

    /// Reset HEAD to a given commit or reflog target
    pub fn reset_head(&self, target_sha: &str, reset_type: ResetType) -> Result<(), String> {
        let obj = if let Ok(oid) = Oid::from_str(target_sha) {
            self.repo
                .find_object(oid, None)
                .map_err(|e| e.to_string())?
        } else {
            self.repo
                .revparse_single(target_sha)
                .map_err(|e| e.to_string())?
        };
        self.repo
            .reset(&obj, reset_type, None)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// 3. The Popol Vuh Chronicle: Advanced Commit Search
    pub fn search_chronicle(&self, query: &str, author_filter: &str) -> Vec<GitCommitNode> {
        let all_commits = self.load_commit_dag(100);
        let q_lower = query.to_lowercase();
        let auth_lower = author_filter.to_lowercase();

        all_commits
            .into_iter()
            .filter(|c| {
                let matches_query = q_lower.is_empty()
                    || c.summary.to_lowercase().contains(&q_lower)
                    || c.short_sha.to_lowercase().contains(&q_lower)
                    || c.sha.to_lowercase().contains(&q_lower);

                let matches_author = auth_lower.is_empty()
                    || c.author_name.to_lowercase().contains(&auth_lower)
                    || c.author_email.to_lowercase().contains(&auth_lower);

                matches_query && matches_author
            })
            .collect()
    }

    /// 4. The K'uk'ulkan Confluence: Branch Merging & Cherry-Picking
    pub fn merge_branch(
        &self,
        target_branch: &str,
        strategy: MergeStrategy,
    ) -> Result<String, String> {
        let ref_name = format!("refs/heads/{}", target_branch);
        let target_commit = self
            .repo
            .revparse_single(&ref_name)
            .map_err(|e| format!("Branch '{}' not found: {}", target_branch, e))?
            .peel_to_commit()
            .map_err(|e| e.to_string())?;

        let head = self.repo.head().map_err(|e| e.to_string())?;
        let head_commit = head.peel_to_commit().map_err(|e| e.to_string())?;

        let _head_annotated = self
            .repo
            .find_annotated_commit(head_commit.id())
            .map_err(|e| e.to_string())?;
        let target_annotated = self
            .repo
            .find_annotated_commit(target_commit.id())
            .map_err(|e| e.to_string())?;

        let (analysis, _preference) = self
            .repo
            .merge_analysis_for_ref(&head, &[&target_annotated])
            .map_err(|e| e.to_string())?;

        if analysis.is_up_to_date() {
            return Ok("Already up to date: branches are in harmony.".to_string());
        }

        if analysis.is_fast_forward() && strategy == MergeStrategy::FastForward {
            let mut checkout = git2::build::CheckoutBuilder::new();
            checkout.safe();
            self.repo
                .checkout_tree(target_commit.as_object(), Some(&mut checkout))
                .map_err(|e| e.to_string())?;
            let mut head_ref = self.repo.head().map_err(|e| e.to_string())?;
            head_ref
                .set_target(
                    target_commit.id(),
                    &format!("Fast-Forward to {}", target_branch),
                )
                .map_err(|e| e.to_string())?;
            return Ok(format!(
                "Fast-Forwarded sovereign branch to [{}]",
                &target_commit.id().to_string()[..7]
            ));
        }

        // Three-way merge or squash
        let mut index = self
            .repo
            .merge_commits(&head_commit, &target_commit, None)
            .map_err(|e| e.to_string())?;

        if index.has_conflicts() {
            return Err("Merge contains conflicts! Resolve in the Conflict Crucible.".to_string());
        }

        let tree_oid = index.write_tree_to(&self.repo).map_err(|e| e.to_string())?;
        let tree = self.repo.find_tree(tree_oid).map_err(|e| e.to_string())?;
        let sig = self.sanctuary_signature()?;

        let parents: Vec<&Commit> = match strategy {
            MergeStrategy::Squash => vec![&head_commit],
            _ => vec![&head_commit, &target_commit],
        };

        let msg = format!("Merge branch '{}' into active HEAD", target_branch);
        let commit_oid = self
            .repo
            .commit(Some("HEAD"), &sig, &sig, &msg, &tree, &parents)
            .map_err(|e| e.to_string())?;

        // Sync working tree with the merged commit tree
        let mut checkout = git2::build::CheckoutBuilder::new();
        checkout.force();
        let _ = self
            .repo
            .checkout_tree(tree.as_object(), Some(&mut checkout));

        Ok(format!(
            "Consecrated merge commit [{}]",
            &commit_oid.to_string()[..7]
        ))
    }

    /// Cherry-pick a specific commit into active HEAD
    pub fn cherry_pick_commit(&self, commit_sha: &str) -> Result<String, String> {
        let commit_oid = Oid::from_str(commit_sha).map_err(|e| e.to_string())?;
        let target_commit = self
            .repo
            .find_commit(commit_oid)
            .map_err(|e| e.to_string())?;

        let head = self.repo.head().map_err(|e| e.to_string())?;
        let head_commit = head.peel_to_commit().map_err(|e| e.to_string())?;

        let mut index = self
            .repo
            .cherrypick_commit(&target_commit, &head_commit, 0, None)
            .map_err(|e| e.to_string())?;

        if index.has_conflicts() {
            return Err("Cherry-pick produced conflicts! Open Conflict Crucible.".to_string());
        }

        let tree_oid = index.write_tree_to(&self.repo).map_err(|e| e.to_string())?;
        let tree = self.repo.find_tree(tree_oid).map_err(|e| e.to_string())?;
        let sig = self.sanctuary_signature()?;

        let msg = format!(
            "cherry-pick: {}\n\n(cherry picked from commit {})",
            target_commit.summary().unwrap_or(""),
            commit_sha
        );
        let new_oid = self
            .repo
            .commit(Some("HEAD"), &sig, &sig, &msg, &tree, &[&head_commit])
            .map_err(|e| e.to_string())?;

        // Sync working tree with the plucked commit tree
        let mut checkout = git2::build::CheckoutBuilder::new();
        checkout.force();
        let _ = self
            .repo
            .checkout_tree(tree.as_object(), Some(&mut checkout));

        Ok(format!(
            "Plucked commit into [{}]",
            &new_oid.to_string()[..7]
        ))
    }

    /// 5. The Chac Purifier: Untracked Files and Stash Vault
    pub fn preview_clean_untracked(&self) -> Vec<String> {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true);
        opts.recurse_untracked_dirs(true);

        let mut untracked = Vec::new();
        if let Ok(statuses) = self.repo.statuses(Some(&mut opts)) {
            for entry in statuses.iter() {
                if entry.status().is_wt_new() {
                    if let Some(path) = entry.path() {
                        untracked.push(path.to_string());
                    }
                }
            }
        }
        untracked
    }

    pub fn purge_untracked(&self) -> Result<usize, String> {
        let files = self.preview_clean_untracked();
        let mut count = 0;
        for f in &files {
            if std::fs::remove_file(f).is_ok() {
                count += 1;
            }
        }
        Ok(count)
    }

    pub fn list_detailed_stashes(&mut self) -> Vec<StashEntryDetail> {
        let mut details = Vec::new();
        let _ = self.repo.stash_foreach(|idx, name, oid| {
            details.push(StashEntryDetail {
                index: idx,
                message: name.to_string(),
                commit_sha: oid.to_string()[..7.min(oid.to_string().len())].to_string(),
                full_sha: oid.to_string(),
            });
            true
        });
        details
    }

    pub fn drop_stash(&mut self, index: usize) -> Result<(), String> {
        self.repo.stash_drop(index).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn sprout_branch_from_stash(
        &mut self,
        stash_index: usize,
        branch_name: &str,
    ) -> Result<String, String> {
        let mut target_commit_oid: Option<Oid> = None;
        let _ = self.repo.stash_foreach(|idx, _name, oid| {
            if idx == stash_index {
                target_commit_oid = Some(*oid);
                false
            } else {
                true
            }
        });

        let stash_oid =
            target_commit_oid.ok_or_else(|| format!("Stash cist @{} not found", stash_index))?;
        let commit = self
            .repo
            .find_commit(stash_oid)
            .map_err(|e| e.to_string())?;

        self.repo
            .branch(branch_name, &commit, false)
            .map_err(|e| e.to_string())?;
        self.switch_branch(branch_name)?;

        Ok(format!(
            "Sprouted branch '{}' from stash commit [{}]",
            branch_name,
            &stash_oid.to_string()[..7]
        ))
    }

    /// 6. The Overseer Portals: Remote Operations
    pub fn get_remote_details(&self) -> Vec<(String, String)> {
        let mut remotes = Vec::new();
        if let Ok(list) = self.repo.remotes() {
            for name in list.iter().flatten() {
                let url = self
                    .repo
                    .find_remote(name)
                    .ok()
                    .and_then(|r| r.url().map(|s| s.to_string()))
                    .unwrap_or_else(|| "local".to_string());
                remotes.push((name.to_string(), url));
            }
        }
        remotes
    }

    /// 7. Multi-Repo and Genesis Rituals
    pub fn repo_path(&self) -> String {
        self.repo
            .workdir()
            .or_else(|| self.repo.path().parent())
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string())
    }

    pub fn repo_name(&self) -> String {
        let p = self.repo_path();
        let path = std::path::Path::new(&p);
        path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "Itzamna Sanctuary".to_string())
    }

    pub fn clone_repo(url: &str, target_path: &str) -> Result<Self, String> {
        let repo = Repository::clone(url, target_path).map_err(|e| e.to_string())?;
        Ok(Self { repo })
    }

    pub fn init_repo(target_path: &str) -> Result<Self, String> {
        let repo = Repository::init(target_path).map_err(|e| e.to_string())?;
        Ok(Self { repo })
    }

    /// Retrieve the historical (old, new) text contents for a specific file within a given commit
    pub fn get_commit_file_diff(
        &self,
        short_or_full_sha: &str,
        file_path: &str,
    ) -> (String, String) {
        let commit = if short_or_full_sha.len() == 40 {
            self.repo
                .find_commit(Oid::from_str(short_or_full_sha).unwrap_or(Oid::zero()))
                .ok()
        } else {
            let mut found = None;
            if let Ok(mut revwalk) = self.repo.revwalk() {
                let _ = revwalk.push_head();
                for id in revwalk.flatten() {
                    if id.to_string().starts_with(short_or_full_sha) {
                        found = Some(id);
                        break;
                    }
                }
            }
            found.and_then(|id| self.repo.find_commit(id).ok())
        };

        let target_commit = match commit {
            Some(c) => c,
            None => return (String::new(), String::new()),
        };

        let target_tree = match target_commit.tree() {
            Ok(t) => t,
            Err(_) => return (String::new(), String::new()),
        };

        let new_text = target_tree
            .get_path(Path::new(file_path))
            .ok()
            .and_then(|entry| entry.to_object(&self.repo).ok())
            .and_then(|obj| obj.into_blob().ok())
            .and_then(|blob| {
                std::str::from_utf8(blob.content())
                    .ok()
                    .map(|s| s.to_string())
            })
            .unwrap_or_default();

        let old_text = if target_commit.parent_count() > 0 {
            target_commit
                .parent(0)
                .ok()
                .and_then(|p| p.tree().ok())
                .and_then(|ptree| ptree.get_path(Path::new(file_path)).ok())
                .and_then(|entry| entry.to_object(&self.repo).ok())
                .and_then(|obj| obj.into_blob().ok())
                .and_then(|blob| {
                    std::str::from_utf8(blob.content())
                        .ok()
                        .map(|s| s.to_string())
                })
                .unwrap_or_default()
        } else {
            String::new()
        };

        (old_text, new_text)
    }

    /// Retrieve (old, new) text contents for a working directory or staged file
    pub fn get_working_file_diff(&self, file_path: &str, is_staged: bool) -> (String, String) {
        let head_commit = self.repo.head().ok().and_then(|h| h.peel_to_commit().ok());
        let head_text = head_commit
            .and_then(|c| c.tree().ok())
            .and_then(|t| t.get_path(Path::new(file_path)).ok())
            .and_then(|entry| entry.to_object(&self.repo).ok())
            .and_then(|obj| obj.into_blob().ok())
            .and_then(|blob| {
                std::str::from_utf8(blob.content())
                    .ok()
                    .map(|s| s.to_string())
            })
            .unwrap_or_default();

        let index_text = self
            .repo
            .index()
            .ok()
            .and_then(|idx| idx.get_path(Path::new(file_path), 0).map(|e| e.id))
            .and_then(|oid| self.repo.find_blob(oid).ok())
            .and_then(|blob| {
                std::str::from_utf8(blob.content())
                    .ok()
                    .map(|s| s.to_string())
            });

        let workdir = self
            .repo
            .workdir()
            .map(|w| w.to_path_buf())
            .unwrap_or_else(|| std::path::PathBuf::from("."));
        let full_path = workdir.join(file_path);
        let worktree_text = std::fs::read_to_string(&full_path).unwrap_or_default();

        if is_staged {
            let current_index = index_text.unwrap_or_default();
            (head_text, current_index)
        } else {
            let base_text = index_text.unwrap_or(head_text);
            (base_text, worktree_text)
        }
    }

    // --- PHASE 3: K'UK'ULKAN REBASE & CONFLICT CRUCIBLE ---

    /// Load commits between HEAD and upstream tracking branch (or recent commits if no upstream)
    /// Returns Vec<(full_sha, commit_summary)> ordered chronologically (oldest at index 0).
    pub fn load_rebase_candidates(&self) -> Result<Vec<(String, String)>, String> {
        let head = self.repo.head().map_err(|e| e.to_string())?;
        let head_commit = head.peel_to_commit().map_err(|e| e.to_string())?;
        let head_oid = head_commit.id();

        let upstream_oid = if let Some(branch_name) = head.shorthand() {
            if let Ok(branch) = self.repo.find_branch(branch_name, BranchType::Local) {
                branch.upstream().ok().and_then(|u| u.get().target())
            } else {
                None
            }
        } else {
            None
        };

        let mut revwalk = self.repo.revwalk().map_err(|e| e.to_string())?;
        revwalk
            .set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::TIME)
            .map_err(|e| e.to_string())?;
        revwalk.push(head_oid).map_err(|e| e.to_string())?;

        if let Some(up_oid) = upstream_oid {
            let _ = revwalk.hide(up_oid);
        }

        let mut candidates = Vec::new();
        let max_count = if upstream_oid.is_some() { 100 } else { 20 };

        for oid in revwalk.take(max_count).flatten() {
            if let Ok(commit) = self.repo.find_commit(oid) {
                let summary = commit.summary().unwrap_or("Sacred Inscription").to_string();
                candidates.push((oid.to_string(), summary));
            }
        }

        // Chronological order: oldest at index 0 (top of rebase sequencer)
        candidates.reverse();
        Ok(candidates)
    }

    /// Retrieve the list of files currently in conflict from the Git index.
    pub fn get_conflicted_files(&self) -> Vec<String> {
        let mut files = Vec::new();
        if let Ok(index) = self.repo.index() {
            if let Ok(conflicts) = index.conflicts() {
                for c in conflicts.flatten() {
                    if let Some(our) = c.our {
                        files.push(String::from_utf8_lossy(&our.path).to_string());
                    } else if let Some(their) = c.their {
                        files.push(String::from_utf8_lossy(&their.path).to_string());
                    } else if let Some(ancestor) = c.ancestor {
                        files.push(String::from_utf8_lossy(&ancestor.path).to_string());
                    }
                }
            }
        }
        files.sort();
        files.dedup();
        files
    }

    /// Read the (ours, theirs) file contents for a conflicted file path.
    pub fn get_conflict_contents(&self, file_path: &str) -> (String, String) {
        let mut ours = String::new();
        let mut theirs = String::new();

        if let Ok(index) = self.repo.index() {
            if let Ok(conflicts) = index.conflicts() {
                for c in conflicts.flatten() {
                    let matches_our = c
                        .our
                        .as_ref()
                        .map(|e| String::from_utf8_lossy(&e.path) == file_path)
                        .unwrap_or(false);
                    let matches_their = c
                        .their
                        .as_ref()
                        .map(|e| String::from_utf8_lossy(&e.path) == file_path)
                        .unwrap_or(false);

                    if matches_our || matches_their {
                        if let Some(our_entry) = c.our {
                            if let Ok(blob) = self.repo.find_blob(our_entry.id) {
                                ours = String::from_utf8_lossy(blob.content()).to_string();
                            }
                        }
                        if let Some(their_entry) = c.their {
                            if let Ok(blob) = self.repo.find_blob(their_entry.id) {
                                theirs = String::from_utf8_lossy(blob.content()).to_string();
                            }
                        }
                        break;
                    }
                }
            }
        }

        // Fallback: if blobs were empty, read using git show :2 / :3
        if ours.is_empty() {
            if let Ok(out) = std::process::Command::new("git")
                .current_dir(self.repo.workdir().unwrap_or(Path::new(".")))
                .args(["show", &format!(":2:{}", file_path)])
                .output()
            {
                if out.status.success() {
                    ours = String::from_utf8_lossy(&out.stdout).to_string();
                }
            }
        }
        if theirs.is_empty() {
            if let Ok(out) = std::process::Command::new("git")
                .current_dir(self.repo.workdir().unwrap_or(Path::new(".")))
                .args(["show", &format!(":3:{}", file_path)])
                .output()
            {
                if out.status.success() {
                    theirs = String::from_utf8_lossy(&out.stdout).to_string();
                }
            }
        }

        (ours, theirs)
    }

    /// Resolve conflict by adopting OURS (local branch version) and staging the result.
    pub fn resolve_conflict_ours(&self, file_path: &str) -> Result<(), String> {
        let output = std::process::Command::new("git")
            .current_dir(self.repo.workdir().unwrap_or(Path::new(".")))
            .args(["checkout", "--ours", "--", file_path])
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        self.stage_path(file_path)
    }

    /// Resolve conflict by adopting THEIRS (incoming rebase version) and staging the result.
    pub fn resolve_conflict_theirs(&self, file_path: &str) -> Result<(), String> {
        let output = std::process::Command::new("git")
            .current_dir(self.repo.workdir().unwrap_or(Path::new(".")))
            .args(["checkout", "--theirs", "--", file_path])
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        self.stage_path(file_path)
    }

    /// Resolve conflict by concatenating both versions (Ours then Theirs) and staging the result.
    pub fn resolve_conflict_both(&self, file_path: &str) -> Result<(), String> {
        let (ours, theirs) = self.get_conflict_contents(file_path);
        let combined = format!("{}\n{}", ours.trim_end(), theirs);

        let workdir = self.repo.workdir().unwrap_or(Path::new("."));
        let full_path = workdir.join(file_path);
        std::fs::write(&full_path, combined).map_err(|e| e.to_string())?;

        self.stage_path(file_path)
    }

    /// Abort in-progress rebase and restore the previous state from ORIG_HEAD.
    pub fn abort_rebase(&self) -> Result<(), String> {
        // Try resetting to ORIG_HEAD
        if let Ok(orig_obj) = self.repo.revparse_single("ORIG_HEAD") {
            self.repo
                .reset(&orig_obj, ResetType::Hard, None)
                .map_err(|e| e.to_string())?;
            if let Ok(mut r) = self.repo.find_reference("ORIG_HEAD") {
                let _ = r.delete();
            }
            return Ok(());
        }

        // Fallback to git CLI
        let _ = std::process::Command::new("git")
            .current_dir(self.repo.workdir().unwrap_or(Path::new(".")))
            .args(["rebase", "--abort"])
            .output();

        let head = self.repo.head().map_err(|e| e.to_string())?;
        let commit = head.peel_to_commit().map_err(|e| e.to_string())?;
        self.repo
            .reset(commit.as_object(), ResetType::Hard, None)
            .map_err(|e| e.to_string())
    }

    /// Execute interactive rebase plan across given steps using cherry-pick sequencer.
    pub fn execute_rebase_plan(&self, steps: &[RebasePlanStep]) -> Result<String, RebaseError> {
        let (unstaged, staged) = self.get_status_files();
        if !unstaged.is_empty() || !staged.is_empty() {
            return Err(RebaseError::Fatal(
                "Cannot execute rebase: working tree has uncommitted modifications. Please stash or commit changes first.".into()
            ));
        }

        if steps.is_empty() {
            return Ok("No rebase operations specified.".into());
        }

        // Validate: first active step cannot be Squash
        if let Some(first_active) = steps.iter().find(|s| s.action != RebaseAction::Drop) {
            if first_active.action == RebaseAction::Squash {
                return Err(RebaseError::Fatal(
                    "Cannot squash the initial commit in rebase range — squash requires an earlier commit to fold into.".into()
                ));
            }
        }

        // Validate: cannot drop all commits
        if steps.iter().all(|s| s.action == RebaseAction::Drop) {
            return Err(RebaseError::Fatal(
                "Cannot drop all commits in rebase range.".into(),
            ));
        }

        // Save ORIG_HEAD
        let head = self
            .repo
            .head()
            .map_err(|e| RebaseError::Fatal(e.to_string()))?;
        let head_commit = head
            .peel_to_commit()
            .map_err(|e| RebaseError::Fatal(e.to_string()))?;
        let _ = self
            .repo
            .reference("ORIG_HEAD", head_commit.id(), true, "itzamna: rebase begin");

        // Base commit is the parent of the first step
        let first_oid =
            Oid::from_str(&steps[0].full_sha).map_err(|e| RebaseError::Fatal(e.to_string()))?;
        let first_commit = self
            .repo
            .find_commit(first_oid)
            .map_err(|e| RebaseError::Fatal(e.to_string()))?;

        if first_commit.parent_count() == 0 {
            return Err(RebaseError::Fatal(
                "Rebasing root commit is not currently supported.".into(),
            ));
        }

        let base_commit = first_commit
            .parent(0)
            .map_err(|e| RebaseError::Fatal(e.to_string()))?;

        // Reset HEAD to base
        self.repo
            .reset(base_commit.as_object(), ResetType::Hard, None)
            .map_err(|e| {
                RebaseError::Fatal(format!("Failed to reset HEAD to base commit: {}", e))
            })?;

        // Sequentially apply each step
        for (idx, step) in steps.iter().enumerate() {
            self.apply_single_rebase_step(idx, step)?;
        }

        let final_head = self.repo.head().ok().and_then(|h| h.peel_to_commit().ok());
        let final_sha = final_head
            .map(|c| c.id().to_string()[..7].to_string())
            .unwrap_or_default();
        Ok(format!(
            "K'uk'ulkan Rebase ritual concluded! Sovereign HEAD is [{}]",
            final_sha
        ))
    }

    /// Internal helper: Apply a single rebase step
    fn apply_single_rebase_step(
        &self,
        idx: usize,
        step: &RebasePlanStep,
    ) -> Result<(), RebaseError> {
        if step.action == RebaseAction::Drop {
            return Ok(());
        }

        let target_oid =
            Oid::from_str(&step.full_sha).map_err(|e| RebaseError::Fatal(e.to_string()))?;
        let target_commit = self
            .repo
            .find_commit(target_oid)
            .map_err(|e| RebaseError::Fatal(e.to_string()))?;

        let head = self
            .repo
            .head()
            .map_err(|e| RebaseError::Fatal(e.to_string()))?;
        let head_commit = head
            .peel_to_commit()
            .map_err(|e| RebaseError::Fatal(e.to_string()))?;

        let mut index = self
            .repo
            .cherrypick_commit(&target_commit, &head_commit, 0, None)
            .map_err(|e| RebaseError::Fatal(e.to_string()))?;

        if index.has_conflicts() {
            let mut checkout_opts = git2::build::CheckoutBuilder::new();
            checkout_opts.conflict_style_merge(true);
            let _ = self
                .repo
                .checkout_index(Some(&mut index), Some(&mut checkout_opts));
            let _ = index.write();

            let conflicted_files = self.get_conflicted_files();
            return Err(RebaseError::Conflict {
                conflicted_files,
                step_index: idx,
                message: format!(
                    "Conflict encountered while applying [{}] '{}'",
                    &step.full_sha[..7.min(step.full_sha.len())],
                    step.current_message
                ),
            });
        }

        let tree_oid = index
            .write_tree_to(&self.repo)
            .map_err(|e| RebaseError::Fatal(e.to_string()))?;
        let tree = self
            .repo
            .find_tree(tree_oid)
            .map_err(|e| RebaseError::Fatal(e.to_string()))?;
        let sig = self.sanctuary_signature().map_err(RebaseError::Fatal)?;

        match step.action {
            RebaseAction::Pick => {
                let msg = if step.current_message.trim().is_empty() {
                    target_commit.message().unwrap_or("Sacred Inscription")
                } else {
                    &step.current_message
                };
                self.repo
                    .commit(Some("HEAD"), &sig, &sig, msg, &tree, &[&head_commit])
                    .map_err(|e| RebaseError::Fatal(e.to_string()))?;
            }
            RebaseAction::Reword => {
                let msg = &step.current_message;
                self.repo
                    .commit(Some("HEAD"), &sig, &sig, msg, &tree, &[&head_commit])
                    .map_err(|e| RebaseError::Fatal(e.to_string()))?;
            }
            RebaseAction::Squash => {
                let parent_msg = head_commit.message().unwrap_or("").trim();
                let combined_msg = format!("{}\n\n- {}", parent_msg, step.current_message.trim());
                head_commit
                    .amend(
                        Some("HEAD"),
                        Some(&sig),
                        Some(&sig),
                        None,
                        Some(&combined_msg),
                        Some(&tree),
                    )
                    .map_err(|e| RebaseError::Fatal(e.to_string()))?;
            }
            RebaseAction::Drop => {}
        }

        let mut checkout = git2::build::CheckoutBuilder::new();
        checkout.force();
        let _ = self.repo.checkout_head(Some(&mut checkout));

        Ok(())
    }

    /// Continue rebase execution after resolving all conflicts in the active step.
    pub fn continue_rebase_plan(
        &self,
        current_step_index: usize,
        steps: &[RebasePlanStep],
    ) -> Result<String, RebaseError> {
        let conflicts = self.get_conflicted_files();
        if !conflicts.is_empty() {
            return Err(RebaseError::Conflict {
                conflicted_files: conflicts,
                step_index: current_step_index,
                message: "Conflicts remain in index. Please resolve all conflicts and stage files before continuing.".into(),
            });
        }

        // Commit the resolved conflict state for the current step
        if let Some(step) = steps.get(current_step_index) {
            let mut index = self
                .repo
                .index()
                .map_err(|e| RebaseError::Fatal(e.to_string()))?;
            let tree_oid = index
                .write_tree_to(&self.repo)
                .map_err(|e| RebaseError::Fatal(e.to_string()))?;
            let tree = self
                .repo
                .find_tree(tree_oid)
                .map_err(|e| RebaseError::Fatal(e.to_string()))?;
            let sig = self.sanctuary_signature().map_err(RebaseError::Fatal)?;
            let head = self
                .repo
                .head()
                .map_err(|e| RebaseError::Fatal(e.to_string()))?;
            let head_commit = head
                .peel_to_commit()
                .map_err(|e| RebaseError::Fatal(e.to_string()))?;

            if step.action == RebaseAction::Squash {
                let parent_msg = head_commit.message().unwrap_or("").trim();
                let combined_msg = format!("{}\n\n- {}", parent_msg, step.current_message.trim());
                head_commit
                    .amend(
                        Some("HEAD"),
                        Some(&sig),
                        Some(&sig),
                        None,
                        Some(&combined_msg),
                        Some(&tree),
                    )
                    .map_err(|e| RebaseError::Fatal(e.to_string()))?;
            } else {
                let msg = if step.current_message.trim().is_empty() {
                    "Resolved rebase step"
                } else {
                    step.current_message.trim()
                };
                self.repo
                    .commit(Some("HEAD"), &sig, &sig, msg, &tree, &[&head_commit])
                    .map_err(|e| RebaseError::Fatal(e.to_string()))?;
            }

            let mut checkout = git2::build::CheckoutBuilder::new();
            checkout.force();
            let _ = self.repo.checkout_head(Some(&mut checkout));
        }

        // Continue remaining steps
        for (idx, step) in steps.iter().enumerate().skip(current_step_index + 1) {
            self.apply_single_rebase_step(idx, step)?;
        }

        let final_head = self.repo.head().ok().and_then(|h| h.peel_to_commit().ok());
        let final_sha = final_head
            .map(|c| c.id().to_string()[..7].to_string())
            .unwrap_or_default();
        Ok(format!(
            "K'uk'ulkan Rebase ritual concluded! Sovereign HEAD is [{}]",
            final_sha
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph_node(sha_char: char, parents: Vec<String>) -> GitCommitNode {
        let sha = std::iter::repeat_n(sha_char, 40).collect::<String>();
        GitCommitNode {
            short_sha: sha[..7].to_string(),
            sha,
            summary: String::new(),
            author_name: String::new(),
            author_email: String::new(),
            time_secs: 0,
            relative_time: String::new(),
            parents,
            branch_lane: 0,
            is_head: false,
            tags: Vec::new(),
            branches: Vec::new(),
        }
    }

    #[test]
    fn lane_packing_routes_merge_parents_and_joins_shared_ancestors() {
        let sha = |c: char| std::iter::repeat_n(c, 40).collect::<String>();
        let mut nodes = vec![
            graph_node('a', vec![sha('b'), sha('c')]),
            graph_node('b', vec![sha('d')]),
            graph_node('c', vec![sha('d')]),
            graph_node('d', Vec::new()),
        ];
        GitSanctuary::assign_topological_lanes(&mut nodes);
        assert_eq!(nodes[0].branch_lane, 0);
        assert_eq!(nodes[1].branch_lane, 0);
        assert_eq!(nodes[2].branch_lane, 1);
        assert_eq!(nodes[3].branch_lane, 0);
    }

    #[test]
    fn local_telemetry_has_no_synthetic_contributor_data() {
        let sanctuary = GitSanctuary::open(".").expect("workspace repository should open");
        let report = sanctuary
            .calculate_local_telemetry()
            .expect("local Git analysis should work");
        assert!(report
            .top_contributors
            .iter()
            .all(|author| author.name != "Ahau Architect"));
        assert!(report.dora.change_failure_rate.is_finite());
        assert!(
            report.churn.net_growth
                == report.churn.lines_added as i64 - report.churn.lines_deleted as i64
        );
    }

    #[test]
    fn test_sanctuary_signature_fallback() {
        if let Ok(sanctuary) = GitSanctuary::open(".") {
            let sig = sanctuary.sanctuary_signature();
            assert!(sig.is_ok(), "Signature generation should always succeed");
            let sig = sig.unwrap();
            assert!(
                !sig.name().unwrap_or("").is_empty(),
                "Signature name must not be empty"
            );
            assert!(
                !sig.email().unwrap_or("").is_empty(),
                "Signature email must not be empty"
            );
        }
    }

    #[test]
    fn test_reflog_full_sha_length() {
        if let Ok(sanctuary) = GitSanctuary::open(".") {
            if let Ok(entries) = sanctuary.get_reflog() {
                for entry in entries {
                    assert_eq!(
                        entry.full_sha.len(),
                        40,
                        "Reflog entry full_sha must be exactly 40 characters"
                    );
                    assert!(
                        entry.old_sha.len() <= 7,
                        "Old SHA should be short representation"
                    );
                    assert!(
                        entry.new_sha.len() <= 7,
                        "New SHA should be short representation"
                    );
                }
            }
        }
    }

    #[test]
    fn test_working_diff_counts() {
        if let Ok(sanctuary) = GitSanctuary::open(".") {
            let counts = sanctuary.get_working_diff_counts(false);
            for (path, (_add, _del)) in counts {
                assert!(!path.is_empty());
            }
        }
    }

    #[test]
    fn test_load_rebase_candidates() {
        if let Ok(sanctuary) = GitSanctuary::open(".") {
            let candidates = sanctuary.load_rebase_candidates();
            assert!(
                candidates.is_ok(),
                "Loading rebase candidates should succeed"
            );
            let candidates = candidates.unwrap();
            for (sha, summary) in &candidates {
                assert_eq!(sha.len(), 40, "Candidate SHA should be 40 chars full hash");
                assert!(
                    !summary.is_empty(),
                    "Candidate commit summary should not be empty"
                );
            }
        }
    }

    #[test]
    fn test_rebase_action_variants() {
        let step = RebasePlanStep {
            sha: "abcdef1".into(),
            full_sha: "abcdef1234567890123456789012345678901234".into(),
            original_message: "original".into(),
            current_message: "updated".into(),
            action: RebaseAction::Pick,
        };
        assert_eq!(step.action, RebaseAction::Pick);
        assert_eq!(step.sha.len(), 7);
        assert_eq!(step.full_sha.len(), 40);
    }
}
