/// use snippets

pub const USE_CLAP: &str = "use clap::{Parser, Subcommand};";
pub const USE_CROSSTERM: &str = r#"
use crossterm::{
    cursor::{self, Hide, Show},
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
"#;
pub const USE_RATATUI: &str = r#"
use crossterm::{
    cursor::{Hide, Show},
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};
"#;
pub const USE_SERDE: &str = "use serde::{Deserialize, Serialize};";

pub const CLAP: &str = r#"

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
pub const CROSSTERM: &str = r#"

#[derive(Debug, Clone)]
pub struct Screen {
    pub something: String,
}
impl Screen {
    pub fn new() -> Self {
        Self {
            something: String::new(),
        }
    }
    pub fn draw(&self) -> anyhow::Result<()> {
        let (x, y) = terminal::size()?;
        Ok(())
    }

    pub fn initialize(&mut self) {
        // enter your initialize code here
    }
    pub fn update(&mut self) {
        // enter your update code here
    }
    pub fn left(&mut self) {
        // enter your left code here
    }
    pub fn right(&mut self) {
        // enter your right code here
    }
    pub fn up(&mut self) {
        // enter your up code here
    }
    pub fn down(&mut self) {
        // enter your down code here
    }
}

fn main() -> anyhow::Result<()> {
    let mut stdout = std::io::stdout();
    enable_raw_mode()?;
    execute!(stdout, Hide, EnterAlternateScreen, DisableMouseCapture)?;

    let mut screen = Screen::new();

    let res = run(&mut screen);

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, EnableMouseCapture, Show)?;
    res?;
    Ok(())
}

fn run(screen: &mut Screen) -> anyhow::Result<()> {
    loop {
        let mut stdout = std::io::stdout();
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

        screen.draw()?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Down => println!("down"),
                KeyCode::Up => println!("up"),
                KeyCode::Left => println!("left"),
                KeyCode::Right => println!("right"),
                KeyCode::Enter => println!("enter"),
                _ => {}
            }
        }
    }
    Ok(())
}

"#;

pub const RATATUI: &str = r#"

#[derive(Debug, Clone)]
pub struct Screen {
    pub something: String,
}
impl Screen {
    pub fn new() -> Self {
        Self {
            something: String::new(),
        }
    }
    pub fn initialize(&mut self) {
        // enter your initialize code here
    }
    pub fn update(&mut self) {
        // enter your update code here
    }
    pub fn draw(&self, f: &mut Frame) {
        // enter your draw code here
    }
    pub fn left(&mut self) {
        // enter your left code here
    }
    pub fn right(&mut self) {
        // enter your right code here
    }
    pub fn up(&mut self) {
        // enter your up code here
    }
    pub fn down(&mut self) {
        // enter your down code here
    }
}

fn main() -> anyhow::Result<()> {
    let mut stdout = std::io::stdout();
    enable_raw_mode()?;
    execute!(stdout, Hide, EnterAlternateScreen, DisableMouseCapture)?;

    let mut term = Terminal::new(CrosstermBackend::new(stdout))?;
    let mut screen = Screen::new();

    let res = run(&mut term, &mut screen);

    disable_raw_mode()?;
    execute!(term.backend_mut(), LeaveAlternateScreen, EnableMouseCapture)?;
    execute!(term.backend_mut(), Show)?;
    res?;
    Ok(())
}

fn run<B: Backend>(term: &mut Terminal<B>, screen: &mut Screen) -> anyhow::Result<()> {
    loop {
        term.draw(|f| screen.draw(f))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Down => println!("down"),
                KeyCode::Up => println!("up"),
                KeyCode::Left => println!("left"),
                KeyCode::Right => println!("right"),
                KeyCode::Enter => println!("enter"),
                _ => {}
            }
        }
    }
    Ok(())
}

"#;

pub const SERDE: &str = r#"

// work with any structs that implement serde::Serialize and serde::Deserialize
pub fn load_from_file() -> String /* Some custopm type */ {
    let dir_path = format!("/path/to/dir/");
    let file_path = format!("{}/path/to/file", dir_path);
    let mut file = std::fs::File::open(file_path.clone()).unwrap_or_else(|_| {
        let _ = std::fs::create_dir_all(dir_path.clone());
        let _ = std::fs::File::create(file_path.clone());
        std::fs::File::open(file_path).unwrap()
    });
    let questions: String /* Some custopm type */ =
        serde_json::from_reader(&mut file).unwrap_or(Questions { questions: vec![] });
    questions
}
pub fn save_to_file(struct: String /* Some custopm type */) {
    let dir_path = format!("/path/to/dir/");
    let file_path = format!("{}/path/to/file", dir_path);
    let mut file = std::fs::File::create(file_path.clone()).unwrap_or_else(|_| {
        let _ = std::fs::create_dir_all(dir_path.clone());
        std::fs::File::create(file_path.clone()).unwrap()
    });
    serde_json::to_writer(&mut file, struct).unwrap();
}

"#;

pub const TOKIO: &str = r#"

#[tokio::main]
async fn main() {
    println!("hello");
}

"#;

/// Using only without RATATUI, CROSSTERM, TOKIO
pub const DEFAULT: &str = r#"

fn main() {
    println!("hello");
}

"#;
