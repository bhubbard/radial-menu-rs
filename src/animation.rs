use crate::polar::RadialPointer;
use glam::Vec4;
use serde::{Deserialize, Serialize};

/// Cubic and smooth easing functions for radial menu animations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EasingType {
    Linear,
    #[default]
    EaseOutCubic,
    EaseInOutCubic,
    EaseOutBack,
}

impl EasingType {
    /// Evaluates easing curve for $t \in [0.0, 1.0]$.
    pub fn evaluate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::EaseOutCubic => {
                let p = 1.0 - t;
                1.0 - p * p * p
            }
            Self::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let p = -2.0 * t + 2.0;
                    1.0 - (p * p * p) * 0.5
                }
            }
            Self::EaseOutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                let p = t - 1.0;
                1.0 + c3 * p * p * p + c1 * p * p
            }
        }
    }
}

/// A 1D second-order damped spring system for physics-based UI transitions.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Spring1D {
    pub position: f32,
    pub velocity: f32,
    pub target: f32,
    /// Spring stiffness / angular frequency $\omega$.
    pub stiffness: f32,
    /// Damping ratio $\zeta$ ($1.0$ is critically damped, $<1.0$ is bouncy).
    pub damping: f32,
}

impl Default for Spring1D {
    fn default() -> Self {
        Self {
            position: 0.0,
            velocity: 0.0,
            target: 0.0,
            stiffness: 180.0,
            damping: 18.0,
        }
    }
}

impl Spring1D {
    /// Creates a new spring with initial position and target.
    pub const fn new(initial: f32) -> Self {
        Self {
            position: initial,
            velocity: 0.0,
            target: initial,
            stiffness: 180.0,
            damping: 18.0,
        }
    }

    /// Sets the target destination for the spring.
    #[inline]
    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    /// Advances the spring simulation by `dt` seconds using semi-implicit Euler integration.
    pub fn update(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 0.05); // Prevent instability on large delta times
        let displacement = self.position - self.target;
        let spring_force = -self.stiffness * displacement;
        let damping_force = -self.damping * self.velocity;
        let acceleration = spring_force + damping_force;

        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;
    }
}

/// Angular spring that wraps differences across $[0, 2\pi)$ boundary smoothly without spinning 360°.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AngleSpring {
    pub angle: f32,
    pub velocity: f32,
    pub target: f32,
    pub stiffness: f32,
    pub damping: f32,
}

impl AngleSpring {
    pub const fn new(initial_rad: f32) -> Self {
        Self {
            angle: initial_rad,
            velocity: 0.0,
            target: initial_rad,
            stiffness: 220.0,
            damping: 20.0,
        }
    }

    pub fn set_target(&mut self, target_rad: f32) {
        self.target = target_rad;
    }

    pub fn update(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 0.05);
        // Shortest angular displacement from current angle to target
        let displacement = RadialPointer::angle_difference(self.angle, self.target);
        // Spring pulls toward target -> force is in direction of displacement
        let spring_force = self.stiffness * displacement;
        let damping_force = -self.damping * self.velocity;
        let acceleration = spring_force + damping_force;

        self.velocity += acceleration * dt;
        self.angle = RadialPointer::normalize_angle(self.angle + self.velocity * dt);
    }
}

/// Color lerp utility for animated sector highlights (RGBA).
#[inline]
pub fn lerp_color(a: Vec4, b: Vec4, t: f32) -> Vec4 {
    let t = t.clamp(0.0, 1.0);
    a + (b - a) * t
}

/// Per-slice animated transition state (scale pop, hover glow, selection alpha).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SliceAnimation {
    /// Animated scale factor (typically $1.0$ unselected to $1.15$ active).
    pub scale_spring: Spring1D,
    /// Animated highlight intensity $[0.0, 1.0]$.
    pub glow_spring: Spring1D,
}

impl Default for SliceAnimation {
    fn default() -> Self {
        Self {
            scale_spring: Spring1D::new(1.0),
            glow_spring: Spring1D::new(0.0),
        }
    }
}

impl SliceAnimation {
    /// Creates a slice animation with custom resting scale.
    pub fn new() -> Self {
        Self::default()
    }

    /// Updates hover state: if `is_active`, targets pop scale and glow.
    pub fn set_active(&mut self, is_active: bool, active_scale: f32) {
        if is_active {
            self.scale_spring.set_target(active_scale);
            self.glow_spring.set_target(1.0);
        } else {
            self.scale_spring.set_target(1.0);
            self.glow_spring.set_target(0.0);
        }
    }

    /// Advances slice physics.
    pub fn update(&mut self, dt: f32) {
        self.scale_spring.update(dt);
        self.glow_spring.update(dt);
    }

    #[inline]
    pub fn current_scale(&self) -> f32 {
        self.scale_spring.position
    }

    #[inline]
    pub fn current_glow(&self) -> f32 {
        self.glow_spring.position.clamp(0.0, 1.0)
    }
}
