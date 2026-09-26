use crate::polar::{PolarSample, RadialPointer, TWO_PI};
use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Arc boundaries and geometry for a specific sector slice.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SectorArc {
    /// Slice index in `0..slice_count`.
    pub index: usize,
    /// Center bisector angle in clock radians $[0, 2\pi)$.
    pub center_angle: f32,
    /// Counter-clockwise starting edge angle in clock radians $[0, 2\pi)$.
    pub start_angle: f32,
    /// Clockwise ending edge angle in clock radians $[0, 2\pi)$.
    pub end_angle: f32,
    /// Total angular span width in radians ($\Delta \theta = 2\pi / N$).
    pub angular_width: f32,
}

impl SectorArc {
    /// Returns true if a given clock angle falls inside this sector arc.
    pub fn contains_angle(&self, clock_angle: f32) -> bool {
        let norm_angle = RadialPointer::normalize_angle(clock_angle);
        let diff = RadialPointer::angle_difference(self.center_angle, norm_angle);
        diff.abs() <= (self.angular_width * 0.5) + 1e-5
    }

    /// Computes the centroid point in local Cartesian coordinates given inner and outer radii.
    pub fn centroid(&self, inner_radius: f32, outer_radius: f32, clock_offset: f32) -> Vec2 {
        let mid_r = (inner_radius + outer_radius) * 0.5;
        let standard_angle = self.center_angle + clock_offset;
        Vec2::new(standard_angle.cos() * mid_r, standard_angle.sin() * mid_r)
    }
}

/// A single item/slot within a radial menu or weapon wheel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadialItem<T = ()> {
    /// Unique identifier for this item or weapon category.
    pub id: String,
    /// Display label (e.g. "Pistols", "Heavy", "Melee").
    pub title: String,
    /// Custom metadata/payload attached to the item.
    pub payload: T,
    /// Whether this sector is disabled / locked / greyed out.
    pub is_enabled: bool,
    /// Optional nested tier or sub-items (e.g. Pistol -> [Combat Pistol, AP Pistol, Revolver]).
    pub sub_items: Vec<RadialItem<T>>,
    /// Currently active sub-item index if sub_items is non-empty.
    pub selected_sub_index: usize,
}

impl<T: Default> RadialItem<T> {
    /// Creates a new enabled radial item with a given ID and title.
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            payload: T::default(),
            is_enabled: true,
            sub_items: Vec::new(),
            selected_sub_index: 0,
        }
    }
}

impl<T> RadialItem<T> {
    /// Creates a new radial item with custom payload.
    pub fn with_payload(id: impl Into<String>, title: impl Into<String>, payload: T) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            payload,
            is_enabled: true,
            sub_items: Vec::new(),
            selected_sub_index: 0,
        }
    }

    /// Adds a sub-item variant to this item.
    pub fn with_sub_item(mut self, sub_item: RadialItem<T>) -> Self {
        self.sub_items.push(sub_item);
        self
    }

    /// Adds multiple sub-items.
    pub fn with_sub_items(mut self, items: impl IntoIterator<Item = RadialItem<T>>) -> Self {
        self.sub_items.extend(items);
        self
    }

    /// Returns the currently active sub-item if any exist, or self.
    pub fn active_variant(&self) -> &Self {
        if self.sub_items.is_empty() {
            self
        } else {
            let idx = self.selected_sub_index.min(self.sub_items.len() - 1);
            &self.sub_items[idx]
        }
    }

    /// Cycles to the next sub-item variant (e.g., mouse wheel up / d-pad right).
    pub fn next_sub_item(&mut self) -> usize {
        if !self.sub_items.is_empty() {
            self.selected_sub_index = (self.selected_sub_index + 1) % self.sub_items.len();
        }
        self.selected_sub_index
    }

    /// Cycles to the previous sub-item variant (e.g., mouse wheel down / d-pad left).
    pub fn prev_sub_item(&mut self) -> usize {
        if !self.sub_items.is_empty() {
            if self.selected_sub_index == 0 {
                self.selected_sub_index = self.sub_items.len() - 1;
            } else {
                self.selected_sub_index -= 1;
            }
        }
        self.selected_sub_index
    }
}

/// Radial menu sector partitioning and slice selection controller.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadialMenu<T = ()> {
    /// Items corresponding to each sector slice.
    pub items: Vec<RadialItem<T>>,
    /// Pointer geometry configuration (deadzone, max radius, clock offset).
    pub pointer: RadialPointer,
}

impl<T> RadialMenu<T> {
    /// Creates a new radial menu with the specified items and pointer settings.
    pub fn new(items: Vec<RadialItem<T>>, pointer: RadialPointer) -> Self {
        Self { items, pointer }
    }

    /// Number of sector slices in this menu.
    #[inline]
    pub fn slice_count(&self) -> usize {
        self.items.len()
    }

    /// Angular span width of each slice in radians: $\Delta \theta = 2\pi / N$.
    #[inline]
    pub fn angular_width(&self) -> f32 {
        let n = self.slice_count();
        if n > 0 {
            TWO_PI / (n as f32)
        } else {
            TWO_PI
        }
    }

    /// Computes the active slice index from a given clock angle $\theta' \in [0, 2\pi)$:
    ///
    /// $$i = \left\lfloor \frac{\theta' + \frac{\Delta \theta}{2}}{\Delta \theta} \right\rfloor \pmod N$$
    pub fn slice_index_from_clock_angle(&self, clock_angle_rad: f32) -> Option<usize> {
        let n = self.slice_count();
        if n == 0 {
            return None;
        }

        let delta_theta = self.angular_width();
        let half_delta = delta_theta * 0.5;

        // Shift by half slice so slice 0 centers symmetrically on 12 o'clock
        let shifted = RadialPointer::normalize_angle(clock_angle_rad + half_delta);
        let index = (shifted / delta_theta).floor() as usize;

        Some(index % n)
    }

    /// Computes the bounding arc geometry for slice `index`.
    pub fn slice_arc(&self, index: usize) -> Option<SectorArc> {
        let n = self.slice_count();
        if index >= n || n == 0 {
            return None;
        }

        let delta_theta = self.angular_width();
        let center_angle = (index as f32) * delta_theta;
        let start_angle = RadialPointer::normalize_angle(center_angle - (delta_theta * 0.5));
        let end_angle = RadialPointer::normalize_angle(center_angle + (delta_theta * 0.5));

        Some(SectorArc {
            index,
            center_angle,
            start_angle,
            end_angle,
            angular_width: delta_theta,
        })
    }

    /// Evaluates cursor position and returns the selected slice index if outside deadzone.
    pub fn select_slice(&self, cursor_pos: Vec2) -> (PolarSample, Option<usize>) {
        let sample = self.pointer.evaluate(cursor_pos);
        if sample.status.is_selectable() {
            let idx = self.slice_index_from_clock_angle(sample.angle_clock);
            (sample, idx)
        } else {
            (sample, None)
        }
    }

    /// Returns a reference to the active item based on cursor position.
    pub fn active_item(&self, cursor_pos: Vec2) -> Option<&RadialItem<T>> {
        let (_, idx) = self.select_slice(cursor_pos);
        idx.and_then(|i| self.items.get(i))
    }

    /// Returns a mutable reference to the active item based on cursor position.
    pub fn active_item_mut(&mut self, cursor_pos: Vec2) -> Option<&mut RadialItem<T>> {
        let (_, idx) = self.select_slice(cursor_pos);
        if let Some(i) = idx {
            self.items.get_mut(i)
        } else {
            None
        }
    }
}
