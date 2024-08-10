#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub(crate) enum Side {
    Right,
    Left,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    pub(crate) fn turn(self, side: Side) -> Direction {
        match side {
            Side::Right => self.right(),
            Side::Left => self.left(),
        }
    }

    pub(crate) fn right(self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }

    pub(crate) fn left(self) -> Direction {
        match self {
            Direction::N => Direction::W,
            Direction::E => Direction::N,
            Direction::S => Direction::E,
            Direction::W => Direction::S,
        }
    }

    pub(crate) fn random() -> Direction {
        use rand::Rng;
        match rand::thread_rng().gen_range(0..4) {
            0 => Direction::N,
            1 => Direction::E,
            2 => Direction::S,
            _ => Direction::W,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct Position {
    x: i32,
    y: i32,
}

impl Position {
    #[cfg(test)]
    pub(crate) fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub(crate) fn random(width: u32, height: u32) -> Position {
        use rand::Rng;
        Position {
            x: rand::thread_rng().gen_range(0..width as i32),
            y: rand::thread_rng().gen_range(0..height as i32),
        }
    }

    /// 向いている方向に対し横(side)方向に移動する
    pub(crate) fn walk(&self, direction: Direction, side: Side) -> Position {
        match side {
            Side::Right => self.move_right(direction),
            Side::Left => self.move_left(direction),
        }
    }

    /// 向いている方向に進む
    pub(crate) fn forward(&self, direction: Direction) -> Position {
        match direction {
            Direction::N => Position {
                y: self.y - 1,
                ..*self
            },
            Direction::E => Position {
                x: self.x + 1,
                ..*self
            },
            Direction::S => Position {
                y: self.y + 1,
                ..*self
            },
            Direction::W => Position {
                x: self.x - 1,
                ..*self
            },
        }
    }

    pub(crate) fn move_right(&self, direction: Direction) -> Position {
        match direction {
            Direction::N => Position {
                x: self.x + 1,
                ..*self
            },
            Direction::E => Position {
                y: self.y + 1,
                ..*self
            },
            Direction::S => Position {
                x: self.x - 1,
                ..*self
            },
            Direction::W => Position {
                y: self.y - 1,
                ..*self
            },
        }
    }

    pub(crate) fn move_left(&self, direction: Direction) -> Position {
        match direction {
            Direction::N => Position {
                x: self.x - 1,
                ..*self
            },
            Direction::E => Position {
                y: self.y - 1,
                ..*self
            },
            Direction::S => Position {
                x: self.x + 1,
                ..*self
            },
            Direction::W => Position {
                y: self.y + 1,
                ..*self
            },
        }
    }

    pub(crate) fn is_inset(&self, width: i32, height: i32) -> bool {
        self.x >= 0 && self.x < width && self.y >= 0 && self.y < height
    }
}
