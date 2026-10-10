//! Fast geometric collision detection routines.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

impl Circle {
    #[inline]
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        Self { x, y, radius }
    }

    #[inline]
    pub fn intersects(&self, other: &Circle) -> bool {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance_sq = dx * dx + dy * dy;
        let radius_sum = self.radius + other.radius;
        distance_sq <= radius_sum * radius_sum
    }

    #[inline]
    pub fn contains_point(&self, px: f32, py: f32) -> bool {
        let dx = self.x - px;
        let dy = self.y - py;
        dx * dx + dy * dy <= self.radius * self.radius
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    #[inline]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    #[inline]
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    #[inline]
    pub fn intersects_circle(&self, circle: &Circle) -> bool {
        let closest_x = circle.x.clamp(self.x, self.x + self.width);
        let closest_y = circle.y.clamp(self.y, self.y + self.height);
        let dx = circle.x - closest_x;
        let dy = circle.y - closest_y;
        dx * dx + dy * dy <= circle.radius * circle.radius
    }
}
