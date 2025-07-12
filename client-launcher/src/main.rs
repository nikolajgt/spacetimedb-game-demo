mod security;

use std::io::stdout;
use std::time::Duration;
use clap::Parser;
use color_eyre::Result;
use crossterm::event::KeyCode;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen};
use ratatui::{DefaultTerminal, Frame, Terminal};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Constraint;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::security::{AuthenticationState};

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long, default_value_t = 250)]
    tick_rate: u64,

}

enum Screen {
    Login,
    Home,
    Loading,
    Error(String),
}

struct AppState {
    pub current_screen: Screen,
    pub auth_state: AuthenticationState
}

struct App {
    state: AppState,
    tick_rate: Duration,
    uni_code: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    color_eyre::install()?;
    let cli = Cli::parse();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    let stdout = stdout();
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default().await;
    let result = app.run(&mut terminal).await?;
    disable_raw_mode()?;
    terminal.show_cursor()?;
    
    Ok(())
}


impl App {
    async fn default() -> Self {
        Self {
            state: AppState::new(),
            tick_rate: Duration::from_millis(250),
            uni_code: true,
        }
    }

    async fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        match self.state.auth_state.attempt_auto_login().await {
            Ok(authenticated) => if authenticated {
                self.state.current_screen = Screen::Home;
            } else {
                self.state.current_screen = Screen::Login;
            },
            Err(e) => {
                eprintln!("Auto-login error: {e}");
                self.state.current_screen = Screen::Login;
            }
        }
        
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if crossterm::event::poll(Duration::from_millis(100))? {
                if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('c') => {
                                self.state.current_screen = match self.state.current_screen  {
                                Screen::Login => Screen::Home,
                                Screen::Home => Screen::Login,
                                _ => Screen::Login
                            };
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        match self.state.current_screen {
            Screen::Login => self.render_login(frame),
            Screen::Home => self.render_home(frame),
            _ => {}
        }
    }
    
    fn render_home(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .split(frame.area());

        let title = Paragraph::new("🏠 Home Screen")
            .block(Block::default().title("Dashboard").borders(Borders::ALL))
            .centered();

        frame.render_widget(title, layout[0]);
    }
    
    fn render_login(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .split(frame.area());

        let title = Paragraph::new("🛂 Login Screen")
            .block(Block::default().title("Welcome").borders(Borders::ALL))
            .centered();

        frame.render_widget(title, layout[0]);
    }
}

impl AppState {
    fn new() -> Self {
        Self {
            current_screen: Screen::Login,
            auth_state: AuthenticationState::default()
        }
    }
    
}