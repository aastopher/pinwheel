use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// A persistent tracker that stores recently used quotes.
///
/// This tracker helps ensure that recently used quotes are not repeated.
/// Tracker is saved as JSON in the platform-specific cache directory.
#[derive(Serialize, Deserialize, Debug)]
pub struct Tracker {
    /// A vector of recently used quotes.
    pub last_quotes: Vec<String>,
    /// Maximum capacity of the tracker.
    pub capacity: usize,
}

impl Tracker {
    /// Creates a new, empty tracker with the given capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of recent quotes to track.
    pub fn new(capacity: usize) -> Self {
        Tracker {
            last_quotes: Vec::new(),
            capacity,
        }
    }

    /// Returns application cache file path.
    ///
    /// Tracker file is stored in the cache directory for the application.
    pub fn get_tracker_path() -> Option<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("org", "TakashiTech", "Pinwheel") {
            let mut path = proj_dirs.cache_dir().to_path_buf();
            // Ensure the cache directory exists.
            let _ = fs::create_dir_all(&path);
            path.push("tracker.json");
            Some(path)
        } else {
            None
        }
    }

    /// Loads tracker from the default file location.
    ///
    /// If file does not exist or cannot be parsed, returns a new tracker.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of recent quotes to track.
    pub fn load_default(capacity: usize) -> Self {
        if let Some(path) = Tracker::get_tracker_path() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(tracker) = serde_json::from_str::<Tracker>(&content) {
                    return tracker;
                }
            }
        }
        Tracker::new(capacity)
    }

    /// Saves tracker to default file location.
    pub fn save_default(&self) {
        if let Some(path) = Tracker::get_tracker_path() {
            if let Ok(json) = serde_json::to_string_pretty(self) {
                let _ = fs::write(path, json);
            }
        }
    }

    /// Checks if a quote is already tracked.
    ///
    /// # Arguments
    ///
    /// * `quote` - A string slice representing the quote to check.
    pub fn is_tracked(&self, quote: &str) -> bool {
        self.last_quotes.contains(&quote.to_string())
    }

    /// Adds a new quote to the tracker.
    ///
    /// If the quote already exists, it’s re-added at the end (most recent).
    /// If the tracker exceeds its capacity, the oldest quote is removed.
    ///
    /// # Arguments
    ///
    /// * `quote` - An owned `String` containing the quote.
    pub fn add_quote(&mut self, quote: String) {
        // Remove any existing instance to update its recency.
        self.last_quotes.retain(|t| t != &quote);
        self.last_quotes.push(quote);
        if self.last_quotes.len() > self.capacity {
            self.last_quotes.remove(0);
        }
    }
}

/// Validates candidate quote against the tracker.
///
/// If the candidate is already tracked, iterates through quotes list
/// (wrapping around) to find a quote that isn’t tracked. If all quotes are tracked,
/// returns the candidate.
///
/// # Arguments
///
/// * `candidate` - A string slice representing the candidate quote.
/// * `quotes` - A slice of all available quotes.
/// * `tracker` - A mutable reference to the current tracker.
///
/// # Returns
///
/// A validated quote as an owned `String`.
pub fn validate_quote(candidate: &str, quotes: &[String], tracker: &mut Tracker) -> String {
    if tracker.is_tracked(candidate) {
        if let Some(index) = quotes.iter().position(|t| t == candidate) {
            for offset in 1..quotes.len() {
                let new_index = (index + offset) % quotes.len();
                let potential = &quotes[new_index];
                if !tracker.is_tracked(potential) {
                    tracker.add_quote(potential.clone());
                    return potential.clone();
                }
            }
        }
        candidate.to_string()
    } else {
        tracker.add_quote(candidate.to_string());
        candidate.to_string()
    }
}
