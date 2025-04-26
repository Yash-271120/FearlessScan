use std::{cmp::Ordering};
use walkdir::DirEntry;

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    name: String,
    path: String,
    pub score: i64,
    indices: Vec<usize>,
}

impl SearchResult {
    pub fn try_from(dir_entry: &DirEntry, matcher: &SkimMatcherV2, query: &str) -> Option<Self> {
        let file_name = dir_entry.file_name().to_string_lossy().to_string();
        let path = dir_entry.path().to_string_lossy().to_string();

        let (score, indices) = matcher.fuzzy_indices(&file_name, query)?;

        Some(Self {
            name: file_name,
            path,
            score,
            indices,
        })
    }
}

impl Eq for SearchResult {}

impl PartialEq for SearchResult {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for SearchResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.score == other.score {
            Some(Ordering::Equal)
        } else if self.score > other.score {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Ord for SearchResult {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.score == other.score {
            Ordering::Equal
        } else if self.score > other.score {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
