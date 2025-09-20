//! Flexbox layout implementation.

use serde::{Deserialize, Serialize};

/// Flex direction values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_direction_default() {
        assert_eq!(FlexDirection::default(), FlexDirection::Row);
    }

    #[test]
    fn test_flex_direction_variants() {
        // Test all variants exist and are distinct
        let variants = [
            FlexDirection::Row,
            FlexDirection::Column,
            FlexDirection::RowReverse,
            FlexDirection::ColumnReverse,
        ];

        // Each variant should be equal to itself
        for variant in &variants {
            assert_eq!(*variant, *variant);
        }

        // Each variant should be different from others
        assert_ne!(FlexDirection::Row, FlexDirection::Column);
        assert_ne!(FlexDirection::Row, FlexDirection::RowReverse);
        assert_ne!(FlexDirection::Row, FlexDirection::ColumnReverse);
        assert_ne!(FlexDirection::Column, FlexDirection::RowReverse);
        assert_ne!(FlexDirection::Column, FlexDirection::ColumnReverse);
        assert_ne!(FlexDirection::RowReverse, FlexDirection::ColumnReverse);
    }

    #[test]
    fn test_flex_direction_serialization() {
        // Test that FlexDirection can be serialized and deserialized
        let direction = FlexDirection::Column;
        let serialized = serde_json::to_string(&direction).unwrap();
        let deserialized: FlexDirection = serde_json::from_str(&serialized).unwrap();
        assert_eq!(direction, deserialized);
    }

    #[test]
    fn test_justify_content_default() {
        assert_eq!(JustifyContent::default(), JustifyContent::FlexStart);
    }

    #[test]
    fn test_justify_content_variants() {
        // Test all variants exist and are distinct
        let variants = [
            JustifyContent::FlexStart,
            JustifyContent::FlexEnd,
            JustifyContent::Center,
            JustifyContent::SpaceBetween,
            JustifyContent::SpaceAround,
            JustifyContent::SpaceEvenly,
        ];

        // Each variant should be equal to itself
        for variant in &variants {
            assert_eq!(*variant, *variant);
        }

        // Test some key differences
        assert_ne!(JustifyContent::FlexStart, JustifyContent::FlexEnd);
        assert_ne!(JustifyContent::Center, JustifyContent::SpaceBetween);
        assert_ne!(JustifyContent::SpaceAround, JustifyContent::SpaceEvenly);
    }

    #[test]
    fn test_justify_content_serialization() {
        // Test that JustifyContent can be serialized and deserialized
        let justify = JustifyContent::Center;
        let serialized = serde_json::to_string(&justify).unwrap();
        let deserialized: JustifyContent = serde_json::from_str(&serialized).unwrap();
        assert_eq!(justify, deserialized);
    }

    #[test]
    fn test_align_items_default() {
        assert_eq!(AlignItems::default(), AlignItems::Stretch);
    }

    #[test]
    fn test_align_items_variants() {
        // Test all variants exist and are distinct
        let variants = [
            AlignItems::FlexStart,
            AlignItems::FlexEnd,
            AlignItems::Center,
            AlignItems::Stretch,
            AlignItems::Baseline,
        ];

        // Each variant should be equal to itself
        for variant in &variants {
            assert_eq!(*variant, *variant);
        }

        // Test some key differences
        assert_ne!(AlignItems::FlexStart, AlignItems::FlexEnd);
        assert_ne!(AlignItems::Center, AlignItems::Stretch);
        assert_ne!(AlignItems::Baseline, AlignItems::Center);
    }

    #[test]
    fn test_align_items_serialization() {
        // Test that AlignItems can be serialized and deserialized
        let align = AlignItems::Center;
        let serialized = serde_json::to_string(&align).unwrap();
        let deserialized: AlignItems = serde_json::from_str(&serialized).unwrap();
        assert_eq!(align, deserialized);
    }

    #[test]
    fn test_all_enums_debug() {
        // Test that all enums implement Debug properly
        let flex_dir = FlexDirection::Row;
        let justify = JustifyContent::Center;
        let align = AlignItems::Stretch;

        // Should not panic and should produce reasonable output
        let debug_output = format!("{:?} {:?} {:?}", flex_dir, justify, align);
        assert!(debug_output.contains("Row"));
        assert!(debug_output.contains("Center"));
        assert!(debug_output.contains("Stretch"));
    }

    #[test]
    fn test_all_enums_clone() {
        // Test that all enums can be cloned
        let flex_dir = FlexDirection::Column;
        let justify = JustifyContent::SpaceBetween;
        let align = AlignItems::FlexEnd;

        let cloned_flex = flex_dir.clone();
        let cloned_justify = justify.clone();
        let cloned_align = align.clone();

        assert_eq!(flex_dir, cloned_flex);
        assert_eq!(justify, cloned_justify);
        assert_eq!(align, cloned_align);
    }

    #[test]
    fn test_enum_copy_semantics() {
        // Test that all enums implement Copy
        let flex_dir = FlexDirection::RowReverse;
        let justify = JustifyContent::SpaceEvenly;
        let align = AlignItems::Baseline;

        // These should work with copy semantics
        let copied_flex = flex_dir;
        let copied_justify = justify;
        let copied_align = align;

        // Original values should still be usable
        assert_eq!(flex_dir, copied_flex);
        assert_eq!(justify, copied_justify);
        assert_eq!(align, copied_align);
    }
}
