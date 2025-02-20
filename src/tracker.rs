use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tracker {
    pub last_titles: Vec<String>,
    pub capacity: usize,
}

impl Tracker {
    /// Creates a new, empty tracker with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Tracker {
            last_titles: Vec::new(),
            capacity,
        }
    }

    /// Loads the tracker from the given file path.
    /// If the file does not exist or cannot be parsed, returns a new tracker.
    pub fn load<P: AsRef<Path>>(path: P, capacity: usize) -> Self {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(tracker) = serde_json::from_str::<Tracker>(&content) {
                return tracker;
            }
        }
        Tracker::new(capacity)
    }

    /// Saves the tracker as JSON to the given file path.
    pub fn save<P: AsRef<Path>>(&self, path: P) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }

    /// Checks if a title is already in the tracker.
    pub fn is_tracked(&self, title: &str) -> bool {
        self.last_titles.contains(&title.to_string())
    }

    /// Adds a new title to the tracker.
    /// If the title already exists, it is re-added at the end (making it the most recent).
    /// If the capacity is exceeded, the oldest title is removed.
    pub fn add_title(&mut self, title: String) {
        // Remove existing entry (if any) to update its position.
        self.last_titles.retain(|t| t != &title);
        self.last_titles.push(title);
        if self.last_titles.len() > self.capacity {
            self.last_titles.remove(0);
        }
    }
}

/// Validates the candidate title by checking it against the tracker.
/// If the candidate is already tracked, it finds the next available title (wrapping around
/// the titles list) that isn’t in the tracker. If all titles are tracked, it returns the candidate.
pub fn validate_title(candidate: &str, titles: &[String], tracker: &mut Tracker) -> String {
    if tracker.is_tracked(candidate) {
        if let Some(index) = titles.iter().position(|t| t == candidate) {
            // Loop through titles starting from the next index.
            for offset in 1..titles.len() {
                let new_index = (index + offset) % titles.len();
                let potential = &titles[new_index];
                if !tracker.is_tracked(potential) {
                    tracker.add_title(potential.clone());
                    return potential.clone();
                }
            }
        }
        // If all titles are tracked, return the candidate.
        candidate.to_string()
    } else {
        tracker.add_title(candidate.to_string());
        candidate.to_string()
    }
}
