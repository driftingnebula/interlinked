//! All supported GEGL operations.

mod generator;

pub use crate::gegl_operation;

gegl_operation!(
  struct_name: Bloom,
  gegl_name: "bloom",
  append_crop: false,
  values: (
    limit_exposure: bool, false, "Don't over-expose highlights.",
    radius: f64, 10.0, "Glow radius.",
    softness: f64, 25.0, "Glow-area edge softness.",
    strength: f64, 50.0, "Glow strength.",
    threshold: f64, 50.0, "Glow-area brightness threshold.",
  ),
);

gegl_operation!(
  struct_name: Cartoon,
  gegl_name: "cartoon",
  append_crop: true,
  values: (
    mask_radius: f64, 7.0, "The mask radius.",
    pct_black: f64, 0.2, "The percentage of black.",
  ),
);
