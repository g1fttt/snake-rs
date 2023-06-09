use crate::{
    snake::{Direction, Snake},
    timer::Timer,
};

use std::{
    io::{self, Error, ErrorKind, Stdout},
    time::Duration,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{self, Stylize},
    terminal, ExecutableCommand,
};
use rand::Rng;

#[derive(Clone)]
pub struct Vec2 {
    pub x: u16,
    pub y: u16,
}

impl Vec2 {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

pub struct Game {
    snake: Snake,
    board_size: Vec2,
    fruit_position: Vec2,
    canvas: Stdout,
}

impl Game {
    pub fn new(board_size: Vec2) -> crossterm::Result<Self> {
        let mut canvas = io::stdout();

        canvas.execute(cursor::Hide)?;
        terminal::enable_raw_mode()?;

        Ok(Self {
            snake: Snake::new(),
            board_size: board_size.clone(),
            fruit_position: Vec2::new((board_size.x / 2) as u16, (board_size.y / 2) as u16),
            canvas,
        })
    }

    pub fn init_loop(&mut self) -> crossterm::Result<()> {
        self.clear_terminal()?;

        let mut timer = Timer::new();

        loop {
            timer.tick();

            if let Err(err) = self.poll_input() {
                self.clear_terminal()?;
                println!("{}", err);
                break;
            }

            if let Some(delta) = timer.delta() {
                if delta >= Duration::from_secs_f32(1.0 / 10.0) {
                    timer.reset();

                    if let Err(err) = self.update() {
                        self.clear_terminal()?;
                        println!("{}", err);
                        break;
                    }
                }
            }

            self.draw()?;
        }

        Ok(())
    }

    pub fn update(&mut self) -> Result<(), String> {
        if self.snake.ate_itself() {
            return Err("Snake ate itself".to_string());
        } else if self.snake.ate_an_fruit(&self.fruit_position) {
            self.snake.add_segment();
            self.respawn_fruit();
        }

        self.snake.update(&self.board_size)
    }

    pub fn draw(&mut self) -> crossterm::Result<()> {
        self.draw_board_floor()?;
        self.draw_fruit()?;
        self.draw_score()?;
        self.snake.draw(&mut self.canvas)
    }

    fn poll_input(&mut self) -> crossterm::Result<()> {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(event) = event::read()? {
                match event.code {
                    KeyCode::Esc => {
                        return Err(Error::new(ErrorKind::Interrupted, "Esc was pressed"))
                    }
                    KeyCode::Up => self.snake.set_direction(Direction::Up),
                    KeyCode::Left => self.snake.set_direction(Direction::Left),
                    KeyCode::Down => self.snake.set_direction(Direction::Down),
                    KeyCode::Right => self.snake.set_direction(Direction::Right),
                    _ => (),
                }
            }
        }
        Ok(())
    }

    fn clear_terminal(&mut self) -> crossterm::Result<()> {
        execute!(
            self.canvas,
            cursor::MoveTo(0, 0),
            terminal::Clear(terminal::ClearType::All)
        )
    }

    fn draw_score(&mut self) -> crossterm::Result<()> {
        execute!(
            self.canvas,
            cursor::MoveTo(self.board_size.x * 2 + 1, 0),
            style::PrintStyledContent(format!("Score: {}", self.snake.fruits_eaten()).cyan())
        )
    }

    fn draw_board_floor(&mut self) -> crossterm::Result<()> {
        for y in 0..self.board_size.y {
            for x in 0..self.board_size.x {
                if (x == self.fruit_position.x && y == self.fruit_position.y)
                    || self.snake.has_segment_at(&Vec2::new(x, y))
                {
                    continue;
                }
                execute!(
                    self.canvas,
                    cursor::MoveTo(x * 2 + 1, y),
                    style::PrintStyledContent('.'.dark_grey())
                )?;
            }
        }
        Ok(())
    }

    fn draw_fruit(&mut self) -> crossterm::Result<()> {
        execute!(
            self.canvas,
            cursor::MoveTo(self.fruit_position.x * 2 + 1, self.fruit_position.y),
            style::PrintStyledContent('F'.red().italic())
        )
    }

    fn respawn_fruit(&mut self) {
        let mut rng = rand::thread_rng();
        while self.snake.has_segment_at(&self.fruit_position) {
            self.fruit_position.x = rng.gen_range(0..self.board_size.x);
            self.fruit_position.y = rng.gen_range(0..self.board_size.y);
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
        self.canvas.execute(cursor::Show).unwrap();
    }
}
