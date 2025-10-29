// Animation system for smooth, beautiful transitions

use crate::types::*;
use crate::card::MediaCard;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Animation system manages all active animations
#[derive(Debug)]
pub struct AnimationSystem {
    animations: Vec<Animation>,
    next_id: u64,
}

impl AnimationSystem {
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            next_id: 0,
        }
    }

    /// Add a new animation
    pub fn add(&mut self, animation: Animation) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let mut anim = animation;
        anim.id = id;
        self.animations.push(anim);

        id
    }

    /// Remove an animation
    pub fn remove(&mut self, id: u64) {
        self.animations.retain(|a| a.id != id);
    }

    /// Update all animations
    pub fn update(&mut self, delta_time: Duration, cards: &mut HashMap<u32, MediaCard>) {
        let now = Instant::now();

        // Update animations
        for animation in &mut self.animations {
            if animation.is_active(now) {
                let progress = animation.progress(now);
                animation.apply(progress, cards);
            }
        }

        // Remove finished animations
        self.animations.retain(|a| !a.is_finished(now));
    }

    /// Animate gallery entrance (staggered card appearance)
    pub fn animate_gallery_entrance(&mut self, cards: &[&MediaCard]) {
        let stagger_delay = Duration::from_millis(50);

        for (i, card) in cards.iter().enumerate() {
            let delay = stagger_delay * i as u32;

            // Fade in
            self.add(Animation {
                id: 0,
                target: AnimationTarget::Card(card.id),
                property: AnimationProperty::Opacity,
                from: 0.0,
                to: 1.0,
                duration: Duration::from_millis(300),
                easing: EasingFunction::EaseOut,
                start_time: Instant::now() + delay,
                ..Default::default()
            });

            // Slide up
            self.add(Animation {
                id: 0,
                target: AnimationTarget::Card(card.id),
                property: AnimationProperty::TranslateY,
                from: 20.0,
                to: 0.0,
                duration: Duration::from_millis(400),
                easing: EasingFunction::Spring,
                start_time: Instant::now() + delay,
                ..Default::default()
            });
        }
    }

    /// Animate card selection
    pub fn animate_select(&mut self, card_id: u32) {
        self.add(Animation {
            id: 0,
            target: AnimationTarget::Card(card_id),
            property: AnimationProperty::Scale,
            from: 1.0,
            to: 1.05,
            duration: Duration::from_millis(200),
            easing: EasingFunction::EaseOut,
            start_time: Instant::now(),
            ..Default::default()
        });
    }

    /// Animate card deselection
    pub fn animate_deselect(&mut self, card_id: u32) {
        self.add(Animation {
            id: 0,
            target: AnimationTarget::Card(card_id),
            property: AnimationProperty::Scale,
            from: 1.05,
            to: 1.0,
            duration: Duration::from_millis(200),
            easing: EasingFunction::EaseIn,
            start_time: Instant::now(),
            ..Default::default()
        });
    }

    /// Animate zoom in (fullscreen)
    pub fn animate_zoom_in(&mut self, card_id: u32, from_bounds: Rect, to_bounds: Rect) {
        let duration = Duration::from_millis(300);

        // Position
        self.add(Animation {
            id: 0,
            target: AnimationTarget::Card(card_id),
            property: AnimationProperty::PositionX,
            from: from_bounds.x,
            to: to_bounds.x,
            duration,
            easing: EasingFunction::EaseInOut,
            start_time: Instant::now(),
            ..Default::default()
        });

        self.add(Animation {
            id: 0,
            target: AnimationTarget::Card(card_id),
            property: AnimationProperty::PositionY,
            from: from_bounds.y,
            to: to_bounds.y,
            duration,
            easing: EasingFunction::EaseInOut,
            start_time: Instant::now(),
            ..Default::default()
        });

        // Size
        self.add(Animation {
            id: 0,
            target: AnimationTarget::Card(card_id),
            property: AnimationProperty::Width,
            from: from_bounds.width,
            to: to_bounds.width,
            duration,
            easing: EasingFunction::EaseInOut,
            start_time: Instant::now(),
            ..Default::default()
        });

        self.add(Animation {
            id: 0,
            target: AnimationTarget::Card(card_id),
            property: AnimationProperty::Height,
            from: from_bounds.height,
            to: to_bounds.height,
            duration,
            easing: EasingFunction::EaseInOut,
            start_time: Instant::now(),
            ..Default::default()
        });
    }
}

/// Animation definition
#[derive(Debug, Clone)]
pub struct Animation {
    pub id: u64,
    pub target: AnimationTarget,
    pub property: AnimationProperty,
    pub from: f32,
    pub to: f32,
    pub duration: Duration,
    pub easing: EasingFunction,
    pub start_time: Instant,
    pub repeat: bool,
    pub reverse: bool,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            id: 0,
            target: AnimationTarget::Card(0),
            property: AnimationProperty::Opacity,
            from: 0.0,
            to: 1.0,
            duration: Duration::from_millis(300),
            easing: EasingFunction::EaseInOut,
            start_time: Instant::now(),
            repeat: false,
            reverse: false,
        }
    }
}

