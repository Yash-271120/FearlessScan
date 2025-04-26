mod search;

use fuzzy_matcher::skim::SkimMatcherV2;
pub use search::SearchResult;

use crossbeam::channel::{bounded, Receiver, Sender};
use rayon::prelude::*;
use std::{
    collections::binary_heap::BinaryHeap,
    fs,
    path::{self, Path},
    sync::{Arc, Mutex},
    thread,
};
use walkdir::WalkDir;

const MAX_SEARCH_RESULTS: usize = 1000;
const MAX_PAR_DEPTH: usize = 3;

pub fn execute_search_low_memory(
    path: &Path,
    query: &str,
    matcher: &SkimMatcherV2,
    sender: &Sender<Option<SearchResult>>,
) {
    // let entries = match fs::read_dir(path) {
    //     Ok(read_dir) => read_dir.filter_map(Result::ok).collect::<Vec<_>>(),
    //     Err(_) => return,
    // };
    //
    //
    // let (files, dirs): (Vec<_>, Vec<_>) = entries
    //     .into_iter()
    //     .partition(|e| e.path().is_file());
    //
    // files.into_par_iter().for_each(|entry| {
    //     sender.send(SearchResult::try_from(&entry, matcher, query)).unwrap();
    // });
    //
    // dirs.into_par_iter().for_each(|entry| {
    //     execute_search_low_memory(&entry.path(), query, matcher, sender);
    // });

    let walkdir = WalkDir::new(path);

    walkdir
        .into_iter()
        .par_bridge()
        .filter_map(Result::ok)
        .for_each(|entry| {
            if entry.path().is_file() {
                sender
                    .send(SearchResult::try_from(&entry, matcher, query))
                    .unwrap();
            }
        });
}
