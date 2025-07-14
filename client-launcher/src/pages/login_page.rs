use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::Screen;


pub struct LoginPage2 {
    pub input: LoginInput,
    pub auth_server_addr: String,
}




pub struct LoginInput {
    pub email: String,
    pub password: String,
    pub focused_field: FieldFocus,
}

#[derive(PartialEq)]
pub enum FieldFocus {
    Email,
    Password,
}

impl LoginInput {
    pub fn new() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            focused_field: FieldFocus::Email,
        }
    }

    pub fn handle_login_input(&mut self, key: KeyEvent) -> Option<Screen>{
        match key.code {
            KeyCode::Enter => {
                self.try_login();
            }
            KeyCode::Tab => {
                self.focused_field = match self.focused_field {
                    FieldFocus::Email => FieldFocus::Password,
                    FieldFocus::Password => FieldFocus::Email,
                };
                
            }

            KeyCode::Char(c) => {
                match self.focused_field {
                    FieldFocus::Email => self.email.push(c),
                    FieldFocus::Password => self.password.push(c),
                };
            }
            KeyCode::Backspace => {
                match self.focused_field {
                    FieldFocus::Email => { self.email.pop(); },
                    FieldFocus::Password => { self.password.pop(); },
                }
            }
            KeyCode::Esc => {
                println!("change to Screen::Register");
                return Some(Screen::Register)
            }
            _ => {
                
            }
        }

        None
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
    
    fn try_login(&mut self) {
        
    }
}