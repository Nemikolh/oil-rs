
/// Rect is mainly used by
/// layout and focus trees.
///
#[derive(Copy, Clone, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {

    /// This function computes the area intersection
    /// between the two rectangles. It returns the
    /// intersection area value.
    // pub fn intersects(&self, other: &Rect) -> f32 {
    //     self.intersects_x(other) * self.intersects_y(other)
    // }

    pub fn intersects_x(&self, other: &Rect) -> f32 {
        if self.x < other.x {
            if self.x + self.width > other.x {
                self.x + self.width - other.x
            } else {
                0f32
            }
        } else {
            if self.x > other.x + other.width {
                0f32
            } else {
                other.x + other.width - self.x
            }
        }
    }

    pub fn intersects_y(&self, other: &Rect) -> f32 {
        if self.y < other.y {
            if self.y + self.height > other.y {
                self.y + self.height - other.y
            } else {
                0f32
            }
        } else {
            if self.y > other.y + other.height {
                0f32
            } else {
                other.y + other.height - self.y
            }
        }
    }
}
