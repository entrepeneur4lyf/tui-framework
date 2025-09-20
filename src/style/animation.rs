//! CSS-like animations and transitions for TUI components.
//!
//! This module provides animation capabilities including keyframe animations,
//! transitions, and easing functions for smooth visual effects.

use crate::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Represents an animation that can be applied to style properties.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Animation {
    /// Name of the animation
    pub name: String,
    /// Duration of the animation
    pub duration: Duration,
    /// Timing function for the animation
    pub timing_function: TimingFunction,
    /// Number of times to repeat (None = infinite)
    pub iteration_count: Option<u32>,
    /// Direction of the animation
    pub direction: AnimationDirection,
    /// Fill mode for the animation
    pub fill_mode: AnimationFillMode,
    /// Delay before starting the animation
    pub delay: Duration,
    /// Whether the animation is currently playing
    pub play_state: AnimationPlayState,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            name: String::new(),
            duration: Duration::from_millis(1000),
            timing_function: TimingFunction::Ease,
            iteration_count: Some(1),
            direction: AnimationDirection::Normal,
            fill_mode: AnimationFillMode::None,
            delay: Duration::ZERO,
            play_state: AnimationPlayState::Running,
        }
    }
}

/// Timing functions for animations (easing).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TimingFunction {
    /// Linear progression
    Linear,
    /// Ease in and out
    Ease,
    /// Ease in (slow start)
    EaseIn,
    /// Ease out (slow end)
    EaseOut,
    /// Ease in and out (slow start and end)
    EaseInOut,
    /// Custom cubic bezier curve
    CubicBezier(f32, f32, f32, f32),
}

impl std::hash::Hash for TimingFunction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            TimingFunction::Linear => 0u8.hash(state),
            TimingFunction::Ease => 1u8.hash(state),
            TimingFunction::EaseIn => 2u8.hash(state),
            TimingFunction::EaseOut => 3u8.hash(state),
            TimingFunction::EaseInOut => 4u8.hash(state),
            TimingFunction::CubicBezier(x1, y1, x2, y2) => {
                5u8.hash(state);
                x1.to_bits().hash(state);
                y1.to_bits().hash(state);
                x2.to_bits().hash(state);
                y2.to_bits().hash(state);
            }
        }
    }
}

impl TimingFunction {
    /// Calculate the eased value for a given progress (0.0 to 1.0).
    pub fn ease(&self, progress: f32) -> f32 {
        match self {
            TimingFunction::Linear => progress,
            TimingFunction::Ease => {
                // Simplified ease function
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    -1.0 + (4.0 - 2.0 * progress) * progress
                }
            },
            TimingFunction::EaseIn => progress * progress,
            TimingFunction::EaseOut => progress * (2.0 - progress),
            TimingFunction::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    -1.0 + (4.0 - 2.0 * progress) * progress
                }
            },
            TimingFunction::CubicBezier(_x1, y1, _x2, y2) => {
                // Simplified cubic bezier - just interpolate between y1 and y2
                y1 + progress * (y2 - y1)
            },
        }
    }
}

/// Animation direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimationDirection {
    /// Normal direction (0% to 100%)
    Normal,
    /// Reverse direction (100% to 0%)
    Reverse,
    /// Alternate between normal and reverse
    Alternate,
    /// Alternate starting with reverse
    AlternateReverse,
}

/// Animation fill mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimationFillMode {
    /// No fill mode
    None,
    /// Apply first keyframe before animation starts
    Backwards,
    /// Apply last keyframe after animation ends
    Forwards,
    /// Apply both backwards and forwards
    Both,
}

/// Animation play state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimationPlayState {
    /// Animation is running
    Running,
    /// Animation is paused
    Paused,
}

/// A keyframe in an animation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Keyframe {
    /// Percentage of animation (0.0 to 1.0)
    pub offset: f32,
    /// Style properties at this keyframe
    pub properties: HashMap<String, AnimatableValue>,
}

/// Values that can be animated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimatableValue {
    /// Color value
    Color(Color),
    /// Numeric value (for dimensions, opacity, etc.)
    Number(f32),
    /// String value (for text content)
    String(String),
}

