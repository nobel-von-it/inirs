use clap::Parser;
use std::{fs::File, io::Write};

const MAIN_SCRIPT: &str = r#"
fn main() {
    println!("hello");
}
"#;
const CROSSTERM_RATATUI_SCRIPT: &str = r#"

use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io::stdout;

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let mut t = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut screen = Screen::new();

    let res = run(&mut t, &mut screen);

    disable_raw_mode()?;
    execute!(t.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    t.show_cursor()?;

    res?;
    Ok(())
}
fn run<B: Backend>(t: &mut Terminal<B>, screen: &mut screen::Screen) -> anyhow::Result<()> {
    loop {
        t.draw(|f| screen.draw(f))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Down => screen.question.down(),
                KeyCode::Up => screen.question.up(),
                KeyCode::Enter => match screen.state {
                    screen::State::Menu => screen.start_game(),
                    screen::State::Game => screen.update(),
                    _ => {}
                },
                _ => {}
            }
        }
    }
    Ok(())
}

"#;
const CROSSTERM_SCRIPT: &str = r#"

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let res = run();

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    res?;
    Ok(())
}

fn run() -> anyhow::Result<()> {
    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            execute!(
                stdout(),
                crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
                crossterm::cursor::MoveTo(0, 0)
            )?;
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Down => println!("down"),
                KeyCode::Up => println!("up"),
                KeyCode::Enter => println!("enter"),
                _ => {}
            }
        }
    }
    Ok(())
}

"#;
const CLAP_SCRIPT: &str = r#"

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(arg_required_else_help = true)]
    Add {
        #[clap(short, long)]
        something: String,
    },
}

"#;
const SCREEN_SCRIPT: &str = r#"

