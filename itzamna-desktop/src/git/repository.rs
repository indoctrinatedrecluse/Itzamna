//! Git Sanctuary — Real repository operations powered by git2-rs

use git2::{
    BlameOptions, BranchType, Commit, IndexAddOption, Oid, Repository, ResetType, Signature,
    StatusOptions,
};
use std::path::Path;

use crate::git::detailed_commit::DetailedCommit;

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
    pub committer_name: String,
    pub message: String,
    pub action_type: &'static str,
}

#[derive(Debug, Clone)]
pub struct StashEntryDetail {
    pub index: usize,
    pub message: String,
    pub commit_sha: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeStrategy {
    FastForward,
    NoFastForward,
    Squash,
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

    pub fn get_head_info(&self) -> (String, String) {
        let branch = match self.repo.head() {
            Ok(head) => head
                .shorthand()
                .unwrap_or("detached HEAD")
                .to_string(),
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
                    staged.push(FileStatus { path: path.clone(), status_code: "A", is_staged: true });
                } else if status.is_index_modified() {
                    staged.push(FileStatus { path: path.clone(), status_code: "M", is_staged: true });
                } else if status.is_index_deleted() {
                    staged.push(FileStatus { path: path.clone(), status_code: "D", is_staged: true });
                }

                // Unstaged checks
                if status.is_wt_new() {
                    unstaged.push(FileStatus { path, status_code: "?", is_staged: false });
                } else if status.is_wt_modified() {
                    unstaged.push(FileStatus { path, status_code: "M", is_staged: false });
                } else if status.is_wt_deleted() {
                    unstaged.push(FileStatus { path, status_code: "D", is_staged: false });
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

        let _ = revwalk.push_head();
        let _ = revwalk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::TIME);

        let head_oid = self.repo.head().ok().and_then(|h| h.target());

        for (idx, oid_res) in revwalk.take(max_commits).enumerate() {
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
                .map(|p| p.to_string()[..7].to_string())
                .collect::<Vec<_>>();

            let is_head = head_oid.map(|h| h == oid).unwrap_or(false);

            let branch_lane = if idx == 0 || parents.len() <= 1 {
                0
            } else {
                1
            };

            let now = chrono::Utc::now().timestamp();
            let diff_mins = ((now - time_secs) / 60).max(1);
            let relative_time = if diff_mins < 60 {
                format!("{} min ago", diff_mins)
            } else if diff_mins < 1440 {
                format!("{} hours ago", diff_mins / 60)
            } else {
                format!("{} days ago", diff_mins / 1440)
            };

            nodes.push(GitCommitNode {
                sha,
                short_sha,
                summary,
                author_name,
                author_email,
                time_secs,
                relative_time,
                parents,
                branch_lane,
                is_head,
                tags: Vec::new(),
            });
        }

        nodes
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
        index
            .add_path(Path::new(path))
            .map_err(|e| e.to_string())?;
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

    // --- REAL COMMIT CREATION ---

    pub fn create_commit(&self, summary: &str, description: &str) -> Result<String, String> {
        let mut index = self.repo.index().map_err(|e| e.to_string())?;
        let tree_oid = index.write_tree().map_err(|e| e.to_string())?;
        let tree = self.repo.find_tree(tree_oid).map_err(|e| e.to_string())?;

        let sig = self
            .repo
            .signature()
            .unwrap_or_else(|_| Signature::now("indoctrinatedrecluse", "recluse@itzamna.dev").unwrap());

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
            .commit(
                Some("HEAD"),
                &sig,
                &sig,
                &full_msg,
                &tree,
                &parents,
            )
            .map_err(|e| e.to_string())?;

        Ok(commit_oid.to_string())
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

        self.repo
            .set_head(&ref_name)
            .map_err(|e| e.to_string())?;

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

    pub fn create_tag(&self, tag_name: &str) -> Result<(), String> {
        let head = self.repo.head().map_err(|e| e.to_string())?;
        let obj = head.peel_to_commit().map_err(|e| e.to_string())?.into_object();
        let sig = self
            .repo
            .signature()
            .unwrap_or_else(|_| Signature::now("indoctrinatedrecluse", "recluse@itzamna.dev").unwrap());

        self.repo
            .tag(tag_name, &obj, &sig, tag_name, false)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn create_stash(&mut self, message: &str) -> Result<(), String> {
        let sig = self
            .repo
            .signature()
            .unwrap_or_else(|_| Signature::now("indoctrinatedrecluse", "recluse@itzamna.dev").unwrap());

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

            entries.push(ReflogEntryInfo {
                index: idx,
                old_sha: entry.id_old().to_string()[..7].to_string(),
                new_sha: entry.id_new().to_string()[..7].to_string(),
                committer_name: entry.committer().name().unwrap_or("Architect").to_string(),
                message: msg,
                action_type,
            });
        }

        Ok(entries)
    }

    /// Reset HEAD to a given commit or reflog target
    pub fn reset_head(&self, target_sha: &str, reset_type: ResetType) -> Result<(), String> {
        let obj = self.repo.revparse_single(target_sha).map_err(|e| e.to_string())?;
        self.repo.reset(&obj, reset_type, None).map_err(|e| e.to_string())?;
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
    pub fn merge_branch(&self, target_branch: &str, strategy: MergeStrategy) -> Result<String, String> {
        let ref_name = format!("refs/heads/{}", target_branch);
        let target_commit = self
            .repo
            .revparse_single(&ref_name)
            .map_err(|e| format!("Branch '{}' not found: {}", target_branch, e))?
            .peel_to_commit()
            .map_err(|e| e.to_string())?;

        let head = self.repo.head().map_err(|e| e.to_string())?;
        let head_commit = head.peel_to_commit().map_err(|e| e.to_string())?;

        let _head_annotated = self.repo.find_annotated_commit(head_commit.id()).map_err(|e| e.to_string())?;
        let target_annotated = self.repo.find_annotated_commit(target_commit.id()).map_err(|e| e.to_string())?;

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
            self.repo.checkout_tree(target_commit.as_object(), Some(&mut checkout)).map_err(|e| e.to_string())?;
            let mut head_ref = self.repo.head().map_err(|e| e.to_string())?;
            head_ref.set_target(target_commit.id(), &format!("Fast-Forward to {}", target_branch)).map_err(|e| e.to_string())?;
            return Ok(format!("Fast-Forwarded sovereign branch to [{}]", &target_commit.id().to_string()[..7]));
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
        let sig = self
            .repo
            .signature()
            .unwrap_or_else(|_| Signature::now("indoctrinatedrecluse", "recluse@itzamna.dev").unwrap());

        let parents: Vec<&Commit> = match strategy {
            MergeStrategy::Squash => vec![&head_commit],
            _ => vec![&head_commit, &target_commit],
        };

        let msg = format!("Merge branch '{}' into active HEAD", target_branch);
        let commit_oid = self
            .repo
            .commit(Some("HEAD"), &sig, &sig, &msg, &tree, &parents)
            .map_err(|e| e.to_string())?;

        Ok(format!("Consecrated merge commit [{}]", &commit_oid.to_string()[..7]))
    }

    /// Cherry-pick a specific commit into active HEAD
    pub fn cherry_pick_commit(&self, commit_sha: &str) -> Result<String, String> {
        let commit_oid = Oid::from_str(commit_sha).map_err(|e| e.to_string())?;
        let target_commit = self.repo.find_commit(commit_oid).map_err(|e| e.to_string())?;

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
        let sig = self
            .repo
            .signature()
            .unwrap_or_else(|_| Signature::now("indoctrinatedrecluse", "recluse@itzamna.dev").unwrap());

        let msg = format!("cherry-pick: {}\n\n(cherry picked from commit {})", target_commit.summary().unwrap_or(""), commit_sha);
        let new_oid = self
            .repo
            .commit(Some("HEAD"), &sig, &sig, &msg, &tree, &[&head_commit])
            .map_err(|e| e.to_string())?;

        Ok(format!("Plucked commit into [{}]", &new_oid.to_string()[..7]))
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
                commit_sha: oid.to_string()[..7].to_string(),
            });
            true
        });
        details
    }

    pub fn drop_stash(&mut self, index: usize) -> Result<(), String> {
        self.repo.stash_drop(index).map_err(|e| e.to_string())?;
        Ok(())
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
}
