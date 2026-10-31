//! Git Sanctuary — Real repository reader powered by git2-rs

use git2::{BranchType, Repository, StatusOptions};

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

            // Simple topological lane routing
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
}
