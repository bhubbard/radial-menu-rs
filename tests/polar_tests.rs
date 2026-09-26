use glam::Vec2;
use radial_menu_rs::{RadialPointer, RadialStatus};
use std::f32::consts::PI;

#[test]
fn test_deadzone_and_radial_status() {
    let pointer = RadialPointer::new(Vec2::ZERO, 40.0, 200.0);

    // Inside deadzone (< 40px)
    let s_neutral = pointer.evaluate(Vec2::new(20.0, 10.0));
    assert_eq!(s_neutral.status, RadialStatus::Neutral);
    assert!(!s_neutral.status.is_selectable());

    // Inside active ring (100px)
    let s_active = pointer.evaluate(Vec2::new(100.0, 0.0));
    assert_eq!(s_active.status, RadialStatus::Active);
    assert!(s_active.status.is_selectable());

    // Beyond max radius (> 200px)
    let s_beyond = pointer.evaluate(Vec2::new(250.0, 0.0));
    assert_eq!(s_beyond.status, RadialStatus::BeyondMax);
    assert!(s_beyond.status.is_selectable());
}

#[test]
fn test_clock_angles_cardinal_directions() {
    let pointer = RadialPointer::new(Vec2::ZERO, 20.0, 200.0);

    // Top (12 o'clock in screen space: y = -100)
    let s_top = pointer.evaluate(Vec2::new(0.0, -100.0));
    assert!(s_top.angle_clock.abs() < 1e-3 || (s_top.angle_clock - 2.0 * PI).abs() < 1e-3);
    assert!(s_top.angle_deg.abs() < 1.0 || (s_top.angle_deg - 360.0).abs() < 1.0);

    // Right (3 o'clock: x = +100, y = 0)
    let s_right = pointer.evaluate(Vec2::new(100.0, 0.0));
    assert!((s_right.angle_clock - (PI * 0.5)).abs() < 1e-3);
    assert!((s_right.angle_deg - 90.0).abs() < 1.0);

    // Bottom (6 o'clock: x = 0, y = +100)
    let s_bottom = pointer.evaluate(Vec2::new(0.0, 100.0));
    assert!((s_bottom.angle_clock - PI).abs() < 1e-3);
    assert!((s_bottom.angle_deg - 180.0).abs() < 1.0);

    // Left (9 o'clock: x = -100, y = 0)
    let s_left = pointer.evaluate(Vec2::new(-100.0, 0.0));
    assert!((s_left.angle_clock - (PI * 1.5)).abs() < 1e-3);
    assert!((s_left.angle_deg - 270.0).abs() < 1.0);
}

#[test]
fn test_angle_normalization_and_differences() {
    let diff = RadialPointer::angle_difference(0.1, 2.0 * PI - 0.1);
    // Shortest way from 0.1 to 2PI - 0.1 (-0.1) is -0.2 rad
    assert!((diff - (-0.2)).abs() < 1e-4);

    let norm = RadialPointer::normalize_angle(3.0 * PI);
    assert!((norm - PI).abs() < 1e-4);
}
