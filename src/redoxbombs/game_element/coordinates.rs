use std::ops::Sub;

/// A `Coordinate` represents a point in a 2D space.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Coordinates {
    /// Represents the position in the horizontal axis (X-Axis).
    pub x: usize,

    /// Represents the position in the vertical axis (Y-Axis).
    pub y: usize,
}

impl Coordinates {
    /// Compute the eclidean distance between two `Coordinates`.
    /// Formula: sqrt( (x1 - x2)^2 + (y1 - y2)^2 )
    pub fn euclidean_distance(&self, other: &Coordinates) -> f64 {
        let dx = abs_sub(self.x, other.x);
        let dy = abs_sub(self.y, other.y);

        let square_sum = dx.pow(2) + dy.pow(2);

        (square_sum as f64).sqrt()
    }

    /// Compute the manhattan distance between two `Coordinates`.
    /// Formula: abs(x1 - x2) + abs(y1 - y2)
    pub fn manhattan_distance(&self, other: &Coordinates) -> usize {
        let dx = abs_sub(self.x, other.x);
        let dy = abs_sub(self.y, other.y);

        dx + dy
    }

    /// Get the next coordinate closer to `target` in both x and y
    /// axis.
    fn target_to(&self, target: &Coordinates) -> Coordinates {
        let x = Coordinates::next_to(self.x, target.x);
        let y = Coordinates::next_to(self.y, target.y);

        Coordinates { x, y }
    }

    /// Get the next coordinate closer to `target` in the y axis.
    fn target_x_to(&self, other: &Coordinates) -> Coordinates {
        let x = Coordinates::next_to(self.x, other.x);

        Coordinates { x, y: self.y }
    }

    /// Get the next coordinate closer to `target` in the y axis.
    fn target_y_to(&self, target: &Coordinates) -> Coordinates {
        let y = Coordinates::next_to(self.y, target.y);

        Coordinates { x: self.x, y }
    }

    /// Compute the next coordinet to get closer to target.
    fn next_to(current: usize, target: usize) -> usize {
        if current > target {
            current - 1
        } else if current < target {
            current + 1
        } else {
            current
        }
    }

    /// Compute the `Coordinate` above the current one.
    pub fn up(&self) -> Coordinates {
        Coordinates {
            x: self.x,
            y: self.y - 1,
        }
    }

    /// Compute the `Coordinate` below the current one.
    pub fn down(&self) -> Coordinates {
        Coordinates {
            x: self.x,
            y: self.y + 1,
        }
    }

    /// Compute the `Coordinate` at the left of the current one.
    pub fn left(&self) -> Coordinates {
        Coordinates {
            x: self.x - 1,
            y: self.y,
        }
    }

    /// Compute the `Coordinate` at the right of the current one.
    pub fn right(&self) -> Coordinates {
        Coordinates {
            x: self.x + 1,
            y: self.y,
        }
    }
}

/// Compute the absolute difference (with no negative values) between
/// x and y.
fn abs_sub<T>(x: T, y: T) -> <T as std::ops::Sub>::Output
where
    T: Sub + PartialOrd,
{
    if x > y {
        x - y
    } else {
        y - x
    }
}
