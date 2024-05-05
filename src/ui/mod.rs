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
    board_offset: Position,
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
                    KeyCode::Right => return Ok(AppEvent::MoveRight),
                    _ => ()
                }
            }
        }
    }
    Ok(AppEvent::None)
}

const UI_OFFSET: u16 = 2;

fn ui(frame: &mut Frame, game: &mut Game, app: &mut App) {
    let frame_size = frame.size();

    let display_width = ((game.width() as u16) * 2) + UI_OFFSET;
    let display_height = game.height() as u16 + UI_OFFSET;

    let width = display_width.min(frame_size.width);
    let height = display_height.min(frame_size.height);
    let board_render_area = Rect { x: 0, y: 0, width, height};

    update_scrollbars(board_render_area, app);
    render_scrollbars(frame, &game, &app, frame_size, display_width, display_height, width, height);

    render_board(frame, board_render_area, app, &game.board).expect("Error while board rendering");

}

fn render_scrollbars(frame: &mut Frame, game: &&mut Game, app: &&mut App, frame_size: Rect, display_width: u16, display_height: u16, width: u16, height: u16) {
    let remaining_width = display_width.saturating_sub(width);
    let remaining_height = display_height.saturating_sub(height);

    if remaining_width > 0 {
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::HorizontalBottom),
            frame_size,
            &mut app.horizontal_scroll_state
                .viewport_content_length(game.width() - remaining_width as usize)
                .content_length(game.width())
        )
    }
    if remaining_height > 0 {
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight),
            frame_size,
            &mut app.vertical_scroll_state
                .viewport_content_length(game.height() - remaining_height as usize)
                .content_length(game.height())
        );
    }
}

const BLOCK: &str = "██";

fn render_board(frame: &mut Frame, board_render_area: Rect, app: &mut App, board: &Board) -> Result<(), String> {
    let width = board.width as i32;
    let height = board.height as i32;

    let cursor_position = app.cursor.position;
    let mut lines: Vec<Line<'_>> = vec![];
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
        Paragraph::new(lines).scroll((app.board_offset.y as u16, app.board_offset.x as u16)).block(Block::default().borders(Borders::ALL).padding(Padding::zero())),
        board_render_area
    );

    Ok(())
}

fn update_scrollbars(board_render_area: Rect, app: &mut App) {
    let rows_shown = board_render_area.height - UI_OFFSET;
    let columns_shown = board_render_area.width - UI_OFFSET;

    let cursor_position = app.cursor.position;

    let relative_cursor_y = cursor_position.y - app.board_offset.y;
    if relative_cursor_y as u16 >= rows_shown {
        app.vertical_scroll_state = app.vertical_scroll_state.position(cursor_position.y as usize);
        app.board_offset.y += 1;
    }
    if relative_cursor_y < 0 {
        app.vertical_scroll_state = app.vertical_scroll_state.position(cursor_position.y as usize);
        app.board_offset.y -= 2;
    }

    let relative_cursor_x = cursor_position.x - app.board_offset.x;
    if relative_cursor_x as u16 >= columns_shown {
        app.vertical_scroll_state = app.vertical_scroll_state.position(cursor_position.x as usize);
        app.board_offset.x += 1;
    }
    if relative_cursor_x < 0 {
        app.vertical_scroll_state = app.vertical_scroll_state.position(cursor_position.x as usize);
        app.board_offset.x -= 2;
    }
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