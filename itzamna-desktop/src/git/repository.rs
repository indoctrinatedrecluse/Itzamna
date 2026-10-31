//! Git Sanctuary — Real repository operations powered by git2-rs

use git2::{
    BranchType, Commit, IndexAddOption, Oid, Repository, ResetType, Signature, StatusOptions,
};
use std::path::Path;

use crate::git::detailed_commit::DetailedCommit;

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
}