use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Screen {
    pub something: String,
}
impl Screen {
    pub fn new() -> Self {
        Screen {
            something: String::from("hello"),
        }
    }
    pub fn initialize(&mut self) {
        // enter your initialize code here
    }
    pub fn update(&mut self) {
        // enter your update code here
    }
    pub fn draw(&mut self, f: &mut ratatui::Frame) {
        // enter your draw code here
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    // for example
    Menu,
    Exit,
    Game,
}

"#;
const SERDE_SCRIPT: &str = r#"

use serde::{Deserialize, Serialize};
pub fn load_from_file() -> Self {
    let dir_path = format!("/home/{}/.enc/", whoami::username());
    let file_path = format!("{}/questions.json", dir_path);
    let mut file = std::fs::File::open(file_path.clone()).unwrap_or_else(|_| {
        let _ = std::fs::create_dir_all(dir_path.clone());
        let _ = std::fs::File::create(file_path.clone());
        std::fs::File::open(file_path).unwrap()
    });
    let questions: Questions =
        serde_json::from_reader(&mut file).unwrap_or(Questions { questions: vec![] });
    questions
}
pub fn save_to_file(&self) {
    let dir_path = format!("/home/{}/.enc/", whoami::username());
    let file_path = format!("{}/questions.json", dir_path);
    let mut file = std::fs::File::create(file_path.clone()).unwrap_or_else(|_| {
        let _ = std::fs::create_dir_all(dir_path.clone());
        std::fs::File::create(file_path.clone()).unwrap()
    });
    serde_json::to_writer(&mut file, self).unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choices {
    pub question: String,
    pub choices: Vec<Choice>,
    pub index: usize,
}

impl Choices {
    pub fn cmp(&self) -> bool {
        self.choices[self.index].correct
    }
    pub fn up(&mut self) {
        if self.index == 0 {
            self.index = self.choices.len() - 1;
        } else {
            self.index -= 1
        }
    }
    pub fn down(&mut self) {
        if self.index == self.choices.len() - 1 {
            self.index = 0
        } else {
            self.index += 1
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub text: String,
    pub correct: bool,
}

"#;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Crate {
    Clap,
    Serde,
    SerdeJson,
    Ratatui,
    Crossterm,
    Whoami,
    Anyhow,
    Rand,
    None,
}
impl Crate {
    pub fn new(s: &str) -> Self {
        match s {
            "clap" => Crate::Clap,
            "serde" => Crate::Serde,
            "serde_json" => Crate::SerdeJson,
            "ratatui" => Crate::Ratatui,
            "crossterm" => Crate::Crossterm,
            "whoami" => Crate::Whoami,
            "anyhow" => Crate::Anyhow,
            "rand" => Crate::Rand,
            _ => Crate::None,
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "/home/nerd/Dev/Rusty/")]
    dir: String,
    #[clap(short, long)]
    crate_name: String,
    crates: String,
}
impl Args {
    fn create_dir(&self) {
        let _ = std::fs::create_dir_all(self.dir.clone());
        let mut com = std::process::Command::new("cargo")
            .args(["new", "--bin", &self.crate_name])
            .current_dir(&self.dir)
            .spawn()
            .unwrap();
        if com.wait().unwrap().success() {
            println!("{} created", self.crate_name);
        }
    }
    fn add_all_crates(&self) {
        let path = format!("{}{}", &self.dir, &self.crate_name);
        for crate_ in self.crates.split(',') {
            let crate_ = Crate::new(crate_);
            match crate_ {
                Crate::Clap => {
                    let mut com = std::process::Command::new("cargo")
                        .args(["add", "clap", "--features", "derive"])
                        .current_dir(&path)
                        .spawn()
                        .unwrap();
                    if com.wait().unwrap().success() {
                        println!("clap added");
                    }
                }
                Crate::Serde => {
                    let mut com = std::process::Command::new("cargo")
                        .args(["add", "serde", "--features", "derive"])
                        .current_dir(&path)
                        .spawn()
                        .unwrap();
                    if com.wait().unwrap().success() {
                        println!("serde added");
                    }
                }
                Crate::SerdeJson => {
                    let mut com = std::process::Command::new("cargo")
                        .args(["add", "serde_json"])
                        .current_dir(&path)
                        .spawn()
                        .unwrap();
                    if com.wait().unwrap().success() {
                        println!("serde_json added");
                    }
                }
                Crate::Ratatui => {
                    let mut com = std::process::Command::new("cargo")
                        .args(["add", "ratatui", "crossterm"])
                        .current_dir(&path)
                        .spawn()
                        .unwrap();
                    if com.wait().unwrap().success() {
                        println!("ratatui and crossterm added");
                    }
                }
                Crate::Whoami => {
                    let mut com = std::process::Command::new("cargo")
                        .args(["add", "whoami"])
                        .current_dir(&path)
                        .spawn()
                        .unwrap();
                    if com.wait().unwrap().success() {
                        println!("whoami added");
                    }
                }
                Crate::Anyhow => {
                    let mut com = std::process::Command::new("cargo")
                        .args(["add", "anyhow"])
                        .current_dir(&path)
                        .spawn()
                        .unwrap();
                    if com.wait().unwrap().success() {
                        println!("anyhow added");
                    }
                }
                Crate::Rand => {
                    let mut com = std::process::Command::new("cargo")
                        .args(["add", "rand"])
                        .current_dir(&path)
                        .spawn()
                        .unwrap();
                    if com.wait().unwrap().success() {
                        println!("rand added");
                    }
                }
                Crate::Crossterm => {
                    let mut com = std::process::Command::new("cargo")
                        .args(["add", "crossterm"])
                        .current_dir(&path)
                        .spawn()
                        .unwrap();
                    if com.wait().unwrap().success() {
                        println!("crossterm added");
                    }
                }
                Crate::None => {
                    println!("unknown crate: {:?}", crate_);
                }
            }
        }
    }
    fn build(&self) {
        let path = format!("{}{}", &self.dir, &self.crate_name);
        let mut com = std::process::Command::new("cargo")
            .args(["build"])
            .current_dir(&path)
            .spawn()
            .unwrap();
        if com.wait().unwrap().success() {
            println!("{} built", self.crate_name);
        }
    }
    fn add_all_scripts(&self) {
        let path = format!("{}{}/src/main.rs", &self.dir, &self.crate_name);
        let mut file = File::create(path).unwrap();
        for crate_ in self.crates.split(',') {
            let crate_ = Crate::new(crate_);
            match crate_ {
                Crate::Ratatui => {
                    let _ = file.write_all(CROSSTERM_RATATUI_SCRIPT.as_bytes());
                    let _ = file.write_all(SCREEN_SCRIPT.as_bytes());
                }
                Crate::Serde => {
                    let _ = file.write_all(SERDE_SCRIPT.as_bytes());
                }
                Crate::Crossterm => {
                    let _ = file.write_all(CROSSTERM_SCRIPT.as_bytes());
                    let _ = file.write_all(SCREEN_SCRIPT.as_bytes());
                }
                Crate::Clap => {
                    let _ = file.write_all(CLAP_SCRIPT.as_bytes());
                }
                Crate::None => {
                    println!("unknown crate: {:?}", crate_);
                }
                _ => {}
            }
        }
        let _ = file.write_all(MAIN_SCRIPT.as_bytes());
    }
}

fn main() {
    let args = Args::parse();
    args.create_dir();
    args.add_all_crates();
    args.add_all_scripts();
    args.build();
}
