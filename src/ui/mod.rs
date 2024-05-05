use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

use crate::game::{Board, Game, Position, State};

#[derive(Default)]
struct App {
    cursor: Cursor,
    vertical_scroll_state: ScrollbarState,
    horizontal_scroll_state: ScrollbarState
}

#[derive(Default)]
struct Cursor {
    position: Position,
    max_x: i32,
    max_y: i32
}

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
    let mut app = App::new(Cursor::new(game.width() as i32, game.height() as i32));

    loop {
        terminal.draw(|frame| ui(frame, game, &mut app))?;

        match handle_events()? {
            AppEvent::Quit => break,
            AppEvent::MoveUp => app.cursor.move_up(),
            AppEvent::MoveDown => app.cursor.move_down(),
            AppEvent::MoveLeft => app.cursor.move_left(),
            AppEvent::MoveRight => app.cursor.move_right(),
            AppEvent::None => ()
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<AppEvent> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(AppEvent::Quit),
                    KeyCode::Up => return Ok(AppEvent::MoveUp),
                    KeyCode::Down => return Ok(AppEvent::MoveDown),
                    KeyCode::Left => return Ok(AppEvent::MoveLeft),
                    KeyCode::Right => return Ok(AppEvent::MoveRight)
                    _ => ()
                }
            }
        }
    }
    Ok(AppEvent::None)
}

fn ui(frame: &mut Frame, game: &mut Game, app: &mut App) {
    let frame_size = frame.size();

    let display_width = ((game.width() as u16) * 2) + 2;
    let display_height = game.height() as u16 + 2;

    let width = display_width.min(frame_size.width);
    let height = display_height.min(frame_size.height);
    let board_render_area = Rect { x: 0, y: 0, width, height};
    render_board(frame, board_render_area, app, &game.board).expect("Error while board rendering");

    let remaining_width = display_width.saturating_sub(width);
    let remaining_height = display_height.saturating_sub(height);

    if remaining_width > 0 {
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::HorizontalBottom),
            frame_size,
            &mut app.horizontal_scroll_state
                .viewport_content_length(width as usize)
                .content_length(display_width as usize)
        )
    }
    if remaining_height > 0 {
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight),
            frame_size,
            &mut app.vertical_scroll_state
                .viewport_content_length(height as usize)
                .content_length(display_height as usize)
        );
    }
}

const BLOCK: &str = "██";

fn render_board(frame: &mut Frame, board_render_area: Rect, app: &mut App, board: &Board) -> Result<(), String> {
    let width = board.width as i32;
    let height = board.height as i32;

    let mut lines: Vec<Line<'_>> = vec![];
    let cursor_position = app.cursor.position;
    for y in 0..height {
        let mut line = vec![];
        for x in 0..width {
            if cursor_position.x == x && cursor_position.y == y {
                line.push(Span::styled(BLOCK, Style::default().fg(Color::Red)))
            }
            let color = match board.get_state_on_position(Position { x, y })? {
                State::Free => Color::Gray,
                State::Occupied(_) => Color::Blue
            };
            line.push(Span::styled(BLOCK, Style::default().fg(color)))
        }
        lines.push(line.into());
    }

    frame.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).padding(Padding::zero())),
        board_render_area
    );

    Ok(())
}

impl App {
    fn new(cursor: Cursor) -> Self {
        App {
            cursor,
            ..Self::default()
        }
    }
}

impl Cursor {
    fn new(max_x: i32, max_y: i32) -> Self {
        Cursor {
            max_x,
            max_y,
            ..Self::default()
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