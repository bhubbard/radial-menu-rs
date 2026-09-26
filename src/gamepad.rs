use crate::polar::{PolarSample, RadialPointer, RadialStatus, TWO_PI};
use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Gamepad thumbstick axis processor for radial menus and weapon wheels.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GamepadStick {
    /// Radial deadzone threshold in $[0.0, 1.0]$ (default: $0.22$).
    pub deadzone: f32,
    /// Maximum thumbstick saturation threshold (default: $0.98$).
    pub saturation: f32,
    /// Optional angular snapping threshold in radians (snaps to slice center).
    pub snap_threshold_rad: Option<f32>,
    /// Invert vertical axis (for platforms where stick down is $-1.0$).
    pub invert_y: bool,
}

impl Default for GamepadStick {
    fn default() -> Self {
        Self {
            deadzone: 0.22,
            saturation: 0.98,
            snap_threshold_rad: None,
            invert_y: false,
        }
    }
}

/// Result of evaluating gamepad thumbstick deflection.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GamepadInput {
    /// Raw input stick vector.
    pub raw: Vec2,
    /// Filtered stick vector after radial deadzone and saturation remapping.
    pub filtered: Vec2,
    /// True if stick magnitude is within the deadzone (no input).
    pub is_neutral: bool,
    /// Normalized deflection magnitude in $[0.0, 1.0]$.
    pub magnitude: f32,
    /// Clock angle in radians $[0, 2\pi)$ (starting at 12 o'clock).
    pub clock_angle: f32,
    /// Evaluated polar sample based on virtual screen pointer.
    pub polar_sample: PolarSample,
}

impl GamepadStick {
    /// Creates a gamepad thumbstick parser with given deadzone.
    pub const fn new(deadzone: f32) -> Self {
        Self {
            deadzone,
            saturation: 0.98,
            snap_threshold_rad: None,
            invert_y: false,
        }
    }

    /// Enables angle snapping within a threshold in radians.
    pub fn with_snapping(mut self, threshold_rad: f32) -> Self {
        self.snap_threshold_rad = Some(threshold_rad);
        self
    }

    /// Inverts the vertical Y axis.
    pub fn with_inverted_y(mut self, invert: bool) -> Self {
        self.invert_y = invert;
        self
    }

    /// Processes raw thumbstick input $(x, y) \in [-1.0, 1.0]$.
    ///
    /// Applies circular/radial deadzone filtering (rather than square/axial deadzone)
    /// to guarantee smooth 360° input without diagonal clipping.
    pub fn process(&self, raw: Vec2, pointer_engine: &RadialPointer) -> GamepadInput {
        let mut stick = raw;
        if self.invert_y {
            stick.y = -stick.y;
        }

        let raw_mag = stick.length();

        if raw_mag < self.deadzone {
            let sample = PolarSample {
                status: RadialStatus::Neutral,
                delta: Vec2::ZERO,
                radius: 0.0,
                normalized_radius: 0.0,
                angle_standard: 0.0,
                angle_clock: 0.0,
                angle_deg: 0.0,
            };

            return GamepadInput {
                raw,
                filtered: Vec2::ZERO,
                is_neutral: true,
                magnitude: 0.0,
                clock_angle: 0.0,
                polar_sample: sample,
            };
        }

        // Remap magnitude linearly from [deadzone, saturation] -> [0.0, 1.0]
        let range = (self.saturation - self.deadzone).max(1e-4);
        let remap_mag = ((raw_mag - self.deadzone) / range).clamp(0.0, 1.0);
        let dir = stick / raw_mag;
        let filtered = dir * remap_mag;

        // Virtual cursor in pointer coordinates
        let virtual_cursor = pointer_engine.origin + filtered * pointer_engine.max_radius;
        let mut polar_sample = pointer_engine.evaluate(virtual_cursor);

        // Optional snapping to nearest slice
        if let Some(snap_thresh) = self.snap_threshold_rad {
            // Check if clock_angle is within snap_thresh of a cardinal or slice division
            let step = TWO_PI / 8.0; // standard 8-way slice
            let nearest_idx = (polar_sample.angle_clock / step).round() as usize % 8;
            let target_angle = (nearest_idx as f32) * step;
            let diff = RadialPointer::angle_difference(polar_sample.angle_clock, target_angle);
            if diff.abs() <= snap_thresh {
                polar_sample.angle_clock = target_angle;
                polar_sample.angle_deg = target_angle.to_degrees();
            }
        }

        GamepadInput {
            raw,
            filtered,
            is_neutral: false,
            magnitude: remap_mag,
            clock_angle: polar_sample.angle_clock,
            polar_sample,
        }
    }
}
