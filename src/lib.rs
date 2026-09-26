//! # radial-menu-rs
//!
//! Pure Rust radial menu and weapon wheel system with angle math, deadzone filtering,
//! sub-item tier support, and physics-based spring animations.
//!
//! Ported and modernized from `Xenobyte/RadialMenu`.

pub mod animation;
pub mod gamepad;
pub mod polar;
pub mod sector;

pub use animation::{AngleSpring, EasingType, SliceAnimation, Spring1D, lerp_color};
pub use gamepad::{GamepadInput, GamepadStick};
pub use polar::{PolarSample, RadialPointer, RadialStatus, TWO_PI};
pub use sector::{RadialItem, RadialMenu, SectorArc};
