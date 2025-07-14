mod security;
mod pages;

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
use tokio::sync::mpsc::Sender;
use crate::pages::login_page::LoginInput;
use crate::pages::register_page::RegisterPage;
use crate::security::{AuthenticationState};

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, default_value_t = 250)]
    tick_rate: u64,
}

#[derive(PartialOrd, PartialEq, Debug)]
enum Screen {
    Register,
    Login,
    Home,
    Loading,
    Error(String),
}

enum AppCommand {
    ToPage(Screen),
    Logout,
    Quit,
}

struct AppState {
    pub current_screen: Screen,
    pub auth_state: AuthenticationState,
}


pub struct AppContext<'a> {
    pub auth: &'a mut AuthenticationState,
    pub tx: Sender<AppCommand>, 
}

struct App {
    state: AppState,
    tick_rate: Duration,
    uni_code: bool,
    login: LoginInput,
    register: RegisterPage,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    color_eyre::install()?;
    let cli = Args::parse();
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
            login: LoginInput::new(),
            register: RegisterPage::new(),
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
                    match self.state.current_screen {
                        Screen::Login => {
                            if let Some(new_screen) = self.login.handle_login_input(key) {
                                println!("new screen: {:?}", new_screen);
                                self.state.current_screen = new_screen;
                            }
                        }
                        Screen::Register => {
                            if let Some(new_screen) = self.register.handle_register_input(key).await {
                                println!("new screen: {:?}", new_screen);
                                self.state.current_screen = new_screen;
                            }
                        }
                        // Screen::Home => {
                        //     if let Some(new_screen) = self.handle_home_input(key) {
                        //         self.state.current_screen = new_screen;
                        //     }
                        // }
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
            Screen::Register => self.render_register(frame),
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
    
    fn render_register(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .split(frame.area());

        let title = Paragraph::new("🛂 Register Screen")
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

impl Screen {
    fn is_authenticated(&self) -> bool {
        matches!(self, Screen::Home)
    }

    fn is_public(&self) -> bool {
        matches!(self, Screen::Login | Screen::Register)
    }
}
