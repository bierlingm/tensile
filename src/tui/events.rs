use crate::TensileResult;
use crossterm::event::{self, Event, KeyEvent};
use std::time::Duration;

#[allow(unused_imports)]
use crossterm::event::{KeyCode, KeyModifiers};

pub struct EventHandler {
    last_key_event: Option<KeyEvent>,
}

impl EventHandler {
    pub fn new() -> Self {
        EventHandler {
            last_key_event: None,
        }
    }

    pub fn poll(&mut self) -> TensileResult<bool> {
        self.last_key_event = None;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                self.last_key_event = Some(key_event);
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn last_key_event(&self) -> Option<KeyEvent> {
        self.last_key_event
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}
