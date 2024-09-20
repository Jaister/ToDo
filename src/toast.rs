use std::io;
use std::io::Write;
use std::fs;
use std::fmt;
use std::string;
use serde_json::{Result, Value, Error,json};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};
use std::time::Duration;
use crate::tarea::Tarea;
use crate::tarea::{generar_id, guardar_json};

struct Toast {
    message: String,
    visible: bool,
}


impl Toast {
    fn toast_new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            visible: true,
        }
    }

    fn toast_render(&self, f: &mut Frame, area: Rect) {
        if self.visible {
            let toast_block = Block::default()
                .title("Warning")
                .style(Style::default().bg(Color::Yellow).fg(Color::Black));

            f.render_widget(toast_block, area);
            f.render_widget(Paragraph::new(&self.message), area);
        }
    }

    fn toast_hide(&mut self) {
        self.visible = false;
    }
}