//! The Stone Diff Engine — Powered by `similar` and `git2-rs`

use similar::{ChangeTag, TextDiff};

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub tag: DiffLineTag,
    pub old_lineno: Option<usize>,
    pub new_lineno: Option<usize>,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffLineTag {
    Context,
    Addition,
    Deletion,
}

#[derive(Debug, Clone)]
pub struct DiffHunk {
    pub header: String,
    pub old_start: usize,
    pub old_lines: usize,
    pub new_start: usize,
    pub new_lines: usize,
    pub lines: Vec<DiffLine>,
    pub is_staged: bool,
}

impl DiffHunk {
    /// Formats this hunk into an exact unified diff patch accepted by `git apply`
    pub fn to_patch(&self, file_path: &str) -> String {
        let mut patch = format!("--- a/{}\n+++ b/{}\n{}\n", file_path, file_path, self.header);
        for line in &self.lines {
            let prefix = match line.tag {
                DiffLineTag::Context => " ",
                DiffLineTag::Addition => "+",
                DiffLineTag::Deletion => "-",
            };
            patch.push_str(prefix);
            patch.push_str(&line.content);
            patch.push('\n');
        }
        patch
    }
}

#[derive(Debug, Clone)]
pub struct FileDiff {
    pub old_path: String,
    pub new_path: String,
    pub hunks: Vec<DiffHunk>,
}

pub struct DiffEngine;

impl DiffEngine {
    pub fn compute_diff(old_text: &str, new_text: &str, file_name: &str) -> FileDiff {
        let text_diff = TextDiff::from_lines(old_text, new_text);
        let mut hunks = Vec::new();

        for (_idx, group) in text_diff.grouped_ops(3).into_iter().enumerate() {
            let mut lines = Vec::new();

            let old_start = group.first().map(|op| op.old_range().start + 1).unwrap_or(1);
            let old_end = group.last().map(|op| op.old_range().end).unwrap_or(old_start);
            let old_lines = old_end.saturating_sub(old_start.saturating_sub(1));

            let new_start = group.first().map(|op| op.new_range().start + 1).unwrap_or(1);
            let new_end = group.last().map(|op| op.new_range().end).unwrap_or(new_start);
            let new_lines = new_end.saturating_sub(new_start.saturating_sub(1));

            let mut old_lineno = old_start;
            let mut new_lineno = new_start;

            for op in &group {
                for change in text_diff.iter_changes(op) {
                    let tag = match change.tag() {
                        ChangeTag::Equal => DiffLineTag::Context,
                        ChangeTag::Insert => DiffLineTag::Addition,
                        ChangeTag::Delete => DiffLineTag::Deletion,
                    };

                    let old_num = if tag != DiffLineTag::Addition {
                        let n = old_lineno;
                        old_lineno += 1;
                        Some(n)
                    } else {
                        None
                    };

                    let new_num = if tag != DiffLineTag::Deletion {
                        let n = new_lineno;
                        new_lineno += 1;
                        Some(n)
                    } else {
                        None
                    };

                    lines.push(DiffLine {
                        tag,
                        old_lineno: old_num,
                        new_lineno: new_num,
                        content: change.value().trim_end_matches(['\r', '\n']).to_string(),
                    });
                }
            }

            hunks.push(DiffHunk {
                header: format!("@@ -{},{} +{},{} @@", old_start, old_lines, new_start, new_lines),
                old_start,
                old_lines,
                new_start,
                new_lines,
                lines,
                is_staged: false,
            });
        }

        FileDiff {
            old_path: file_name.to_string(),
            new_path: file_name.to_string(),
            hunks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_diff_additions_and_deletions() {
        let old = "fn main() {\n    println!(\"Hello\");\n}";
        let new = "fn main() {\n    println!(\"Awakened Itzamna\");\n    let x = 42;\n}";
        let diff = DiffEngine::compute_diff(old, new, "src/main.rs");

        assert_eq!(diff.old_path, "src/main.rs");
        assert_eq!(diff.new_path, "src/main.rs");
        assert!(!diff.hunks.is_empty(), "Hunks should be generated for differences");

        let has_addition = diff.hunks.iter().any(|h| h.lines.iter().any(|l| l.tag == DiffLineTag::Addition));
        let has_deletion = diff.hunks.iter().any(|h| h.lines.iter().any(|l| l.tag == DiffLineTag::Deletion));
        assert!(has_addition, "Should contain addition line");
        assert!(has_deletion, "Should contain deletion line");
    }

    #[test]
    fn test_compute_diff_empty_to_new() {
        let old = "";
        let new = "const SACRED_KIN: u32 = 260;\n";
        let diff = DiffEngine::compute_diff(old, new, "src/calendar.rs");

        assert!(!diff.hunks.is_empty());
        let all_additions = diff.hunks[0].lines.iter().all(|l| l.tag == DiffLineTag::Addition);
        assert!(all_additions, "All lines should be additions when starting from empty");
    }

    #[test]
    fn test_hunk_to_patch_format() {
        let old = "line1\nline2\n";
        let new = "line1\nline2_modified\n";
        let diff = DiffEngine::compute_diff(old, new, "test.txt");
        assert!(!diff.hunks.is_empty());
        let patch = diff.hunks[0].to_patch("test.txt");
        assert!(patch.starts_with("--- a/test.txt\n+++ b/test.txt\n@@"));
        assert!(patch.contains("-line2"));
        assert!(patch.contains("+line2_modified"));
    }
}
