use crate::tracker;
use clippers::Clipboard;
use freya::prelude::*;
use rand::Rng;
use rfd::FileDialog;
use std::fs;

/// Returns default quotes as a vector of owned Strings.
///
/// Default quotes are embedded from `resources/default.csv`.
/// Each line is trimmed of surrounding double quotes.
pub fn get_quotes() -> Vec<String> {
    include_str!("../resources/default.csv")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim_matches('"').to_string())
        .collect()
}

/// Copies given text to the system clipboard.
///
/// # Arguments
///
/// * `text` - A string slice containing text to be copied.
pub fn copy_to_clipboard(text: &str) {
    let mut clipboard: Clipboard = Clipboard::get();
    clipboard.write_text(text).unwrap();
    println!("Copy button clicked! quote copied: {}", text);
}

/// Randomizes quote from provided list and updates the given signal.
///
/// Uses a persistent cache (via the tracker module) to avoid repeating recently used quotes.
/// The validated quote is then set as the current quote.
///
/// # Arguments
///
/// * `selected_quote` - A signal holding current quote (owned String).
/// * `quotes` - A slice of available quotes.
pub fn randomize_quote(mut selected_quote: Signal<String>, quotes: &[String]) {
    if quotes.is_empty() {
        println!("No quotes loaded.");
    } else {
        let mut rng = rand::rng();
        let index = rng.random_range(0..quotes.len());
        let candidate = &quotes[index];

        // Load tracker from persistent storage.
        let mut tracker = tracker::Tracker::load_default(10);
        let validated_quote = tracker::validate_quote(candidate, quotes, &mut tracker);
        tracker.save_default();

        selected_quote.set(validated_quote.clone());
        println!(
            "Randomize quote button clicked! New quote: {}",
            validated_quote
        );
    }
}

/// Opens file dialog for CSV files, reads selected file, returns its quotes.
///
/// Each non-empty line is trimmed of surrounding double quotes.
///
/// # Returns
///
/// * `Some(Vec<String>)` if a file is selected and read successfully.
/// * `None` if no file is selected or there is an error reading the file.
pub fn load_quotes_from_file() -> Option<Vec<String>> {
    if let Some(path) = FileDialog::new().add_filter("CSV", &["csv"]).pick_file() {
        match fs::read_to_string(&path) {
            Ok(content) => {
                let new_quotes: Vec<String> = content
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(|line| line.trim_matches('"').to_string())
                    .collect();
                println!("Loaded quotes from file: {:?}", path);
                Some(new_quotes)
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