impl AnimatableValue {
    /// Interpolate between two animatable values.
    pub fn interpolate(&self, other: &Self, progress: f32) -> Self {
        match (self, other) {
            (AnimatableValue::Color(from), AnimatableValue::Color(to)) => {
                AnimatableValue::Color(interpolate_color(*from, *to, progress))
            }
            (AnimatableValue::Number(from), AnimatableValue::Number(to)) => {
                AnimatableValue::Number(from + (to - from) * progress)
            }
            (AnimatableValue::String(from), AnimatableValue::String(to)) => {
                // For strings, we can't interpolate smoothly, so we switch at 50%
                if progress < 0.5 {
                    AnimatableValue::String(from.clone())
                } else {
                    AnimatableValue::String(to.clone())
                }
            }
            _ => self.clone(), // Incompatible types, return original
        }
    }
}

/// A complete animation definition with keyframes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyframeAnimation {
    /// Animation metadata
    pub animation: Animation,
    /// Keyframes for the animation
    pub keyframes: Vec<Keyframe>,
}

impl KeyframeAnimation {
    /// Create a new keyframe animation.
    pub fn new(name: impl Into<String>, duration: Duration) -> Self {
        Self {
            animation: Animation {
                name: name.into(),
                duration,
                ..Default::default()
            },
            keyframes: Vec::new(),
        }
    }

    /// Add a keyframe to the animation.
    pub fn add_keyframe(mut self, offset: f32, properties: HashMap<String, AnimatableValue>) -> Self {
        self.keyframes.push(Keyframe { offset, properties });
        // Keep keyframes sorted by offset
        self.keyframes.sort_by(|a, b| a.offset.partial_cmp(&b.offset).unwrap());
        self
    }

    /// Get the interpolated properties at a given progress (0.0 to 1.0).
    pub fn get_properties_at(&self, progress: f32) -> HashMap<String, AnimatableValue> {
        if self.keyframes.is_empty() {
            return HashMap::new();
        }

        // Apply timing function
        let eased_progress = self.animation.timing_function.ease(progress);

        // Find the two keyframes to interpolate between
        let mut before_keyframe = &self.keyframes[0];
        let mut after_keyframe = &self.keyframes[self.keyframes.len() - 1];

        for keyframe in &self.keyframes {
            if keyframe.offset <= eased_progress {
                before_keyframe = keyframe;
            }
            if keyframe.offset >= eased_progress {
                after_keyframe = keyframe;
                break;
            }
        }

        // If we're exactly on a keyframe, return its properties
        if before_keyframe.offset == after_keyframe.offset {
            return before_keyframe.properties.clone();
        }

        // Interpolate between the two keyframes
        let local_progress = (eased_progress - before_keyframe.offset) 
            / (after_keyframe.offset - before_keyframe.offset);

        let mut result = HashMap::new();
        
        // Collect all property names from both keyframes
        let mut all_properties = std::collections::HashSet::new();
        for key in before_keyframe.properties.keys() {
            all_properties.insert(key.clone());
        }
        for key in after_keyframe.properties.keys() {
            all_properties.insert(key.clone());
        }

        // Interpolate each property
        for property in all_properties {
            if let (Some(from), Some(to)) = (
                before_keyframe.properties.get(&property),
                after_keyframe.properties.get(&property),
            ) {
                result.insert(property, from.interpolate(to, local_progress));
            } else if let Some(value) = before_keyframe.properties.get(&property) {
                result.insert(property, value.clone());
            } else if let Some(value) = after_keyframe.properties.get(&property) {
                result.insert(property, value.clone());
            }
        }

        result
    }
}

/// Animation state tracker for a component.
#[derive(Debug, Clone)]
pub struct AnimationState {
    /// Start time of the animation
    pub start_time: Instant,
    /// Current iteration
    pub current_iteration: u32,
    /// Whether the animation is currently in reverse
    pub is_reverse: bool,
    /// Current progress (0.0 to 1.0)
    pub progress: f32,
}

