# radial-menu-rs

[![GitHub Pages](https://img.shields.io/badge/docs-GitHub%20Pages-blue.svg)](https://code.brandonhubbard.com/radial-menu-rs/)
[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![Rust: 2024 Edition](https://img.shields.io/badge/Rust-2024%20Edition-black?logo=rust)](https://www.rust-lang.org)

> Pure Rust radial menu and weapon wheel system with angle math, deadzone filtering, sub-item tier support, and physics-based spring animations. Ported and modernized from `Xenobyte/RadialMenu`.

Interactive Simulator Demo: **[code.brandonhubbard.com/radial-menu-rs](https://code.brandonhubbard.com/radial-menu-rs/)**

---

## Features

- 🧭 **Trigonometry & Polar Mapping**:
  - Center origin $\mathbf{C} = (x_c, y_c)$, cursor $\mathbf{P} = (x_p, y_p)$, vector $\Delta = \mathbf{P} - \mathbf{C}$.
  - Radius $r = \|\Delta\|$ and polar angle $\theta = \operatorname{atan2}(\Delta y, \Delta x)$.
  - 12 o'clock top orientation offset ($-\pi/2$).
  - Configurable deadzone $r_{\min}$ (neutral state) and outer radius $r_{\max}$.
- 🍕 **Sector Partitioning & Slice Selection**:
  - Partition into $N \ge 2$ slices with angular width $\Delta \theta = 2\pi / N$.
  - Symmetrical center alignment index calculation:
    $$i = \left\lfloor \frac{\theta' + \frac{\Delta \theta}{2}}{\Delta \theta} \right\rfloor \pmod N$$
  - Sub-items / nested tier cycling (e.g. Pistols $\rightarrow$ [Combat Pistol, AP Pistol, Heavy Revolver]).
- 🌊 **Motion & Physics-Based Spring Easing**:
  - Second-order damped spring system (`Spring1D`) for hover scale pop and glow.
  - Angular spring (`AngleSpring`) that wraps shortest path across $[0, 2\pi)$ boundary without full 360° spinouts.
- 🎮 **Gamepad & Thumbstick Input**:
  - Circular/radial deadzone filtering (prevents diagonal clipping found in square axial deadzones).
  - Optional angle snapping to nearest slice bisector.

---

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
radial-menu-rs = { git = "https://github.com/bhubbard/radial-menu-rs" }
glam = "0.29"
```

### Building an 8-Slot Weapon Wheel

```rust
use glam::Vec2;
use radial_menu_rs::{RadialItem, RadialMenu, RadialPointer};

fn main() {
    let pointer = RadialPointer::new(Vec2::new(960.0, 540.0), 40.0, 240.0);

    let items = vec![
        RadialItem::new("pistols", "Pistols")
            .with_sub_item(RadialItem::new("combat_pistol", "Combat Pistol"))
            .with_sub_item(RadialItem::new("ap_pistol", "AP Pistol")),
        RadialItem::new("smg", "SMG"),
        RadialItem::new("rifles", "Rifles"),
        RadialItem::new("snipers", "Snipers"),
        RadialItem::new("melee", "Melee"),
        RadialItem::new("shotguns", "Shotguns"),
        RadialItem::new("heavy", "Heavy"),
        RadialItem::new("throwables", "Throwables"),
    ];

    let mut menu = RadialMenu::new(items, pointer);

    // Evaluate cursor input at (960, 400) -> directly above center (12 o'clock)
    let (sample, active_slot) = menu.select_slice(Vec2::new(960.0, 400.0));

    if let Some(idx) = active_slot {
        let item = &menu.items[idx];
        println!("Selected Category: {}", item.title);
        println!("Active Weapon: {}", item.active_variant().title);
    }
}
```

### Gamepad Thumbstick Handling

```rust
use glam::Vec2;
use radial_menu_rs::{GamepadStick, RadialPointer};

let pointer = RadialPointer::new(Vec2::ZERO, 30.0, 200.0);
let stick = GamepadStick::new(0.20).with_snapping(0.12);

// Right stick tilted to upper-right (0.707, -0.707)
let raw_axis = Vec2::new(0.707, -0.707);
let input = stick.process(raw_axis, &pointer);

if !input.is_neutral {
    println!("Thumbstick Deflection: {:.2}", input.magnitude);
    println!("Clock Angle: {:.1}°", input.clock_angle.to_degrees());
}
```

---

## Interactive Weapon Wheel Simulator

Experience the live simulator at [code.brandonhubbard.com/radial-menu-rs](https://code.brandonhubbard.com/radial-menu-rs/):
- **8-Slot GTA V Weapon Wheel**: Full mouse and joystick radial tracking.
- **Deadzone Visualization**: Live visual threshold feedback for neutral state.
- **Spring Pop Physics**: Animated hover expansion with damping.
- **Sub-Item Tier Cycling**: Use mouse scroll wheel or arrows to change weapon variants in any highlighted slot.
- **Synthesized Audio**: Procedural click audio feedback via Web Audio API.

---

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.
