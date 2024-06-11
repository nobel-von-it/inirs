# Crate Manager

================

This crate manager is a command-line utility that helps you create and manage Rust projects with specific dependencies. It uses the `clap` crate for command-line parsing.

## Usage

---

### Command-Line Arguments

#### `dir`

The directory where the new project will be created. Defaults to `/home/nerd/Dev/Rusty/`. This is my personal project directory.

#### `crate_name`

The name of the new crate.

#### `crates`

A comma-separated list of crates to add to the project.

### Example

```bash
inirs --dir /path/to/project --crate-name my_crate --crates clap,serde,serde_json
```

### Crate Options

---

The following crates are supported:

#### `clap`

Adds the `clap` crate for command-line parsing.

#### `serde`

Adds the `serde` crate for data serialization.

#### `serde_json`

Adds the `serde_json` crate for JSON serialization.

#### `ratatui`

Adds the `ratatui` crate for terminal user interfaces.

#### `crossterm`

Adds the `crossterm` crate for cross-platform terminal handling.

#### `whoami`

Adds the `whoami` crate for user information.

#### `anyhow`

Adds the `anyhow` crate for error handling.

#### `rand`

Adds the `rand` crate for random number generation.

### Scripts

---

The following scripts are added to the project:

#### `clap`

Adds a basic `clap` script for command-line parsing.

#### `serde`

Adds a basic `serde` script for data serialization.

#### `ratatui`

Adds a basic `ratatui` script for terminal user interfaces.

#### `crossterm`

Adds a basic `crossterm` script for cross-platform terminal handling.

#### `main`

Adds a basic `main` script for the project.

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