impl AnimationState {
    /// Create a new animation state.
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            current_iteration: 0,
            is_reverse: false,
            progress: 0.0,
        }
    }

    /// Update the animation state based on elapsed time.
    pub fn update(&mut self, animation: &Animation) -> bool {
        if animation.play_state == AnimationPlayState::Paused {
            return false;
        }

        let elapsed = self.start_time.elapsed();
        if elapsed < animation.delay {
            return false; // Still in delay phase
        }

        let animation_elapsed = elapsed - animation.delay;
        let cycle_progress = (animation_elapsed.as_secs_f32() / animation.duration.as_secs_f32()) % 1.0;
        
        // Handle direction
        self.progress = match animation.direction {
            AnimationDirection::Normal => cycle_progress,
            AnimationDirection::Reverse => 1.0 - cycle_progress,
            AnimationDirection::Alternate => {
                if self.current_iteration % 2 == 0 {
                    cycle_progress
                } else {
                    1.0 - cycle_progress
                }
            }
            AnimationDirection::AlternateReverse => {
                if self.current_iteration % 2 == 0 {
                    1.0 - cycle_progress
                } else {
                    cycle_progress
                }
            }
        };

        // Check if we've completed an iteration
        let total_cycles = animation_elapsed.as_secs_f32() / animation.duration.as_secs_f32();
        let new_iteration = total_cycles.floor() as u32;
        
        if new_iteration > self.current_iteration {
            self.current_iteration = new_iteration;
        }

        // Check if animation is complete
        if let Some(max_iterations) = animation.iteration_count {
            if self.current_iteration >= max_iterations {
                match animation.fill_mode {
                    AnimationFillMode::Forwards | AnimationFillMode::Both => {
                        self.progress = 1.0;
                    }
                    _ => {
                        self.progress = 0.0;
                    }
                }
                return false; // Animation complete
            }
        }

        true // Animation still running
    }
}

/// Cubic bezier easing function implementation.
#[allow(dead_code)]
fn cubic_bezier(t: f32, _x1: f32, y1: f32, _x2: f32, y2: f32) -> f32 {
    // Simplified cubic bezier calculation
    // For a more accurate implementation, you'd use Newton-Raphson method
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let mt3 = mt2 * mt;
    
    mt3 * 0.0 + 3.0 * mt2 * t * y1 + 3.0 * mt * t2 * y2 + t3 * 1.0
}

/// Interpolate between two colors.
fn interpolate_color(from: Color, to: Color, progress: f32) -> Color {
    Color::rgba(
        (from.r as f32 + (to.r as f32 - from.r as f32) * progress) as u8,
        (from.g as f32 + (to.g as f32 - from.g as f32) * progress) as u8,
        (from.b as f32 + (to.b as f32 - from.b as f32) * progress) as u8,
        (from.a as f32 + (to.a as f32 - from.a as f32) * progress) as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timing_functions() {
        assert_eq!(TimingFunction::Linear.ease(0.5), 0.5);
        // Note: These are simplified implementations, so we just test they're different
        assert_ne!(TimingFunction::EaseIn.ease(0.5), 0.5);
        assert_ne!(TimingFunction::EaseOut.ease(0.5), 0.5);
    }

    #[test]
    fn test_animatable_value_interpolation() {
        let from = AnimatableValue::Number(0.0);
        let to = AnimatableValue::Number(100.0);
        
        if let AnimatableValue::Number(result) = from.interpolate(&to, 0.5) {
            assert_eq!(result, 50.0);
        } else {
            panic!("Expected Number variant");
        }
    }

    #[test]
    fn test_keyframe_animation() {
        let mut animation = KeyframeAnimation::new("test", Duration::from_secs(1));
        
        let mut start_props = HashMap::new();
        start_props.insert("opacity".to_string(), AnimatableValue::Number(0.0));
        animation = animation.add_keyframe(0.0, start_props);
        
        let mut end_props = HashMap::new();
        end_props.insert("opacity".to_string(), AnimatableValue::Number(1.0));
        animation = animation.add_keyframe(1.0, end_props);
        
        let props = animation.get_properties_at(0.5);
        if let Some(AnimatableValue::Number(opacity)) = props.get("opacity") {
            assert!((opacity - 0.5).abs() < 0.1); // Allow for easing
        } else {
            panic!("Expected opacity property");
        }
    }

    #[test]
    fn test_color_interpolation() {
        let from = Color::rgb(0, 0, 0);
        let to = Color::rgb(255, 255, 255);
        let result = interpolate_color(from, to, 0.5);

        assert_eq!(result.r, 127);
        assert_eq!(result.g, 127);
        assert_eq!(result.b, 127);
    }
}