impl Animation {
    /// Check if animation is currently active
    pub fn is_active(&self, now: Instant) -> bool {
        now >= self.start_time && !self.is_finished(now)
    }

    /// Check if animation is finished
    pub fn is_finished(&self, now: Instant) -> bool {
        now >= self.start_time + self.duration && !self.repeat
    }

    /// Get animation progress (0.0 to 1.0)
    pub fn progress(&self, now: Instant) -> f32 {
        if now < self.start_time {
            return 0.0;
        }

        let elapsed = now.duration_since(self.start_time);
        let t = elapsed.as_secs_f32() / self.duration.as_secs_f32();

        t.min(1.0)
    }

    /// Apply animation to cards
    pub fn apply(&self, progress: f32, cards: &mut HashMap<u32, MediaCard>) {
        let eased_progress = self.easing.apply(progress);
        let value = self.from + (self.to - self.from) * eased_progress;

        match self.target {
            AnimationTarget::Card(card_id) => {
                if let Some(card) = cards.get_mut(&card_id) {
                    match self.property {
                        AnimationProperty::PositionX => card.layout.bounds.x = value,
                        AnimationProperty::PositionY => card.layout.bounds.y = value,
                        AnimationProperty::Width => card.layout.bounds.width = value,
                        AnimationProperty::Height => card.layout.bounds.height = value,
                        AnimationProperty::Scale => {
                            // Store scale factor (would need to add to CardLayout)
                        }
                        AnimationProperty::Opacity => {
                            // Store opacity (would need to add to CardLayout)
                        }
                        AnimationProperty::Elevation => card.layout.elevation = value,
                        AnimationProperty::Rotation => {
                            // Store rotation (would need to add to CardLayout)
                        }
                        AnimationProperty::TranslateY => {
                            // Temporary offset (would need to add to CardLayout)
                        }
                    }
                }
            }
            AnimationTarget::Gallery(_) => {
                // Gallery-level animations
            }
        }
    }
}

/// Animation target
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationTarget {
    Card(u32),
    Gallery(u32),
}

/// Animatable properties
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationProperty {
    PositionX,
    PositionY,
    Width,
    Height,
    Scale,
    Opacity,
    Elevation,
    Rotation,
    TranslateY,
}

/// Easing functions for natural motion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Spring,
    Smooth,
}

impl EasingFunction {
    /// Apply easing function to progress value (0.0 to 1.0)
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            Self::Linear => t,

            Self::EaseIn => t * t,

            Self::EaseOut => t * (2.0 - t),

            Self::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }

            Self::Spring => {
                // Damped spring oscillation
                let omega = 2.0 * std::f32::consts::PI * 2.0; // 2 cycles
                let zeta = 0.3; // Damping ratio
                let exp = (-zeta * 10.0 * t).exp();
                1.0 - exp * (omega * t).cos()
            }

            Self::Smooth => {
                // Smooth step (cubic hermite)
                t * t * (3.0 - 2.0 * t)
            }
        }
    }
}

/// Interpolate between two values
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Interpolate between two colors
pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color {
        r: lerp(a.r, b.r, t),
        g: lerp(a.g, b.g, t),
        b: lerp(a.b, b.b, t),
        a: lerp(a.a, b.a, t),
    }
}

/// Interpolate between two rectangles
pub fn lerp_rect(a: Rect, b: Rect, t: f32) -> Rect {
    Rect {
        x: lerp(a.x, b.x, t),
        y: lerp(a.y, b.y, t),
        width: lerp(a.width, b.width, t),
        height: lerp(a.height, b.height, t),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_functions() {
        assert_eq!(EasingFunction::Linear.apply(0.5), 0.5);
        assert_eq!(EasingFunction::EaseIn.apply(0.0), 0.0);
        assert_eq!(EasingFunction::EaseOut.apply(1.0), 1.0);

        // EaseIn should be slower at start
        assert!(EasingFunction::EaseIn.apply(0.5) < 0.5);

        // EaseOut should be faster at start
        assert!(EasingFunction::EaseOut.apply(0.5) > 0.5);
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
    }

    #[test]
    fn test_lerp_color() {
        let black = Color::rgb(0, 0, 0);
        let white = Color::rgb(255, 255, 255);
        let gray = lerp_color(black, white, 0.5);

        assert!((gray.r - 0.5).abs() < 0.01);
        assert!((gray.g - 0.5).abs() < 0.01);
        assert!((gray.b - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_animation_progress() {
        let start = Instant::now();
        let anim = Animation {
            start_time: start,
            duration: Duration::from_secs(1),
            ..Default::default()
        };

        assert_eq!(anim.progress(start), 0.0);
        assert_eq!(anim.progress(start + Duration::from_millis(500)), 0.5);
        assert_eq!(anim.progress(start + Duration::from_secs(1)), 1.0);
        assert_eq!(anim.progress(start + Duration::from_secs(2)), 1.0); // Clamped
    }
}
