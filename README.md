# Crate Manager

================

This crate manager is a command-line utility that helps you create and manage Rust projects with specific dependencies. It uses the `clap` crate for command-line parsing.

## Usage

---

### Command-Line Arguments

#### `dir`

The directory where the new project will be created. Defaults to `/home/nerd/Dev/Rusty/`. This is my personal project directory.

#### `name`

The name of the new crate.

#### `crates`

A list of crates to add to the project, separated by spaces.

### Example

```bash
inirs --dir /path/to/project --name clap serde serde_json ...
```

### Crate Options

---

## Supported Libraries

---

Currently, full snippets are supported only for the following libraries:

- **Ratatui**: A Rust library for building terminal user interfaces.
- **Crossterm**: A cross-platform terminal handling library.
- **Clap**: A command-line argument parser.
- **Serde**: A serialization and deserialization library.
- **Tokio**: An asynchronous runtime for Rust.

All other libraries will be added in the future, but for now, only these libraries have comprehensive snippet support.

### Project Structure

---

The project structure will be as follows:

```bash
src/main.rs
Cargo.toml
```

### Build and Run

---

After running the crate manager, you can build and run your project using `cargo build` and `cargo run` respectively.

### Note

---

This crate manager assumes that you have `cargo` installed and configured on your system.
