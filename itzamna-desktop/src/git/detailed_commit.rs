//! Detailed Commit Inspection — Full metadata, tree files, and parent diffs

use git2::{Commit, Delta, DiffOptions, Repository};

#[derive(Debug, Clone)]
pub struct CommitFileChange {
    pub path: String,
    pub status: &'static str,
    pub additions: usize,
    pub deletions: usize,
}

#[derive(Debug, Clone)]
pub struct DetailedCommit {
    pub sha: String,
    pub short_sha: String,
    pub tree_sha: String,
    pub parent_shas: Vec<String>,
    pub author_name: String,
    pub author_email: String,
    pub author_time: String,
    pub committer_name: String,
    pub committer_email: String,
    pub summary: String,
    pub full_message: String,
    pub changed_files: Vec<CommitFileChange>,
    pub total_additions: usize,
    pub total_deletions: usize,
}

impl DetailedCommit {
    pub fn from_git2_commit(repo: &Repository, commit: &Commit) -> Self {
        let sha = commit.id().to_string();
        let short_sha = sha[..sha.len().min(7)].to_string();
        let tree_sha = commit.tree_id().to_string()[..7].to_string();

        let parent_shas = commit
            .parent_ids()
            .map(|id| id.to_string()[..7].to_string())
            .collect();

        let author = commit.author();
        let author_name = author.name().unwrap_or("Ahau").to_string();
        let author_email = author.email().unwrap_or("unknown@itzamna.dev").to_string();

        let time_secs = commit.time().seconds();
        let author_time = chrono::DateTime::from_timestamp(time_secs, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
            .unwrap_or_else(|| "Ancient Era".to_string());

        let committer = commit.committer();
        let committer_name = committer.name().unwrap_or("Ahau").to_string();
        let committer_email = committer.email().unwrap_or("unknown@itzamna.dev").to_string();

        let summary = commit.summary().unwrap_or("Sacred Inscription").to_string();
        let full_message = commit.message().unwrap_or(&summary).to_string();

        // Calculate changed files by diffing against first parent (or empty tree if root)
        let mut changed_files = Vec::new();
        let mut total_additions = 0;
        let mut total_deletions = 0;

        let parent_tree = commit.parent(0).ok().and_then(|p| p.tree().ok());
        let current_tree = commit.tree().ok();

        if let Some(curr) = current_tree.as_ref() {
            let mut opts = DiffOptions::new();
            if let Ok(diff) = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(curr), Some(&mut opts)) {
                if let Ok(stats) = diff.stats() {
                    total_additions = stats.insertions();
                    total_deletions = stats.deletions();
                }

                for delta in diff.deltas() {
                    let path = delta
                        .new_file()
                        .path()
                        .or_else(|| delta.old_file().path())
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|| "unknown".to_string());

                    let status = match delta.status() {
                        Delta::Added => "A",
                        Delta::Deleted => "D",
                        Delta::Modified => "M",
                        Delta::Renamed => "R",
                        Delta::Copied => "C",
                        _ => "M",
                    };

                    changed_files.push(CommitFileChange {
                        path,
                        status,
                        additions: 0,
                        deletions: 0,
                    });
                }
            }
        }

        Self {
            sha,
            short_sha,
            tree_sha,
            parent_shas,
            author_name,
            author_email,
            author_time,
            committer_name,
            committer_email,
            summary,
            full_message,
            changed_files,
            total_additions,
            total_deletions,
        }
    }
}
