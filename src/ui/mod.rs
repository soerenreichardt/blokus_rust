use std::collections::HashMap;
use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

use crate::game::{Game, Position};
use crate::ui::board_display::BoardDisplay;

mod cursor_scrollbar;
mod board_display;

const UI_OFFSET: u16 = 2;

#[derive(Default)]
struct App {
    modules: HashMap<ModuleKind, Box<dyn Module>>
}

pub trait Module {
    fn update(&mut self, event: AppEvent);
    fn render(&mut self, frame: &mut Frame, area: &Rect, game: &mut Game);
    fn kind(&self) -> ModuleKind;
}

#[derive(Eq, Hash, PartialEq)]
enum ModuleKind {
    BoardDisplay
}

struct Cursor {
    position: Position,
    max_x: i32,
    max_y: i32
}

#[derive(Copy, Clone, Debug)]
enum AppEvent {
    Quit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    None
}

pub fn run(game: &mut Game) -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::default();

    let horizontal = Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)]);
    let vertical = Layout::vertical([Constraint::Percentage(30), Constraint::Percentage(70)]);
    let board_display = BoardDisplay::new(game.width(), game.height());
    app.add_module(board_display);

    loop {
        terminal.draw(|frame| {
            let [board_area, side_menu_area] = horizontal.areas(frame.size());
            let [player_area, piece_area] = vertical.areas(side_menu_area);

            let areas = vec![
                (ModuleKind::BoardDisplay, board_area)
            ].into_iter().collect::<HashMap<ModuleKind, Rect>>();
            app.render_modules(frame, game, areas)
        })?;

        let event = poll_event()?;
        app.update_modules(event);

        match event {
            AppEvent::Quit => break,
            _ => ()
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn poll_event() -> io::Result<AppEvent> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(AppEvent::Quit),
                    KeyCode::Up => return Ok(AppEvent::MoveUp),
                    KeyCode::Down => return Ok(AppEvent::MoveDown),
                    KeyCode::Left => return Ok(AppEvent::MoveLeft),
                    KeyCode::Right => return Ok(AppEvent::MoveRight),
                    _ => ()
                }
            }
        }
    }
    Ok(AppEvent::None)
}

impl App {
    fn add_module(&mut self, module: impl Module + 'static) {
        self.modules.insert(module.kind(), Box::new(module));
    }

    fn update_modules(&mut self, event: AppEvent) {
        for (_, module) in self.modules.iter_mut() {
            module.update(event)
        }
    }

    fn render_modules(&mut self, frame: &mut Frame, game: &mut Game, areas: HashMap<ModuleKind, Rect>) {
        for (kind, module) in self.modules.iter_mut() {
            module.render(frame, areas.get(kind).unwrap(), game)
        }
    }
}

impl Cursor {
    fn new(max_x: i32, max_y: i32) -> Self {
        Cursor {
            max_x,
            max_y,
            position: Position::default()
        }
    }

    fn move_down(&mut self) {
        if self.position.y < self.max_y - 1 {
            self.position.y += 1
        }
    }

    fn move_up(&mut self) {
        if self.position.y > 0 {
            self.position.y -= 1
        }
    }

    fn move_right(&mut self) {
        if self.position.x < self.max_x - 1 {
            self.position.x += 1
        }
    }

    fn move_left(&mut self) {
        if self.position.x > 0 {
            self.position.x -= 1
        }
    }
}