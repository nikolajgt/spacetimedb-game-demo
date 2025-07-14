use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::prelude::*;
use crate::AppContext;

pub mod login_page;
pub mod register_page;
pub mod home_page;


pub trait Page {
    fn render(&mut self, f: &mut Frame, area: Rect);
    fn handle_input(&mut self, event: &KeyEvent, ctx: &mut AppContext);
    fn on_tick(&mut self, ctx: &mut AppContext);
}