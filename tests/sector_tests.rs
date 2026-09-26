use glam::Vec2;
use radial_menu_rs::{RadialItem, RadialMenu, RadialPointer};
use std::f32::consts::PI;

#[test]
fn test_eight_slot_weapon_wheel_partitioning() {
    let pointer = RadialPointer::new(Vec2::ZERO, 30.0, 200.0);
    let items = (0..8)
        .map(|i| RadialItem::<()>::new(format!("slot_{}", i), format!("Slot {}", i)))
        .collect();

    let menu = RadialMenu::new(items, pointer);
    assert_eq!(menu.slice_count(), 8);
    // Angular width = 2PI / 8 = PI / 4 (45 degrees)
    assert!((menu.angular_width() - (PI / 4.0)).abs() < 1e-4);

    // 12 o'clock top (0 rad) -> should select slice 0
    let (s0, idx0) = menu.select_slice(Vec2::new(0.0, -100.0));
    assert!(s0.status.is_selectable());
    assert_eq!(idx0, Some(0));

    // 3 o'clock right (90 deg) -> should select slice 2 (0=top, 1=top-right, 2=right)
    let (_s2, idx2) = menu.select_slice(Vec2::new(100.0, 0.0));
    assert_eq!(idx2, Some(2));

    // 6 o'clock bottom (180 deg) -> slice 4
    let (_, idx4) = menu.select_slice(Vec2::new(0.0, 100.0));
    assert_eq!(idx4, Some(4));

    // Inside deadzone -> None
    let (_, idx_neutral) = menu.select_slice(Vec2::new(10.0, 10.0));
    assert_eq!(idx_neutral, None);
}

#[test]
fn test_sub_items_tier_cycling() {
    let mut pistol_slot = RadialItem::<()>::new("pistols", "Pistols")
        .with_sub_item(RadialItem::new("combat_pistol", "Combat Pistol"))
        .with_sub_item(RadialItem::new("ap_pistol", "AP Pistol"))
        .with_sub_item(RadialItem::new("heavy_revolver", "Heavy Revolver"));

    assert_eq!(pistol_slot.active_variant().id, "combat_pistol");

    // Cycle next
    pistol_slot.next_sub_item();
    assert_eq!(pistol_slot.active_variant().id, "ap_pistol");

    pistol_slot.next_sub_item();
    assert_eq!(pistol_slot.active_variant().id, "heavy_revolver");

    // Wrap around
    pistol_slot.next_sub_item();
    assert_eq!(pistol_slot.active_variant().id, "combat_pistol");

    // Cycle backwards
    pistol_slot.prev_sub_item();
    assert_eq!(pistol_slot.active_variant().id, "heavy_revolver");
}

#[test]
fn test_sector_arc_contains_angle() {
    let pointer = RadialPointer::new(Vec2::ZERO, 30.0, 200.0);
    let items = (0..4)
        .map(|i| RadialItem::<()>::new(format!("s{}", i), format!("S{}", i)))
        .collect();
    let menu = RadialMenu::new(items, pointer);

    // 4 slices: each is 90 degrees (PI/2)
    // Slice 0 center is 0 (12 o'clock), spans [-PI/4, PI/4] or [7PI/4, PI/4]
    let arc0 = menu.slice_arc(0).unwrap();
    assert!(arc0.contains_angle(0.1));
    assert!(arc0.contains_angle(2.0 * PI - 0.1));
    assert!(!arc0.contains_angle(PI));
}
