use core::f32::consts::PI;
use glam::Vec2;
use serde::{Deserialize, Serialize};

pub const TWO_PI: f32 = 2.0 * PI;
pub const HALF_PI: f32 = 0.5 * PI;

/// Pointer selection status based on distance from menu origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RadialStatus {
    /// Inside deadzone ($r < r_{\min}$); no slice selected.
    Neutral,
    /// Within active selection ring ($r_{\min} \le r \le r_{\max}$).
    Active,
    /// Beyond maximum radius ($r > r_{\max}$), but can clamp directionally.
    BeyondMax,
}

impl RadialStatus {
    /// Returns true if pointer is outside deadzone and can select an item.
    #[inline]
    pub fn is_selectable(&self) -> bool {
        matches!(self, Self::Active | Self::BeyondMax)
    }
}

/// Evaluated polar sample from cursor/pointer position relative to menu center.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PolarSample {
    /// Selection status (Neutral inside deadzone, Active, or BeyondMax).
    pub status: RadialStatus,
    /// Offset vector $\Delta = \mathbf{P} - \mathbf{C}$.
    pub delta: Vec2,
    /// Euclidean distance from origin $r = \|\Delta\|$.
    pub radius: f32,
    /// Normalized radius $t \in [0.0, 1.0]$ between deadzone and max radius.
    pub normalized_radius: f32,
    /// Standard mathematical polar angle in radians $(-\pi, \pi]$ where $0$ is $+X$ (3 o'clock).
    pub angle_standard: f32,
    /// Clockwise angle in $[0, 2\pi)$ starting at 12 o'clock (top, $-Y$ in screen space).
    pub angle_clock: f32,
    /// Clockwise angle in degrees $[0.0, 360.0)$ starting at 12 o'clock.
    pub angle_deg: f32,
}

/// Trigonometry and polar coordinate mapping engine for radial menus and weapon wheels.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RadialPointer {
    /// Center origin point $\mathbf{C} = (x_c, y_c)$.
    pub origin: Vec2,
    /// Minimum deadzone radius threshold $r_{\min}$.
    pub min_radius: f32,
    /// Maximum selection radius $r_{\max}$.
    pub max_radius: f32,
    /// Angle offset in radians to orient slice 0 (default: $-\pi/2$, top 12 o'clock).
    pub clock_offset: f32,
}

impl Default for RadialPointer {
    fn default() -> Self {
        Self {
            origin: Vec2::ZERO,
            min_radius: 30.0,
            max_radius: 200.0,
            clock_offset: -HALF_PI, // 12 o'clock top in screen coordinates
        }
    }
}

impl RadialPointer {
    /// Creates a new `RadialPointer` engine with origin and radii bounds.
    pub const fn new(origin: Vec2, min_radius: f32, max_radius: f32) -> Self {
        Self {
            origin,
            min_radius,
            max_radius,
            clock_offset: -HALF_PI,
        }
    }

    /// Sets custom angular offset (radians) for index 0 orientation.
    pub fn with_clock_offset(mut self, offset_rad: f32) -> Self {
        self.clock_offset = offset_rad;
        self
    }

    /// Evaluates cursor or touch position $\mathbf{P} = (x_p, y_p)$ relative to origin.
    ///
    /// - Vector $\Delta = \mathbf{P} - \mathbf{C}$.
    /// - Radius $r = \|\Delta\|$.
    /// - Polar angle $\theta = \operatorname{atan2}(\Delta y, \Delta x)$.
    pub fn evaluate(&self, cursor_pos: Vec2) -> PolarSample {
        let delta = cursor_pos - self.origin;
        let radius = delta.length();

        let status = if radius < self.min_radius {
            RadialStatus::Neutral
        } else if radius > self.max_radius {
            RadialStatus::BeyondMax
        } else {
            RadialStatus::Active
        };

        let span = (self.max_radius - self.min_radius).max(1e-4);
        let normalized_radius = ((radius - self.min_radius) / span).clamp(0.0, 1.0);

        let angle_standard = if radius > 1e-5 {
            delta.y.atan2(delta.x)
        } else {
            0.0
        };

        // In standard screen space (+Y is down):
        // Top 12 o'clock is at delta = (0, -1), whose atan2 is -PI/2.
        // We calculate clockwise angular displacement starting from 12 o'clock:
        let angle_clock = Self::normalize_angle(angle_standard - self.clock_offset);
        let angle_deg = angle_clock.to_degrees();

        PolarSample {
            status,
            delta,
            radius,
            normalized_radius,
            angle_standard,
            angle_clock,
            angle_deg,
        }
    }

    /// Evaluates direct normalized thumbstick axes $(x, y) \in [-1.0, 1.0]$.
    pub fn evaluate_thumbstick(&self, stick: Vec2) -> PolarSample {
        // Virtual cursor mapped onto [min_radius, max_radius]
        let virtual_cursor = self.origin + stick * self.max_radius;
        self.evaluate(virtual_cursor)
    }

    /// Normalizes any angle to $[0, 2\pi)$.
    #[inline]
    pub fn normalize_angle(angle: f32) -> f32 {
        let mut a = angle % TWO_PI;
        if a < 0.0 {
            a += TWO_PI;
        }
        a
    }

    /// Normalizes angle to $(-\pi, \pi]$.
    #[inline]
    pub fn normalize_angle_signed(angle: f32) -> f32 {
        let mut a = Self::normalize_angle(angle);
        if a > PI {
            a -= TWO_PI;
        }
        a
    }

    /// Computes the shortest angular difference $(\beta - \alpha)$ in radians $(-\pi, \pi]$.
    pub fn angle_difference(from: f32, to: f32) -> f32 {
        Self::normalize_angle_signed(to - from)
    }

    /// Computes unit Cartesian vector from clockwise clock angle (starting at top 12 o'clock).
    pub fn vector_from_clock_angle(clock_angle_rad: f32, clock_offset: f32) -> Vec2 {
        let standard = clock_angle_rad + clock_offset;
        Vec2::new(standard.cos(), standard.sin())
    }
}
