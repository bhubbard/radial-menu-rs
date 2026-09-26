use glam::Vec2;
use radial_menu_rs::{GamepadStick, RadialPointer};
use std::f32::consts::PI;

#[test]
fn test_gamepad_radial_deadzone() {
    let stick = GamepadStick::new(0.20);
    let pointer = RadialPointer::new(Vec2::ZERO, 30.0, 200.0);

    // Minor stick drift (0.10 magnitude) -> Neutral
    let res_drift = stick.process(Vec2::new(0.08, 0.06), &pointer);
    assert!(res_drift.is_neutral);
    assert_eq!(res_drift.magnitude, 0.0);

    // Intentional deflection (0.80 magnitude right) -> Active
    let res_active = stick.process(Vec2::new(0.80, 0.0), &pointer);
    assert!(!res_active.is_neutral);
    assert!(res_active.magnitude > 0.7);
    assert!((res_active.clock_angle - (PI * 0.5)).abs() < 1e-2);
}

#[test]
fn test_gamepad_angle_snapping() {
    let stick = GamepadStick::new(0.20).with_snapping(0.15); // Snap within ~8.5 degrees
    let pointer = RadialPointer::new(Vec2::ZERO, 30.0, 200.0);

    // Vector pointing slightly off 12 o'clock (e.g. angle ~0.05 rad)
    let raw = Vec2::new(0.04, -0.9);
    let res = stick.process(raw, &pointer);
    // Should snap to exact 0.0 rad (12 o'clock)
    assert_eq!(res.polar_sample.angle_clock, 0.0);
}
