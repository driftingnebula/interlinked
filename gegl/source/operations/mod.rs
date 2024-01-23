//! All supported GEGL operations.

mod enums;
mod generator;

pub use enums::*;

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

gegl_operation!(
  struct_name: CellNoise,
  gegl_name: "cell-noise",
  append_crop: true,
  values: (
    iterations: i64, 1, "The number of noise octaves.",
    palettize: bool, false, "Fill each cell with a random color.",
    rank: i64, 1, "Select the n-th closest point",
    scale: f64, 1.0, "The scale of the noise function.",
    seed: f64, 0.0, "The random seed for the noise function.",
    shape: f64, 2.0, "Interpolate between Manhattan and Euclidean distance.",
  ),
);

gegl_operation!(
  struct_name: Crop,
  gegl_name: "crop",
  append_crop: false,
  values: (
    height: f64, 0.0, "The wanted height of the buffer.",
    reset_origin: bool, false, "Reset the origin for the coordinates.",
    width: f64, 0.0, "The wanted width of the buffer.",
    x: f64, 0.0, "The X coordinate to start from.",
    y: f64, 0.0, "The Y coordinate to start from.",
  ),
);

gegl_operation!(
  struct_name: DiffractionPatterns,
  gegl_name: "diffraction-patterns",
  append_crop: true,
  values: (
    blue_contours: f64, 0.97, "Number of contours (blue);",
    blue_frequency: f64, 1.12, "Light frequency (blue).",
    blue_sedges: f64, 0.64, "Number of sharp edges (blue).",
    brightness: f64, 0.07, "Brightness and shifting/fattening of contours.",
    green_contours: f64, 0.82, "Number of contours (green);",
    green_frequency: f64, 1.22, "Light frequency (green).",
    green_sedges: f64, 0.68, "Number of sharp edges (green).",
    height: i64, 200, "Height of the generated buffer.",
    polarization: f64, -0.47, "Polarization.",
    red_contours: f64, 0.82, "Number of contours (red);",
    red_frequency: f64, 0.81, "Light frequency (red).",
    red_sedges: f64, 0.61, "Number of sharp edges (red).",
    scattering: f64, 37.13, "Scattering (speed vs. quality).",
    width: i64, 200, "Width of the generated buffer.",
  ),
);

gegl_operation!(
  struct_name: FocusBlur,
  gegl_name: "focus-blur",
  append_crop: false,
  values: (
    aspect_ratio: f64, 0.0, "The aspect ratio of the focus region.",
    blur_radius: f64, 25.0, "Out-of-focus blur radius.",
    blur_type: FocusBlurType, FocusBlurType::Gaussian, "The blur type.",
    focus: f64, 0.25, "The focus region's inner limit.",
    highlight_factor: f64, 0.0, "Relative highlight strength.",
    highlight_threshold_high: f64, 1.0, "Highlight threshold (high).",
    highlight_threshold_low: f64, 0.0, "Highlight threshold (low).",
    high_quality: bool, false, "Generate more accurate and consistent output.",
    midpoint: f64, 0.5, "The focus region's transition midpoint.",
    radius: f64, 0.75, "The focus region's outer radius.",
    rotation: f64, 0.0, "The rotation of the focus region.",
    shape: FocusBlurShape, FocusBlurShape::Circle, "The blur shape.",
    x: f64, 0.5, "The X coordinate for the center of the blur.",
    y: f64, 0.5, "The Y coordinate for the center of the blur.",
  ),
);

gegl_operation!(
  struct_name: Maze,
  gegl_name: "maze",
  append_crop: false,
  values: (
    algorithm_type: MazeAlgorithmType, MazeAlgorithmType::DepthFirst, "Maze algorithm type",
    bg_color: String, "#fff".to_string(), "The background color.",
    fg_color: String, "#000".to_string(), "The foreground color.",
    seed: f64, 0.0, "The random seed.",
    tileable: bool, false, "Whether the maze should be tileable.",
    x: i64, 16, "Horizontal width of cells pixels.",
    y: i64, 16, "Vertical width of cells pixels.",
  ),
);
