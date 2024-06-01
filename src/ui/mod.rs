use std::collections::{HashMap, VecDeque};
use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

use crate::game::Game;
use crate::ui::board_module::BoardDisplay;
use crate::ui::piece_module::PieceDisplay;
use crate::ui::player_module::PlayerDisplay;

mod scrollbars;
mod board_module;
mod player_module;
mod piece_module;

const BLOCK: &str = "██";
const SHADED_BLOCK: &str = "░░";
const UI_OFFSET: u16 = 2;

#[derive(Default)]
struct App {
    modules: HashMap<ModuleKind, Box<dyn Module>>
}

pub(crate) trait Module {
    fn update(&mut self, event: AppEvent, game: &mut Game) -> Option<AppEvent>;
    fn render(&mut self, frame: &mut Frame, area: Rect, game: &mut Game);
    fn kind(&self) -> ModuleKind;
}

pub trait RenderCanvas {
    fn render(&self) -> Vec<Line<'_>>;
}

#[derive(Eq, Hash, PartialEq)]
pub(crate) enum ModuleKind {
    Board,
    Player,
    Piece
}

#[derive(Default)]
struct Cursor {
    area: Rect,
    max_x: u16,
    max_y: u16
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum AppEvent {
    Quit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    OpenPieceSelection,
    PieceSelected(usize),
    Select,
    Rotate,
    PiecePlaced,
    None
}

pub fn run(game: &mut Game) -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut event_queue = VecDeque::new();
    let mut app = App::default();

    app.add_module(BoardDisplay::new(game.width(), game.height()));
    app.add_module(PlayerDisplay);
    app.add_module(PieceDisplay::new());

    let name_area_height = game.players().len() as u16 + UI_OFFSET;
    let piece_area_height = game.height() - name_area_height + UI_OFFSET;

    let horizontal = Layout::horizontal([Constraint::Max((game.width() * 2) + UI_OFFSET), Constraint::Max(20)]);
    let vertical = Layout::vertical([Constraint::Max(name_area_height), Constraint::Max(piece_area_height)]);

    'main_loop: loop {
        terminal.draw(|frame| {
            let [board_area, side_menu_area] = horizontal.areas(frame.size());
            let [player_area, piece_area] = vertical.areas(side_menu_area);

            let areas = vec![
                (ModuleKind::Board, board_area),
                (ModuleKind::Player, player_area),
                (ModuleKind::Piece, piece_area)
            ].into_iter().collect::<HashMap<ModuleKind, Rect>>();
            app.render_modules(frame, game, areas)
        })?;

        event_queue.push_back(poll_event()?);
        while let Some(event) = event_queue.pop_front() {
            if let AppEvent::Quit = event { break 'main_loop }
            app.update_modules(event, game, &mut event_queue);
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
                    KeyCode::Char('i') => return Ok(AppEvent::OpenPieceSelection),
                    KeyCode::Enter => return Ok(AppEvent::Select),
                    KeyCode::Char('c') => return Ok(AppEvent::Rotate),
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

    fn update_modules(&mut self, event: AppEvent, game: &mut Game, event_queue: &mut VecDeque<AppEvent>) {
        for (_, module) in self.modules.iter_mut() {
            if let Some(event) = module.update(event, game) {
                event_queue.push_back(event);
            }
        }
    }

    fn render_modules(&mut self, frame: &mut Frame, game: &mut Game, areas: HashMap<ModuleKind, Rect>) {
        for (kind, module) in self.modules.iter_mut() {
            module.render(frame, *areas.get(kind).unwrap(), game)
        }
    }
}

impl Cursor {
    fn simple(max_x: u16, max_y: u16) -> Self {
        Cursor {
            max_x,
            max_y,
            area: Rect::new(0, 0, 1, 1)
        }
    }

    fn new(max_x: u16, max_y: u16, area: Rect) -> Self {
        Cursor {
            max_x,
            max_y,
            area
        }
    }

    fn move_down(&mut self, distance: u16) {
        if self.area.y <= self.max_y - self.area.height - distance {
            self.area.y += distance
        } else {
            self.area.y = self.max_y - self.area.height
        }
    }

    fn move_up(&mut self, distance: u16) {
        if self.area.y >= distance {
            self.area.y -= distance
        } else {
            self.area.y = 0
        }
    }

    fn move_right(&mut self, distance: u16) {
        if self.area.x <= self.max_x - self.area.width - distance {
            self.area.x += distance
        } else {
            self.area.x = self.max_x - self.area.width
        }
    }

    fn move_left(&mut self, distance: u16) {
        if self.area.x >= distance {
            self.area.x -= distance
        } else {
            self.area.x = 0
        }
    }

    fn move_cursor(&mut self, x: i32, y: i32) {
        if x < 0 { self.move_left(x.abs() as u16) } else { self.move_right(x as u16) }
        if y < 0 { self.move_up(y.abs() as u16) } else { self.move_down(y as u16) }
    }

    fn rotate_cursor(&mut self) {
        std::mem::swap(&mut self.area.width, &mut self.area.height);
    }
}