use crate::{models::Database, persistence, TensileResult};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    VisionList,
    VisionDetail,
    ActionForm,
    MetricsSummary,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub db: Database,
    pub screen: Screen,
    pub input_mode: InputMode,
    pub selected_vision: Option<Uuid>,
    pub input_buffer: String,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> TensileResult<Self> {
        let db = persistence::load_database()?;
        Ok(App {
            db,
            screen: Screen::VisionList,
            input_mode: InputMode::Normal,
            selected_vision: None,
            input_buffer: String::new(),
            should_quit: false,
        })
    }

    pub fn refresh(&mut self) -> TensileResult<()> {
        self.db = persistence::load_database()?;
        Ok(())
    }

    pub fn save(&self) -> TensileResult<()> {
        persistence::save_database(&self.db)
    }

    pub fn handle_events(
        &mut self,
        event_handler: &super::events::EventHandler,
    ) -> TensileResult<bool> {
        use crossterm::event::{KeyCode, KeyModifiers};

        if let Some(key_event) = event_handler.last_key_event() {
            match self.input_mode {
                InputMode::Normal => self.handle_normal_mode(key_event)?,
                InputMode::Editing => self.handle_editing_mode(key_event)?,
            }
        }

        Ok(self.should_quit)
    }

    fn handle_normal_mode(&mut self, key_event: crossterm::event::KeyEvent) -> TensileResult<()> {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('1') => self.screen = Screen::VisionList,
            KeyCode::Char('2') => self.screen = Screen::MetricsSummary,
            KeyCode::Char('a') => {
                self.screen = Screen::ActionForm;
                self.input_mode = InputMode::Editing;
                self.input_buffer.clear();
            }
            KeyCode::Char('n') => {
                self.screen = Screen::VisionList;
                self.input_mode = InputMode::Editing;
                self.input_buffer.clear();
            }
            KeyCode::Up => self.select_previous_vision(),
            KeyCode::Down => self.select_next_vision(),
            KeyCode::Enter => {
                if self.selected_vision.is_some() {
                    self.screen = Screen::VisionDetail;
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_editing_mode(&mut self, key_event: crossterm::event::KeyEvent) -> TensileResult<()> {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char(c) => self.input_buffer.push(c),
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Enter => {
                self.process_input()?;
                self.input_mode = InputMode::Normal;
                self.input_buffer.clear();
            }
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.input_buffer.clear();
                self.screen = Screen::VisionList;
            }
            _ => {}
        }

        Ok(())
    }

    fn process_input(&mut self) -> TensileResult<()> {
        match self.screen {
            Screen::ActionForm => {
                if let Some(vision_id) = self.selected_vision {
                    let action =
                        crate::models::ActionLog::new(vision_id, self.input_buffer.clone());
                    self.db.actions.push(action);
                    self.save()?;
                }
            }
            Screen::VisionList if self.input_mode == InputMode::Editing => {
                let vision = crate::models::Vision::new(self.input_buffer.clone());
                self.db.visions.push(vision);
                self.save()?;
            }
            _ => {}
        }

        Ok(())
    }

    fn select_previous_vision(&mut self) {
        let visions: Vec<Uuid> = self.db.visions.iter().map(|v| v.id).collect();
        if let Some(selected) = self.selected_vision {
            if let Some(pos) = visions.iter().position(|&id| id == selected) {
                if pos > 0 {
                    self.selected_vision = Some(visions[pos - 1]);
                }
            }
        } else if !visions.is_empty() {
            self.selected_vision = Some(visions[0]);
        }
    }

    fn select_next_vision(&mut self) {
        let visions: Vec<Uuid> = self.db.visions.iter().map(|v| v.id).collect();
        if let Some(selected) = self.selected_vision {
            if let Some(pos) = visions.iter().position(|&id| id == selected) {
                if pos < visions.len() - 1 {
                    self.selected_vision = Some(visions[pos + 1]);
                }
            }
        } else if !visions.is_empty() {
            self.selected_vision = Some(visions[0]);
        }
    }

    pub fn get_selected_vision(&self) -> Option<&crate::models::Vision> {
        self.selected_vision
            .and_then(|id| self.db.visions.iter().find(|v| v.id == id))
    }

    pub fn get_active_visions_count(&self) -> usize {
        self.db
            .visions
            .iter()
            .filter(|v| !v.completed && matches!(v.state, crate::models::VisionState::InProgress))
            .count()
    }

    pub fn get_total_actions(&self) -> usize {
        self.db.actions.len()
    }
}
