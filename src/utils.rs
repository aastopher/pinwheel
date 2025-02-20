use crate::tracker;
use clippers::Clipboard;
use freya::prelude::*;
use rand::Rng;
use rfd::FileDialog;
use std::fs;

/// Returns the default titles as a vector of owned Strings.
pub fn get_titles() -> Vec<String> {
    include_str!("../resources/default_titles.csv")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim_matches('"').to_string())
        .collect()
}

/// Copies the given text to the clipboard.
pub fn copy_to_clipboard(text: &str) {
    let mut clipboard: Clipboard = Clipboard::get();
    clipboard.write_text(text).unwrap();
    println!("Copy button clicked! Title copied: {}", text);
}

// /// Randomizes the title from the provided list and updates the given signal.
// /// Any surrounding double quotes are removed.
// pub fn randomize_title(mut selected_title: Signal<String>, titles: &[String]) {
//     if titles.is_empty() {
//         println!("No titles loaded.");
//     } else {
//         let mut rng = rand::rng();
//         let index = rng.random_range(0..titles.len());
//         let random_title = &titles[index];
//         selected_title.set(random_title.clone());
//         println!("Randomize title button clicked! New title: {}", random_title);
//     }
// }

pub fn randomize_title(mut selected_title: Signal<String>, titles: &[String]) {
    if titles.is_empty() {
        println!("No titles loaded.");
    } else {
        let mut rng = rand::rng();
        let index = rng.random_range(0..titles.len());
        let candidate = &titles[index];

        // Define the cache file path.
        let cache_file = "assets/cache.json";
        // Load the tracker with a capacity of 3.
        let mut tracker = tracker::Tracker::load(cache_file, 5);
        // Validate the candidate against the tracker.
        let validated_title = tracker::validate_title(candidate, titles, &mut tracker);
        // Save the updated tracker to persist the cache.
        tracker.save(cache_file);

        selected_title.set(validated_title.clone());
        println!("Randomize title button clicked! New title: {}", validated_title);
    }
}


/// Opens a file dialog for CSV files, reads the selected file, and returns its titles.
pub fn load_titles_from_file() -> Option<Vec<String>> {
    if let Some(path) = FileDialog::new().add_filter("CSV", &["csv"]).pick_file() {
        match fs::read_to_string(&path) {
            Ok(content) => {
                let new_titles: Vec<String> = content
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(|line| line.trim_matches('"').to_string())
                    .collect();
                println!("Loaded titles from file: {:?}", path);
                Some(new_titles)
            }
            Err(e) => {
                println!("Error reading file: {}", e);
                None
            }
        }
    } else {
        None
    }
}
