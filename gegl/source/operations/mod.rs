//! All supported GEGL operations.

mod enums;
mod macros;

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

gegl_operation!(
  struct_name: MedianBlur,
  gegl_name: "median-blur",
  append_crop: false,
  values: (
    abyss_policy: MedianBlurAbyssPolicy, MedianBlurAbyssPolicy::Clamp, "How image edges are handled.",
    alpha_percentile: f64, 50.0, "Neighborhood alpha percentile.",
    high_precision: bool, false, "Avoid clipping and quantization",
    neighborhood: MedianBlurNeighborhood, MedianBlurNeighborhood::Circle, "Neighborhood type.",
    percentile: f64, 50.0, "Neighborhood color percentile.",
    radius: f64, 3.0, "Neighborhood radius, a negative value will calculate with inverted percentiles.",
  ),
);

gegl_operation!(
  struct_name: Mirrors,
  gegl_name: "mirrors",
  append_crop: false,
  values: (
    clip: bool, true, "Clip result to input size.",
    c_x: f64, 0.5, "X coordinate of symmetry center in output.",
    c_y: f64, 0.5, "Y coordinate of symmetry center in output.",
    input_scale: f64, 100.0, "Scale factor to make rendering size bigger.",
    m_angle: f64, 0.0, "Rotation applied to the mirrors.",
    n_segs: i64, 6, "Number of mirrors to use.",
    output_scale: f64, 1.0, "Scale factor to make rendering size bigger.",
    o_x: f64, 0.0, "X axis ratio for the center of mirroring",
    o_y: f64, 0.0, "Y axis ratio for the center of mirroring",
    r_angle: f64, 0.0, "Rotation applied to the result.",
    trim_x: f64, 0.0, "X axis ratio for trimming mirror expanse",
    trim_y: f64, 0.0, "Y axis ratio for trimming mirror expanse",
    warp: bool, true, "Fill full output area.",
  ),
);

gegl_operation!(
  struct_name: Mosaic,
  gegl_name: "mosaic",
  append_crop: false,
  values: (
    antialiasing: bool, true, "Enables smoother tile output.",
    color_averaging: bool, true, "Tile color based on average of subsumed pixels.",
    color_variation: f64, 0.2, "Magnitude of random color variations.",
    joints_color: String, "#000".to_string(), "Joints color.",
    light_color: String, "#fff".to_string(), "Light color.",
    light_dir: f64, 135.0, "Direction of light-source (in degrees).",
    seed: f64, 0.0, "Random seed.",
    tile_allow_split: bool, true, "Allows splitting tiles at hard edges.",
    tile_height: f64, 4.0, "Apparent height of each tile (in pixels).",
    tile_neatness: f64, 0.65, "Deviation from perfectly formed tiles.",
    tile_size: f64, 15.0, "Average diameter of each tile (in pixels).",
    tile_spacing: f64, 1.0, "Inter-tile spacing (in pixels).",
    tile_surface: bool, false, "Surface characteristics.",
    tile_type: MosaicTileType, MosaicTileType::Hexagons, "What shape to use for tiles.",
  ),
);

gegl_operation!(
  struct_name: Newsprint,
  gegl_name: "newsprint",
  append_crop: false,
  values: (
    aa_samples: i64, 16, "Number of samples that are averaged for antialiasing the result.",
    angle: f64, 75.0, "Black angle.",
    angle2: f64, 15.0, "Red and cyan angle.",
    angle3: f64, 45.0, "Green and magenta angle.",
    angle4: f64, 0.0, "Blue and yellow angle.",
    angleboost: f64, 0.0, "Multiplication factor for desired rotation of the local space for texture, the way this is computed makes it weak for desaturated colors and possibly stronger where there is color.",
    black_pullout: f64, 1.0, "How much of common gray to pull out of CMY.",
    blocksize: f64, -1.0, "Number of periods per tile, this tiling avoids high frequency anomaly that angle boost causes.",
    color_model: NewsprintColorModel, NewsprintColorModel::BlackOnWhite, "How many inks to use.",
    pattern: NewsprintPattern, NewsprintPattern::Line, "Black halftoning/dot pattern to use.",
    pattern2: NewsprintPattern, NewsprintPattern::Line, "Red and cyan halftoning/dot pattern to use.",
    pattern3: NewsprintPattern, NewsprintPattern::Line, "Green and magenta halftoning/dot pattern to use.",
    pattern4: NewsprintPattern, NewsprintPattern::Line, "Blue and yellow halftoning/dot pattern to use.",
    period: f64, 12.0, "Black number of pixels across one repetition of a base pattern at base resolution.",
    period2: f64, 12.0, "Red and cyan number of pixels across one repetition of a base pattern at base resolution.",
    period3: f64, 12.0, "Green and magenta number of pixels across one repetition of a base pattern at base resolution.",
    period4: f64, 12.0, "Blue and yellow number of pixels across one repetition of a base pattern at base resolution.",
    turbulence: f64, 0.0, "Color saturation dependent compression of period.",
  ),
);

