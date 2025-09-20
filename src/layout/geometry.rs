//! Geometric types and utilities for layout.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a 2D size with width and height.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Size {
    /// Width in terminal cells
    pub width: u32,
    /// Height in terminal cells
    pub height: u32,
}

impl Size {
    /// Create a new size.
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Create a zero size.
    pub const fn zero() -> Self {
        Self::new(0, 0)
    }

    /// Create a size with equal width and height.
    pub const fn square(size: u32) -> Self {
        Self::new(size, size)
    }

    /// Get the area (width * height).
    pub fn area(&self) -> u64 {
        self.width as u64 * self.height as u64
    }

    /// Check if the size is zero.
    pub fn is_zero(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Scale the size by a factor.
    pub fn scale(&self, factor: f32) -> Self {
        Self::new(
            (self.width as f32 * factor) as u32,
            (self.height as f32 * factor) as u32,
        )
    }

    /// Clamp the size to fit within bounds.
    pub fn clamp(&self, min: Size, max: Size) -> Self {
        Self::new(
            self.width.clamp(min.width, max.width),
            self.height.clamp(min.height, max.height),
        )
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

/// Represents a 2D position with x and y coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    /// X coordinate (column)
    pub x: i32,
    /// Y coordinate (row)
    pub y: i32,
}

impl Position {
    /// Create a new position.
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Create a position at the origin (0, 0).
    pub const fn origin() -> Self {
        Self::new(0, 0)
    }

    /// Translate the position by an offset.
    pub fn translate(&self, dx: i32, dy: i32) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }

    /// Calculate the distance to another position.
    pub fn distance_to(&self, other: Position) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Represents a rectangular area with position and size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rect {
    /// Position of the top-left corner
    pub position: Position,
    /// Size of the rectangle
    pub size: Size,
}

impl Rect {
    /// Create a new rectangle.
    pub const fn new(position: Position, size: Size) -> Self {
        Self { position, size }
    }

    /// Create a rectangle from coordinates and dimensions.
    pub const fn from_coords(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self::new(Position::new(x, y), Size::new(width, height))
    }

    /// Create a rectangle at the origin with the given size.
    pub const fn from_size(size: Size) -> Self {
        Self::new(Position::origin(), size)
    }

    /// Get the x coordinate.
    pub fn x(&self) -> i32 {
        self.position.x
    }

    /// Get the y coordinate.
    pub fn y(&self) -> i32 {
        self.position.y
    }

    /// Get the width.
    pub fn width(&self) -> u32 {
        self.size.width
    }

    /// Get the height.
    pub fn height(&self) -> u32 {
        self.size.height
    }

    /// Get the right edge x coordinate.
    pub fn right(&self) -> i32 {
        self.position.x + self.size.width as i32
    }

    /// Get the bottom edge y coordinate.
    pub fn bottom(&self) -> i32 {
        self.position.y + self.size.height as i32
    }

    /// Get the center position.
    pub fn center(&self) -> Position {
        Position::new(
            self.position.x + self.size.width as i32 / 2,
            self.position.y + self.size.height as i32 / 2,
        )
    }

    /// Check if the rectangle contains a point.
    pub fn contains(&self, point: Position) -> bool {
        point.x >= self.position.x
            && point.x < self.right()
            && point.y >= self.position.y
            && point.y < self.bottom()
    }

    /// Check if this rectangle intersects with another.
    pub fn intersects(&self, other: &Rect) -> bool {
        self.position.x < other.right()
            && self.right() > other.position.x
            && self.position.y < other.bottom()
            && self.bottom() > other.position.y
    }

    /// Get the intersection of this rectangle with another.
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        if !self.intersects(other) {
            return None;
        }

        let left = self.position.x.max(other.position.x);
        let top = self.position.y.max(other.position.y);
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());

        Some(Rect::from_coords(
            left,
            top,
            (right - left) as u32,
            (bottom - top) as u32,
        ))
    }

    /// Get the union of this rectangle with another.
    pub fn union(&self, other: &Rect) -> Rect {
        let left = self.position.x.min(other.position.x);
        let top = self.position.y.min(other.position.y);
        let right = self.right().max(other.right());
        let bottom = self.bottom().max(other.bottom());

        Rect::from_coords(left, top, (right - left) as u32, (bottom - top) as u32)
    }

