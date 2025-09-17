//! Flexbox layout implementation.

use serde::{Deserialize, Serialize};

/// Flex direction values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum FlexDirection {
    /// Horizontal layout, left to right
    #[default]
    Row,
    /// Vertical layout, top to bottom
    Column,
    /// Horizontal layout, right to left
    RowReverse,
    /// Vertical layout, bottom to top
    ColumnReverse,
}


/// Justify content values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum JustifyContent {
    /// Items are packed toward the start of the flex direction
    #[default]
    FlexStart,
    /// Items are packed toward the end of the flex direction
    FlexEnd,
    /// Items are centered along the main axis
    Center,
    /// Items are evenly distributed with space between them
    SpaceBetween,
    /// Items are evenly distributed with space around them
    SpaceAround,
    /// Items are evenly distributed with equal space around them
    SpaceEvenly,
}


/// Align items values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum AlignItems {
    /// Items are aligned to the start of the cross axis
    FlexStart,
    /// Items are aligned to the end of the cross axis
    FlexEnd,
    /// Items are centered on the cross axis
    Center,
    /// Items are stretched to fill the cross axis
    #[default]
    Stretch,
    /// Items are aligned to their baseline
    Baseline,
}

