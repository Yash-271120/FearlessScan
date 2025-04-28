use rayon::prelude::*;
use std::{cmp::Ordering, time::Instant};
use tauri::State;
use walkdir::DirEntry;

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use serde::Serialize;

use crate::{error::MyError, SafeMyState};

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

pub fn check_file(
    file_name: &str,
    file_path: &str,
    matcher: &SkimMatcherV2,
    query: &str,
) -> Option<SearchResult> {
    let (score, indices) = matcher.fuzzy_indices(file_name, query)?;
    Some(SearchResult {
        name: file_name.to_string(),
        path: file_path.to_string(),
        score,
        indices,
    })
}

#[tauri::command]
pub async fn search_directory_fast(
    state_mux: State<'_, SafeMyState>,
    mount_point: String,
    query: String,
    dir_path: String,
) -> Result<Vec<SearchResult>, MyError> {
    let start_time = Instant::now();
    let matcher = SkimMatcherV2::default().smart_case();
    let query = query.trim().to_lowercase();
    let state = state_mux.lock().unwrap();

    let mut results = Vec::new();

    let storage_cache = state.storage_cache.get(&mount_point).unwrap();

    for (filename, paths) in storage_cache {
        for path in paths {
            let file_path = &path.file_path;
        
            if !file_path.starts_with(&dir_path) {
                continue;
            }
        
            let search_result = check_file(&filename, &file_path, &matcher, &query);
        
            if let Some(result) = search_result {
                results.push(result);
            }
        
        }
    }

    results.sort_by(|a, b| b.score.cmp(&a.score));
    println!("Search took {:?}", start_time.elapsed());
    Ok(results)
}