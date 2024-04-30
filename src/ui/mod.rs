use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use ratatui::widgets::canvas::{Canvas, Rectangle};

use crate::game::{Game, Position};

#[derive(Default)]
struct App {
    cursor: Cursor,
    board_offset: Position,
    vertical_scroll_state: ScrollbarState,
    horizontal_scroll_state: ScrollbarState
}

#[derive(Default)]
struct Cursor {
    position: Position,
    max_x: usize,
    max_y: usize
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
    let mut app = App::new(Cursor::new(game.width() * 2, game.height()));

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
                    KeyCode::Right => return Ok(AppEvent::MoveRight),
                    _ => ()
                }
            }
        }
    }
    Ok(AppEvent::None)
}

fn ui(frame: &mut Frame, game: &mut Game, app: &mut App) {
    let frame_size = frame.size();

    let display_width = (game.width() as u16) * 2;
    let display_height = game.height() as u16;

    let width = display_width.min(frame_size.width);
    let height = display_height.min(frame_size.height);
    render_board(frame, width, height, app);

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
                .viewport_content_length(width as usize)
                .content_length(display_width as usize)
        )
    }
}

fn render_board(frame: &mut Frame, width: u16, height: u16, app: &mut App) {
    let cursor_y = (app.board_offset.y + height as usize) as i16 - app.cursor.position.y as i16;
    if cursor_y < 0 {
        app.board_offset.y += cursor_y.abs() as usize;
    } else if cursor_y as u16 > height {
        app.board_offset.y -= (cursor_y as u16 - height) as usize;
    }

    frame.render_widget(
        Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Board"))
            .marker(Marker::Dot)
            .paint(|ctx| {
                for x in (0..width).step_by(2) {
                    for y in 0..height {
                        ctx.draw(&Rectangle {
                            x: x as f64 + 1.0,
                            y: y as f64,
                            width: 1.0,
                            height: 1.0,
                            color: Color::Black
                        })
                    }
                }

                ctx.draw(&Rectangle {
                    x: app.cursor.position.x as f64,
                    y: cursor_y as f64,
                    width: 1.0,
                    height: 1.0,
                    color: Color::Red
                });
            })
            .x_bounds([0.0, width as f64])
            .y_bounds([0.0, height as f64]),
        Rect { x: 0, y: 0, width, height }
    );
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
    fn new(max_x: usize, max_y: usize) -> Self {
        Cursor {
            max_x,
            max_y,
            ..Self::default()
        }
    }

    fn move_down(&mut self) {
        if self.position.y < self.max_y {
            self.position.y += 1
        }
    }

    fn move_up(&mut self) {
        if self.position.y > 1 {
            self.position.y -= 1
        }
    }

    fn move_right(&mut self) {
        if self.position.x < self.max_x {
            self.position.x += 1
        }
    }

    fn move_left(&mut self) {
        if self.position.x > 1 {
            self.position.x -= 1
        }
    }
}