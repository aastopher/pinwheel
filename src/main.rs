#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod utils;
mod tracker;

use freya::prelude::*;
use utils::{get_titles, copy_to_clipboard, randomize_title, load_titles_from_file};

const ICON: &[u8] = include_bytes!("../assets/pinwheel.png");

fn app() -> Element {
    rsx!(
        ThemeProvider {
            theme: DARK_THEME,
            Component { }
        }
    )
}

#[allow(non_snake_case)]
fn Component() -> Element {
    let theme: Theme = use_get_theme();
    let button_theme: &ButtonTheme = &theme.button;

    // Signal holding the currently selected title (owned String).
    let mut selected_title: Signal<String> = use_signal(|| "I eat stickers all the time, dude!".to_string());
    // Signal holding the list of titles.
    let mut titles: Signal<Vec<String>> = use_signal(|| get_titles());

    rsx!(
        // Title display area
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
                "{selected_title()}"
            }
        }
        // Button area
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            direction: "horizontal",
            background: "{button_theme.background}",
            color: "rgb(221, 221, 221)",
            Button {
                onclick: move |_| {
                    copy_to_clipboard(selected_title().as_str());
                },
                label { "Copy" }
            }
            Button {
                onclick: move |_| {
                    randomize_title(selected_title, &titles());
                },
                label { "Randomize" }
            }
            Button {
                onclick: move |_| {
                    if let Some(new_titles) = load_titles_from_file() {
                        titles.set(new_titles);
                        if let Some(first) = titles().first() {
                            selected_title.set(first.clone());
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

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_width(1200.0)
            .with_height(400.0)
            .with_title("Title Pinwheel")
            .with_icon(LaunchConfig::load_icon(ICON))
            .build(),
    );
}
