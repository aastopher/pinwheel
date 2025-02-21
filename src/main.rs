#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

/// Main module for Pinwheel application.
///
/// This module sets up the Freya GUI and launches the app.
mod utils;
mod tracker;

use freya::prelude::*;
use utils::{get_quotes, copy_to_clipboard, randomize_quote, load_quotes_from_file};

const ICON: &[u8] = include_bytes!("../assets/pinwheel.png");

/// Constructs root application element.
///
/// Wraps `Component` in a `ThemeProvider` to use dark theme.
fn app() -> Element {
    rsx!(
        ThemeProvider {
            theme: DARK_THEME,
            Component { }
        }
    )
}

/// Main component of the application.
///
/// This component sets up the UI layout including:
/// - A display area for currently selected quote.
/// - A button area with buttons for "Copy", "Randomize", and "Load" actions.
#[allow(non_snake_case)]
fn Component() -> Element {
    let theme: Theme = use_get_theme();
    let button_theme: &ButtonTheme = &theme.button;

    // Signal holding currently selected quote (owned String).
    let mut selected_quote: Signal<String> = use_signal(|| "I eat stickers all the time, dude!".to_string());
    // Signal holding list of quotes.
    let mut quotes: Signal<Vec<String>> = use_signal(|| get_quotes());

    rsx!(
        // Display box for selected quote.
        rect {
            height: "50%",
            width: "100%",
            padding: "20",
            main_align: "center",
            cross_align: "center",
            background: "rgb(163, 109, 250)",
            color: "rgb(221, 221, 221)",
            shadow: "0 4 20 5 rgb(0, 0, 0, 80)",
            label {
                font_size: "55",
                font_weight: "bold",
                "{selected_quote()}"
            }
        }
        // Button box for actions.
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            direction: "horizontal",
            background: "{button_theme.background}",
            color: "rgb(221, 221, 221)",
            Button {
                // Copy current quote to clipboard.
                onclick: move |_| {
                    copy_to_clipboard(selected_quote().as_str());
                },
                label { "Copy" }
            }
            Button {
                // Randomize current quote from list.
                onclick: move |_| {
                    randomize_quote(selected_quote, &quotes());
                },
                label { "Randomize" }
            }
            Button {
                // Load quotes from a CSV file.
                onclick: move |_| {
                    if let Some(new_quotes) = load_quotes_from_file() {
                        quotes.set(new_quotes);
                        if let Some(first) = quotes().first() {
                            selected_quote.set(first.clone());
                        }
                    } else {
                        println!("No file selected or error reading file.");
                    }
                },
                label { "Load" }
            }
        }
    )
}

/// Application entry point.
///
/// Configures and launches Freya GUI using specified window parameters.
fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_width(1200.0)
            .with_height(400.0)
            .with_title("Pinwheel")
            .with_icon(LaunchConfig::load_icon(ICON))
            .build(),
    );
}
