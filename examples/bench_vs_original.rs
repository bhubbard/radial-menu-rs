//! Benchmark comparing `radial-menu-rs` (Rust) vs original FiveM Lua + NUI/CEF RadialMenu.

use glam::Vec2;
use radial_menu_rs::{
    AngleSpring, GamepadStick, RadialItem, RadialMenu, RadialPointer, SliceAnimation, Spring1D,
};
use std::time::Instant;

fn main() {
    println!("============================================================");
    println!("  radial-menu-rs (Rust) vs FiveM Lua + NUI/CEF Radial Menu  ");
    println!("============================================================");

    // 1. Full 8-Slot Radial Menu Selection (Polar Math + Deadzone + Partitioning)
    println!("\n--- 1. 8-Slot Weapon Wheel Polar Sector Selection ---");
    {
        let pointer = RadialPointer::new(Vec2::ZERO, 25.0, 220.0);
        let items: Vec<RadialItem<()>> = (0..8)
            .map(|i| RadialItem::new(format!("slot_{}", i), format!("Weapon Slot {}", i)))
            .collect();
        let menu = RadialMenu::new(items, pointer);

        let iterations = 10_000_000;
        let start = Instant::now();
        let mut selections = 0;

        for i in 0..iterations {
            let angle = (i as f32) * 0.05;
            let radius = 15.0 + (i % 200) as f32; // spans deadzone to outer radius
            let cursor = Vec2::new(angle.cos() * radius, angle.sin() * radius);

            let (_sample, idx) = menu.select_slice(cursor);
            if idx.is_some() {
                selections += 1;
            }
        }

        std::hint::black_box(selections);
        let elapsed = start.elapsed();
        let ns_per_select = elapsed.as_nanos() as f64 / iterations as f64;
        let selects_per_sec = iterations as f64 / elapsed.as_secs_f64();

        println!(
            "Polar Selections: {} | Time: {:.2?} | Latency: {:.2} ns/eval | {:>10.0} evals/s | Hits: {}",
            iterations, elapsed, ns_per_select, selects_per_sec, selections
        );
    }

    // 2. Physics-Based Shortest-Arc Spring Interpolation
    println!("\n--- 2. Shortest-Arc Angular Spring Damping (Snapping Math) ---");
    {
        let mut spring = AngleSpring::new(0.0);
        let mut scale_spring = Spring1D::new(1.0);
        let mut slice_anim = SliceAnimation::new();

        let iterations = 10_000_000;
        let dt = 0.016;
        let start = Instant::now();
        let mut sum_values = 0.0f32;

        for i in 0..iterations {
            let target_angle = (i % 8) as f32 * std::f32::consts::FRAC_PI_4;
            spring.set_target(target_angle);
            spring.update(dt);

            scale_spring.set_target(1.15);
            scale_spring.update(dt);

            slice_anim.set_active(i % 2 == 0, 1.2);
            slice_anim.update(dt);

            sum_values += spring.angle + scale_spring.position + slice_anim.current_scale();
        }

        std::hint::black_box(sum_values);
        let elapsed = start.elapsed();
        let ns_per_step = elapsed.as_nanos() as f64 / iterations as f64;
        let steps_per_sec = iterations as f64 / elapsed.as_secs_f64();

        println!(
            "Spring Steps: {} | Time: {:.2?} | Latency: {:.2} ns/step | {:>10.0} steps/s",
            iterations, elapsed, ns_per_step, steps_per_sec
        );
    }

    // 3. Gamepad Thumbstick Deadzone Calibration & Polarization
    println!("\n--- 3. Gamepad Thumbstick Deadzone Calibration & Normalization ---");
    {
        let stick = GamepadStick::new(0.20);
        let pointer = RadialPointer::new(Vec2::ZERO, 30.0, 200.0);
        let iterations = 10_000_000;
        let start = Instant::now();
        let mut active_count = 0;

        for i in 0..iterations {
            let rx = ((i % 200) as f32 - 100.0) / 100.0;
            let ry = (((i / 200) % 200) as f32 - 100.0) / 100.0;

            let res = stick.process(Vec2::new(rx, ry), &pointer);
            if !res.is_neutral {
                active_count += 1;
            }
        }

        std::hint::black_box(active_count);
        let elapsed = start.elapsed();
        let ns_per_stick = elapsed.as_nanos() as f64 / iterations as f64;
        let sticks_per_sec = iterations as f64 / elapsed.as_secs_f64();

        println!(
            "Stick Evaluations: {} | Time: {:.2?} | Latency: {:.2} ns/eval | {:>10.0} evals/s | Active: {}",
            iterations, elapsed, ns_per_stick, sticks_per_sec, active_count
        );
    }

    // 4. Nested Sub-Item Variant Cycling Throughput
    println!("\n--- 4. Nested Sub-Item Weapon Variant Cycling ---");
    {
        let mut slot = RadialItem::<()>::new("rifles", "Assault Rifles")
            .with_sub_item(RadialItem::new("carbine_rifle", "Carbine Rifle"))
            .with_sub_item(RadialItem::new("special_carbine", "Special Carbine"))
            .with_sub_item(RadialItem::new("bullpup_rifle", "Bullpup Rifle"))
            .with_sub_item(RadialItem::new("advanced_rifle", "Advanced Rifle"))
            .with_sub_item(RadialItem::new("heavy_rifle", "Heavy Rifle"));

        let iterations = 10_000_000;
        let start = Instant::now();
        let mut cycles = 0;

        for i in 0..iterations {
            if i % 2 == 0 {
                slot.next_sub_item();
            } else {
                slot.prev_sub_item();
            }
            cycles += slot.active_variant().id.len();
        }

        std::hint::black_box(cycles);
        let elapsed = start.elapsed();
        let ns_per_cycle = elapsed.as_nanos() as f64 / iterations as f64;
        let cycles_per_sec = iterations as f64 / elapsed.as_secs_f64();

        println!(
            "Sub-item Cycles: {} | Time: {:.2?} | Latency: {:.2} ns/cycle | {:>10.0} cycles/s",
            iterations, elapsed, ns_per_cycle, cycles_per_sec
        );
    }

    println!("\n============================================================");
    println!("                      Benchmark Complete                    ");
    println!("============================================================");
}
