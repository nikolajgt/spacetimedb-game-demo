use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};
use serde::{Deserialize, Serialize};
use crate::Screen;

#[derive(Serialize, Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

pub struct RegisterPage {
    pub email: String,
    pub password: String,
    focused_field: FieldFocus,
    auth_server_addr: String,
}

#[derive(PartialEq)]
pub enum FieldFocus {
    Email,
    Password,
}

impl RegisterPage {
    pub fn new() -> Self {
        let auth_server_addr = std::env::var("AUTH_SERVER_ADDR")
            .expect("Environment variable AUTH_SERVER_ADDR is required.");
        Self {
            email: String::new(),
            password: String::new(),
            focused_field: FieldFocus::Email,
            auth_server_addr
        }
    }

    pub async fn handle_register_input(&mut self, key: KeyEvent)-> Option<Screen>{
        println!("handle_register_input hit");
        match key.code {
            KeyCode::Enter => {
                match self.try_register().await {
                    Ok(_) => {return Some(Screen::Login)},
                    Err(e) => {  }
                };
            }
            KeyCode::Tab => {
                self.focused_field = match &self.focused_field {
                    FieldFocus::Email => FieldFocus::Password,
                    FieldFocus::Password => FieldFocus::Email,
                };

            }
            KeyCode::Char(c) => {
                match &self.focused_field {
                    FieldFocus::Email => self.email.push(c),
                    FieldFocus::Password => self.password.push(c),
                };
            }
            KeyCode::Backspace => {
                match &self.focused_field {
                    FieldFocus::Email => { self.email.pop(); },
                    FieldFocus::Password => { self.password.pop(); },
                }
            }
            KeyCode::Esc => {
                return Some(Screen::Login)
            }
            _ => {

            }
        }

        None
    }
    
    async fn try_register(&mut self) -> Result<(), String>{
        let client = reqwest::Client::new();
        let url = format!("{}/api/register", self.auth_server_addr.trim_end_matches('/'));
        
        match client
            .post(url)
            .json(&RegisterRequest {
                email: self.email.clone(),
                password: self.password.clone(),
            })
            .send()
            .await 
        {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("User registration failed: HTTP {}", response.status())),
            Err(e) => Err(format!("Request error while registration user: {e}")),
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(area);

        let email_input = Paragraph::new(self.email.as_str())
            .block(Block::default()
                .title(if self.focused_field == FieldFocus::Email {
                    "Email (focused)"
                } else {
                    "Email"
                })
                .borders(Borders::ALL));

        let password_input = Paragraph::new("*".repeat(self.password.len()))
            .block(Block::default()
                .title(if self.focused_field == FieldFocus::Password {
                    "Password (focused)"
                } else {
                    "Password"
                })
                .borders(Borders::ALL));

        frame.render_widget(email_input, layout[0]);
        frame.render_widget(password_input, layout[1]);

        let instructions = Paragraph::new("TAB to switch, ENTER to submit, q to quit");
        frame.render_widget(instructions, layout[2]);
    }
}