gegl_operation!(
  struct_name: NoisePick,
  gegl_name: "noise-pick",
  append_crop: true,
  values: (
    pct_random: f64, 50.0, "Randomization percentage.",
    repeat: i64, 1, "Amount of repetitions to make.",
    seed: f64, 0.0, "Random seed.",
  ),
);

gegl_operation!(
  struct_name: Oilify,
  gegl_name: "oilify",
  append_crop: false,
  values: (
    exponent: i64, 8, "Exponent for processing, controls smoothness.",
    intensities: i64, 128, "Histogram size.",
    mask_radius: i64, 4, "Radius of circle around pixel.",
    use_inten: bool, true, "Use pixel luminance values.",
  ),
);

gegl_operation!(
  struct_name: Plasma,
  gegl_name: "plasma",
  append_crop: false,
  values: (
    height: i64, 768, "Height of the generated buffer",
    seed: f64, 0.0, "Random seed.",
    turbulence: f64, 1.0, "High values give more variation in details.",
    width: i64, 1024, "Width of the generated buffer.",
    x: i64, 0, "X coordinate start of the generated buffer.",
    y: i64, 0, "Y coordinate start of the generated buffer.",
  ),
);

gegl_operation!(
  struct_name: SimplexNoise,
  gegl_name: "simplex-noise",
  append_crop: true,
  values: (
    iterations: i64, 1, "The number of noise octaves.",
    scale: f64, 1.0, "The scale of the noise function.",
    seed: f64, 1.0, "The random seed for the noise function.",
  ),
);

gegl_operation!(
  struct_name: Softglow,
  gegl_name: "softglow",
  append_crop: false,
  values: (
    brightness: f64, 0.3, "Brightness intensity.",
    glow_radius: f64, 10.0, "Glow radius.",
    sharpness: f64, 0.85, "Sharpness of the highlights.",
  ),
);

gegl_operation!(
  struct_name: StereographicProjection,
  gegl_name: "stereographic-projection",
  append_crop: false,
  values: (
    height: i64, -1, "Output/rendering height in pixels, -1 for input height.",
    inverse: bool, false, "Do the inverse mapping.",
    pan: f64, 0.0, "Horizontal camera panning.",
    sampler_type: StereographicProjectionSamplerType, StereographicProjectionSamplerType::Nearest, "Image resampling method to use.",
    spin: f64, 0., "Spin angle around camera axis.",
    tilt: f64, 90., "Vertical camera panning.",
    width: i64, -1, "Output/rendering width in pixels, -1 for input width.",
    zoom: f64, 100.0, "Zoom level.",
  ),
);

gegl_operation!(
  struct_name: TileGlass,
  gegl_name: "tile-glass",
  append_crop: false,
  values: (
    tile_height: i64, 25, "Tile height.",
    tile_width: i64, 25, "Tile width.",
  ),
);

gegl_operation!(
  struct_name: TileSeamless,
  gegl_name: "tile-seamless",
  append_crop: false,
  values: (,),
);

gegl_operation!(
  struct_name: Waterpixels,
  gegl_name: "waterpixels",
  append_crop: false,
  values: (
    fill: WaterpixelsFill, WaterpixelsFill::Average, "How to fill superpixels.",
    regularization: i64, 0, "Spatial regularization, trade-off between superpixel regularity and adherence to object boundaries.",
    size: i64, 32, "Superpixels size.",
    smoothness: f64, 1.0, "Gradient smoothness.",
  ),
);

gegl_operation!(
  struct_name: Waves,
  gegl_name: "waves",
  append_crop: true,
  values: (
    amplitude: f64, 25.0, "Amplitude of the wave ripples.",
    aspect: f64, 1.0, "Aspect ratio.",
    clamp: bool, false, "Limit deformation in the image area.",
    period: f64, 100.0, "Period/wavelength of the ripples.",
    phi: f64, 0.0, "Phase shift of the waves.",
    sampler_type: WavesSamplerType, WavesSamplerType::Cubic, "Mathematical method for reconstructing pixel values.",
    x: f64, 0.5, "Center X coordinate to start the waves from.",
    y: f64, 0.5, "Center Y coordinate to start the waves from.",
  ),
);
