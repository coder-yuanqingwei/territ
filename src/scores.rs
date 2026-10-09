use std::fs;
use std::path::Path;

const MAX_ENTRIES: usize = 3;

#[derive(Debug, Clone, PartialEq)]
pub struct HighScore {
    pub name: String,
    pub score: u32,
}

#[derive(Debug)]
pub struct HighScores {
    entries: Vec<HighScore>,
}

impl HighScores {
    pub fn new() -> Self {
        HighScores {
            entries: Vec::new(),
        }
    }

    pub fn load(path: &Path) -> Self {
        let mut scores = HighScores::new();
        let Ok(content) = fs::read_to_string(path) else {
            return scores;
        };
        for line in content.lines() {
            let mut parts = line.split_whitespace();
            let (Some(name), Some(score_str), None) = (parts.next(), parts.next(), parts.next())
            else {
                continue;
            };
            let Ok(score) = score_str.parse::<u32>() else {
                continue;
            };
            scores.entries.push(HighScore {
                name: name.to_owned(),
                score,
            });
        }
        scores.entries.sort_by_key(|e| std::cmp::Reverse(e.score));
        scores.entries.truncate(MAX_ENTRIES);
        scores
    }

    pub fn save(&self, path: &Path) {
        let content = self
            .entries
            .iter()
            .map(|e| format!("{} {}", e.name, e.score))
            .collect::<Vec<_>>()
            .join("\n");
        if !content.is_empty() {
            let _ = fs::write(path, content + "\n");
        } else {
            let _ = fs::write(path, "");
        }
    }

    pub fn qualifies(&self, score: u32) -> bool {
        if score == 0 {
            return false;
        }
        if self.entries.len() < MAX_ENTRIES {
            return true;
        }
        score > self.entries.last().map(|e| e.score).unwrap_or(0)
    }

    pub fn insert(&mut self, name: String, score: u32) {
        let name = if name.is_empty() {
            "PLAYER".to_owned()
        } else {
            name
        };
        self.entries.push(HighScore { name, score });
        self.entries.sort_by_key(|e| std::cmp::Reverse(e.score));
        self.entries.truncate(MAX_ENTRIES);
    }

    pub fn entries(&self) -> &[HighScore] {
        &self.entries
    }
}

pub fn scores_path() -> std::path::PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        return Path::new(&home).join(".territ_highscores");
    }
    std::path::PathBuf::from(".territ_highscores")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_board_qualifies_any_positive_score() {
        let scores = HighScores::new();
        assert!(scores.qualifies(1));
        assert!(!scores.qualifies(0));
    }

    #[test]
    fn full_board_requires_beating_lowest() {
        let mut scores = HighScores::new();
        scores.insert("AAA".to_owned(), 100);
        scores.insert("BBB".to_owned(), 200);
        scores.insert("CCC".to_owned(), 300);
        assert!(!scores.qualifies(100));
        assert!(scores.qualifies(101));
    }

    #[test]
    fn insert_sorts_and_truncates() {
        let mut scores = HighScores::new();
        scores.insert("LOW".to_owned(), 50);
        scores.insert("HIGH".to_owned(), 500);
        scores.insert("MID".to_owned(), 200);
        scores.insert("NEW".to_owned(), 300);
        let names: Vec<&str> = scores.entries().iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["HIGH", "NEW", "MID"]);
    }

    #[test]
    fn empty_name_defaults_to_player() {
        let mut scores = HighScores::new();
        scores.insert(String::new(), 100);
        assert_eq!(scores.entries()[0].name, "PLAYER");
    }

    #[test]
    fn load_skips_corrupt_lines_and_sorts() {
        let dir = std::env::temp_dir().join("territ_scores_test");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("scores.txt");
        fs::write(
            &path,
            "AAA 100\nbadline\nBBB notanumber\nCCC 300\nDDD 200\n",
        )
        .unwrap();
        let scores = HighScores::load(&path);
        let names: Vec<&str> = scores.entries().iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["CCC", "DDD", "AAA"]);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn load_missing_file_returns_empty() {
        let scores = HighScores::load(Path::new("/nonexistent/path/scores.txt"));
        assert!(scores.entries().is_empty());
    }

    #[test]
    fn save_then_load_roundtrip() {
        let dir = std::env::temp_dir().join("territ_scores_roundtrip");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("scores.txt");
        let mut scores = HighScores::new();
        scores.insert("AAA".to_owned(), 100);
        scores.insert("BBB".to_owned(), 300);
        scores.save(&path);
        let loaded = HighScores::load(&path);
        assert_eq!(loaded.entries(), scores.entries());
        fs::remove_dir_all(&dir).unwrap();
    }
}