    /// Translate the rectangle by an offset.
    pub fn translate(&self, dx: i32, dy: i32) -> Self {
        Self::new(self.position.translate(dx, dy), self.size)
    }

    /// Resize the rectangle.
    pub fn resize(&self, new_size: Size) -> Self {
        Self::new(self.position, new_size)
    }

    /// Check if the rectangle is empty (zero area).
    pub fn is_empty(&self) -> bool {
        self.size.is_zero()
    }

    /// Get the area of the rectangle.
    pub fn area(&self) -> u64 {
        self.size.area()
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} @ {}", self.size, self.position)
    }
}

/// Represents spacing/padding around an element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Spacing {
    /// Top spacing
    pub top: u32,
    /// Right spacing
    pub right: u32,
    /// Bottom spacing
    pub bottom: u32,
    /// Left spacing
    pub left: u32,
}

impl Spacing {
    /// Create spacing with individual values.
    pub const fn new(top: u32, right: u32, bottom: u32, left: u32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Create uniform spacing on all sides.
    pub const fn all(value: u32) -> Self {
        Self::new(value, value, value, value)
    }

    /// Create spacing with different horizontal and vertical values.
    pub const fn symmetric(horizontal: u32, vertical: u32) -> Self {
        Self::new(vertical, horizontal, vertical, horizontal)
    }

    /// Create spacing with only horizontal values.
    pub const fn horizontal(value: u32) -> Self {
        Self::new(0, value, 0, value)
    }

    /// Create spacing with only vertical values.
    pub const fn vertical(value: u32) -> Self {
        Self::new(value, 0, value, 0)
    }

    /// Create zero spacing.
    pub const fn zero() -> Self {
        Self::all(0)
    }

    /// Get the total horizontal spacing.
    pub fn horizontal_total(&self) -> u32 {
        self.left + self.right
    }

    /// Get the total vertical spacing.
    pub fn vertical_total(&self) -> u32 {
        self.top + self.bottom
    }

    /// Get the total spacing as a size.
    pub fn total_size(&self) -> Size {
        Size::new(self.horizontal_total(), self.vertical_total())
    }
}

impl Default for Spacing {
    fn default() -> Self {
        Self::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let size = Size::new(10, 20);
        assert_eq!(size.width, 10);
        assert_eq!(size.height, 20);
        assert_eq!(size.area(), 200);
        assert!(!size.is_zero());

        let zero = Size::zero();
        assert!(zero.is_zero());
    }

    #[test]
    fn test_position() {
        let pos = Position::new(5, 10);
        assert_eq!(pos.x, 5);
        assert_eq!(pos.y, 10);

        let translated = pos.translate(2, 3);
        assert_eq!(translated.x, 7);
        assert_eq!(translated.y, 13);
    }

    #[test]
    fn test_rect() {
        let rect = Rect::from_coords(10, 20, 30, 40);
        assert_eq!(rect.x(), 10);
        assert_eq!(rect.y(), 20);
        assert_eq!(rect.width(), 30);
        assert_eq!(rect.height(), 40);
        assert_eq!(rect.right(), 40);
        assert_eq!(rect.bottom(), 60);

        let point_inside = Position::new(25, 35);
        let point_outside = Position::new(5, 15);
        assert!(rect.contains(point_inside));
        assert!(!rect.contains(point_outside));
    }

    #[test]
    fn test_rect_intersection() {
        let rect1 = Rect::from_coords(0, 0, 10, 10);
        let rect2 = Rect::from_coords(5, 5, 10, 10);

        assert!(rect1.intersects(&rect2));

        let intersection = rect1.intersection(&rect2).unwrap();
        assert_eq!(intersection, Rect::from_coords(5, 5, 5, 5));
    }

    #[test]
    fn test_spacing() {
        let spacing = Spacing::all(5);
        assert_eq!(spacing.horizontal_total(), 10);
        assert_eq!(spacing.vertical_total(), 10);

        let asymmetric = Spacing::new(1, 2, 3, 4);
        assert_eq!(asymmetric.horizontal_total(), 6);
        assert_eq!(asymmetric.vertical_total(), 4);
    }
}
