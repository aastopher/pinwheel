# Pinwheel

Pinwheel is a simple Rust application built using [Freya](https://github.com/PlayForm/Freya) GUI framework. Designed to help randomly select from a set of quotes. Load quotes from a CSV file containing a single column of quotes, and ensures that recently used quotes (5) aren’t repeated using persistent cache.

## Features

- **Custom CSV Input:**  
  Load quotes from a CSV file with a single column. A default set is provided, but you can load a new file using the "Load" button.

- **Persistent Cache:**  
  Recently selected quotes are cached in an internal file stored in an OS-specific cache directory. Preventing repetition across sessions.

---

![app_preview](./assets/preview.png)