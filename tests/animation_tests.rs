use radial_menu_rs::{AngleSpring, EasingType, SliceAnimation, Spring1D};
use std::f32::consts::PI;

#[test]
fn test_cubic_easing() {
    let ease_out = EasingType::EaseOutCubic;
    assert_eq!(ease_out.evaluate(0.0), 0.0);
    assert_eq!(ease_out.evaluate(1.0), 1.0);
    // At t=0.5, 1 - 0.5^3 = 0.875
    assert!((ease_out.evaluate(0.5) - 0.875).abs() < 1e-4);
}

#[test]
fn test_spring_1d_convergence() {
    let mut spring = Spring1D::new(0.0);
    spring.set_target(10.0);

    // Simulate 2 seconds of physics
    for _ in 0..120 {
        spring.update(1.0 / 60.0);
    }

    assert!((spring.position - 10.0).abs() < 0.05);
    assert!(spring.velocity.abs() < 0.1);
}

#[test]
fn test_angle_spring_shortest_wrap() {
    // Current angle is 0.1 rad, target is 2PI - 0.1 rad (~6.18 rad)
    // Shortest path is rotating backwards across 0 boundary (-0.2 rad), NOT +6.08 rad!
    let mut angle_spring = AngleSpring::new(0.1);
    angle_spring.set_target(2.0 * PI - 0.1);

    // First step should push velocity negative
    angle_spring.update(0.016);
    assert!(angle_spring.velocity < 0.0);
}

#[test]
fn test_slice_animation_hover_pop() {
    let mut slice_anim = SliceAnimation::new();
    assert_eq!(slice_anim.current_scale(), 1.0);

    slice_anim.set_active(true, 1.15);
    for _ in 0..60 {
        slice_anim.update(1.0 / 60.0);
    }

    assert!((slice_anim.current_scale() - 1.15).abs() < 0.02);
    assert!((slice_anim.current_glow() - 1.0).abs() < 0.02);
}
