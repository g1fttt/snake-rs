use crate::game::Vec2;

use std::io::Stdout;

use crossterm::{
    cursor, execute,
    style::{self, Stylize},
};

#[derive(PartialEq)]
enum SegmentKind {
    Head,
    Body,
}

#[derive(PartialEq)]
struct Segment {
    x: u16,
    y: u16,
    kind: SegmentKind,
}

impl Segment {
    pub fn new(x: u16, y: u16, kind: SegmentKind) -> Self {
        Self { x, y, kind }
    }
}

// Public enum for Input System
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub struct Snake {
    segments: Vec<Segment>,
    direction: Direction,
    new_segment_added: bool,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            segments: vec![
                Segment::new(2, 0, SegmentKind::Head),
                Segment::new(1, 0, SegmentKind::Body),
                Segment::new(0, 0, SegmentKind::Body),
            ],
            direction: Direction::Right,
            new_segment_added: false,
        }
    }

    pub fn update(&mut self) {
        let positions: Vec<(u16, u16)> = self
            .segments
            .iter()
            .rev()
            .map(|segment| (segment.x, segment.y))
            .collect();

        let iter = self.segments.iter_mut().rev();
        for (i, segment) in iter.enumerate() {
            if self.new_segment_added {
                self.new_segment_added = false;
            } else {
                if i < positions.len() - 1 {
                    let (x, y) = positions[i + 1];
                    segment.x = x;
                    segment.y = y;
                }
            }
        }

        if let Some(head) = self.segments.first_mut() {
            match self.direction {
                Direction::Right => head.x += 1,
                Direction::Left => head.x -= 1,
                Direction::Down => head.y += 1,
                Direction::Up => head.y -= 1,
            }
        }
    }

    pub fn draw(&self, canvas: &mut Stdout) -> crossterm::Result<()> {
        for segment in self.segments.iter() {
            execute!(
                canvas,
                cursor::MoveTo(segment.x * 2 + 1, segment.y),
                style::PrintStyledContent(
                    match segment.kind {
                        SegmentKind::Head => 'O',
                        SegmentKind::Body => 'o',
                    }
                    .white()
                )
            )?;
        }
        Ok(())
    }

    pub fn add_segment(&mut self) {
        if let Some(tail) = self.segments.last() {
            self.segments
                .push(Segment::new(tail.x, tail.y, SegmentKind::Body));
            self.new_segment_added = true;
        }
    }

    pub fn ate_an_fruit(&self, fruit_position: &Vec2) -> bool {
        if let Some(head) = self.segments.first() {
            head.x == fruit_position.x && head.y == fruit_position.y
        } else {
            false
        }
    }

    pub fn ate_itself(&self) -> bool {
        if let Some(head) = self.segments.first() {
            self.segments.iter().any(|segment| {
                head.x == segment.x && head.y == segment.y && segment.kind == SegmentKind::Body
            })
        } else {
            false
        }
    }

    pub fn hit_the_edge(&self, board_size: &Vec2) -> bool {
        if let Some(head) = self.segments.first() {
            head.x == board_size.x - 1 || head.y == board_size.y - 1
        } else {
            false
        }
    }

    pub fn has_segment_at(&self, position: &Vec2) -> bool {
        self.segments
            .iter()
            .any(|segment| segment.x == position.x && segment.y == position.y)
    }

    pub fn set_direction(&mut self, direction: Direction) {
        // TODO: Check for reversed direction
        self.direction = direction;
    }
}