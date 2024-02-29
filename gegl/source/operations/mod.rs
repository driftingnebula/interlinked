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
    /// Don't over-expose highlights.
    limit_exposure: bool, false,
    /// Glow radius.
    radius: f64, 10.0,
    /// Glow-area edge softness.
    softness: f64, 25.0,
    /// Glow strength.
    strength: f64, 50.0,
    /// Glow-area brightness threshold.
    threshold: f64, 50.0,
  ),
);

gegl_operation!(
  struct_name: Cartoon,
  gegl_name: "cartoon",
  append_crop: true,
  values: (
    /// The mask radius.
    mask_radius: f64, 7.0,
    /// The percentage of black.
    pct_black: f64, 0.2,
  ),
);

gegl_operation!(
  struct_name: CellNoise,
  gegl_name: "cell-noise",
  append_crop: true,
  values: (
    /// The number of noise octaves.
    iterations: i64, 1,
    /// Fill each cell with a random color.
    palettize: bool, false,
    /// Select the n-th closest point
    rank: i64, 1,
    /// The scale of the noise function.
    scale: f64, 1.0,
    /// The random seed for the noise function.
    seed: f64, 0.0,
    /// Interpolate between Manhattan and Euclidean distance.
    shape: f64, 2.0,
  ),
);

gegl_operation!(
  struct_name: Crop,
  gegl_name: "crop",
  append_crop: false,
  values: (
    /// The wanted height of the buffer.
    height: f64, 0.0,
    /// Reset the origin for the coordinates.
    reset_origin: bool, false,
    /// The wanted width of the buffer.
    width: f64, 0.0,
    /// The X coordinate to start from.
    x: f64, 0.0,
    /// The Y coordinate to start from.
    y: f64, 0.0,
  ),
);

gegl_operation!(
  struct_name: DiffractionPatterns,
  gegl_name: "diffraction-patterns",
  append_crop: true,
  values: (
    /// Number of contours (blue);
    blue_contours: f64, 0.97,
    /// Light frequency (blue).
    blue_frequency: f64, 1.12,
    /// Number of sharp edges (blue).
    blue_sedges: f64, 0.64,
    /// Brightness and shifting/fattening of contours.
    brightness: f64, 0.07,
    /// Number of contours (green);
    green_contours: f64, 0.82,
    /// Light frequency (green).
    green_frequency: f64, 1.22,
    /// Number of sharp edges (green).
    green_sedges: f64, 0.68,
    /// Height of the generated buffer.
    height: i64, 200,
    /// Polarization.
    polarization: f64, -0.47,
    /// Number of contours (red);
    red_contours: f64, 0.82,
    /// Light frequency (red).
    red_frequency: f64, 0.81,
    /// Number of sharp edges (red).
    red_sedges: f64, 0.61,
    /// Scattering (speed vs. quality).
    scattering: f64, 37.13,
    /// Width of the generated buffer.
    width: i64, 200,
  ),
);

gegl_operation!(
  struct_name: Edge,
  gegl_name: "edge",
  append_crop: false,
  values: (
    /// The edge detection algorithm to use.
    algorithm: EdgeAlgorithm, EdgeAlgorithm::Sobel,
    /// The amount of edge detection to perform.
    amount: f64, 2.0,
    /// The edge detection border behavior.
    border_behavior: AbyssPolicy, AbyssPolicy::Clamp,
  ),
);

gegl_operation!(
  struct_name: EdgeNeon,
  gegl_name: "edge-neon",
  append_crop: false,
  values: (
    /// Strength of effect.
    amount: f64, 0.0,
    /// Radius of effect in pixels.
    radius: f64, 5.0,
  ),
);

gegl_operation!(
  struct_name: FocusBlur,
  gegl_name: "focus-blur",
  append_crop: false,
  values: (
    /// The aspect ratio of the focus region.
    aspect_ratio: f64, 0.0,
    /// Out-of-focus blur radius.
    blur_radius: f64, 25.0,
    /// The blur type.
    blur_type: FocusBlurType, FocusBlurType::Gaussian,
    /// The focus region's inner limit.
    focus: f64, 0.25,
    /// Relative highlight strength.
    highlight_factor: f64, 0.0,
    /// Highlight threshold (high).
    highlight_threshold_high: f64, 1.0,
    /// Highlight threshold (low).
    highlight_threshold_low: f64, 0.0,
    /// Generate more accurate and consistent output.
    high_quality: bool, false,
    /// The focus region's transition midpoint.
    midpoint: f64, 0.5,
    /// The focus region's outer radius.
    radius: f64, 0.75,
    /// The rotation of the focus region.
    rotation: f64, 0.0,
    /// The blur shape.
    shape: FocusBlurShape, FocusBlurShape::Circle,
    /// The X coordinate for the center of the blur.
    x: f64, 0.5,
    /// The Y coordinate for the center of the blur.
    y: f64, 0.5,
  ),
);

gegl_operation!(
  struct_name: Maze,
  gegl_name: "maze",
  append_crop: false,
  values: (
    /// Maze algorithm type
    algorithm_type: MazeAlgorithmType, MazeAlgorithmType::DepthFirst,
    /// The background color.
    bg_color: String, "#fff".to_string(),
    /// The foreground color.
    fg_color: String, "#000".to_string(),
    /// The random seed.
    seed: f64, 0.0,
    /// Whether the maze should be tileable.
    tileable: bool, false,
    /// Horizontal width of cells pixels.
    x: i64, 16,
    /// Vertical width of cells pixels.
    y: i64, 16,
  ),
);

gegl_operation!(
  struct_name: MedianBlur,
  gegl_name: "median-blur",
  append_crop: false,
  values: (
    /// How image edges are handled.
    abyss_policy: MedianBlurAbyssPolicy, MedianBlurAbyssPolicy::Clamp,
    /// Neighborhood alpha percentile.
    alpha_percentile: f64, 50.0,
    /// Avoid clipping and quantization
    high_precision: bool, false,
    /// Neighborhood type.
    neighborhood: MedianBlurNeighborhood, MedianBlurNeighborhood::Circle,
    /// Neighborhood color percentile.
    percentile: f64, 50.0,
    /// Neighborhood radius, a negative value will calculate with inverted
    /// percentiles.
    radius: f64, 3.0,
  ),
);

gegl_operation!(
  struct_name: Mirrors,
  gegl_name: "mirrors",
  append_crop: false,
  values: (
    /// Clip result to input size.
    clip: bool, true,
    /// X coordinate of symmetry center in output.
    c_x: f64, 0.5,
    /// Y coordinate of symmetry center in output.
    c_y: f64, 0.5,
    /// Scale factor to make rendering size bigger.
    input_scale: f64, 100.0,
    /// Rotation applied to the mirrors.
    m_angle: f64, 0.0,
    /// Number of mirrors to use.
    n_segs: i64, 6,
    /// Scale factor to make rendering size bigger.
    output_scale: f64, 1.0,
    /// X axis ratio for the center of mirroring
    o_x: f64, 0.0,
    /// Y axis ratio for the center of mirroring
    o_y: f64, 0.0,
    /// Rotation applied to the result.
    r_angle: f64, 0.0,
    /// X axis ratio for trimming mirror expanse
    trim_x: f64, 0.0,
    /// Y axis ratio for trimming mirror expanse
    trim_y: f64, 0.0,
    /// Fill full output area.
    warp: bool, true,
  ),
);

gegl_operation!(
  struct_name: Mosaic,
  gegl_name: "mosaic",
  append_crop: false,
  values: (
    /// Enables smoother tile output.
    antialiasing: bool, true,
    /// Tile color based on average of subsumed pixels.
    color_averaging: bool, true,
    /// Magnitude of random color variations.
    color_variation: f64, 0.2,
    /// Joints color.
    joints_color: String, "#000".to_string(),
    /// Light color.
    light_color: String, "#fff".to_string(),
    /// Direction of light-source (in degrees).
    light_dir: f64, 135.0,
    /// Random seed.
    seed: f64, 0.0,
    /// Allows splitting tiles at hard edges.
    tile_allow_split: bool, true,
    /// Apparent height of each tile (in pixels).
    tile_height: f64, 4.0,
    /// Deviation from perfectly formed tiles.
    tile_neatness: f64, 0.65,
    /// Average diameter of each tile (in pixels).
    tile_size: f64, 15.0,
    /// Inter-tile spacing (in pixels).
    tile_spacing: f64, 1.0,
    /// Surface characteristics.
    tile_surface: bool, false,
    /// What shape to use for tiles.
    tile_type: MosaicTileType, MosaicTileType::Hexagons,
  ),
);

gegl_operation!(
  struct_name: Newsprint,
  gegl_name: "newsprint",
  append_crop: false,
  values: (
    /// Number of samples that are averaged for antialiasing the result.
    aa_samples: i64, 16,
    /// Black angle.
    angle: f64, 75.0,
    /// Red and cyan angle.
    angle2: f64, 15.0,
    /// Green and magenta angle.
    angle3: f64, 45.0,
    /// Blue and yellow angle.
    angle4: f64, 0.0,
    /// Multiplication factor for desired rotation of the local space for
    /// texture, the way this is computed makes it weak for desaturated colors
    /// and possibly stronger where there is color.
    angleboost: f64, 0.0,
    /// How much of common gray to pull out of CMY.
    black_pullout: f64, 1.0,
    /// Number of periods per tile, this tiling avoids high frequency anomaly
    /// that angle boost causes.
    blocksize: f64, -1.0,
    /// How many inks to use.
    color_model: NewsprintColorModel, NewsprintColorModel::BlackOnWhite,
    /// Black halftoning/dot pattern to use.
    pattern: NewsprintPattern, NewsprintPattern::Line,
    /// Red and cyan halftoning/dot pattern to use.
    pattern2: NewsprintPattern, NewsprintPattern::Line,
    /// Green and magenta halftoning/dot pattern to use.
    pattern3: NewsprintPattern, NewsprintPattern::Line,
    /// Blue and yellow halftoning/dot pattern to use.
    pattern4: NewsprintPattern, NewsprintPattern::Line,
    /// Black number of pixels across one repetition of a base pattern at base
    /// resolution.
    period: f64, 12.0,
    /// Red and cyan number of pixels across one repetition of a base pattern at
    /// base resolution.
    period2: f64, 12.0,
    /// Green and magenta number of pixels across one repetition of a base pattern
    /// at base resolution.
    period3: f64, 12.0,
    /// Blue and yellow number of pixels across one repetition of a base pattern
    /// at base resolution.
    period4: f64, 12.0,
    /// Color saturation dependent compression of period.
    turbulence: f64, 0.0,
  ),
);

gegl_operation!(
  struct_name: NoisePick,
  gegl_name: "noise-pick",
  append_crop: true,
  values: (
    /// Randomization percentage.
    pct_random: f64, 50.0,
    /// Amount of repetitions to make.
    repeat: i64, 1,
    /// Random seed.
    seed: f64, 0.0,
  ),
);

gegl_operation!(
  struct_name: Oilify,
  gegl_name: "oilify",
  append_crop: false,
  values: (
    /// Exponent for processing, controls smoothness.
    exponent: i64, 8,
    /// Histogram size.
    intensities: i64, 128,
    /// Radius of circle around pixel.
    mask_radius: i64, 4,
    /// Use pixel luminance values.
    use_inten: bool, true,
  ),
);

gegl_operation!(
  struct_name: Plasma,
  gegl_name: "plasma",
  append_crop: false,
  values: (
    /// Height of the generated buffer.
    height: i64, 768,
    /// Random seed.
    seed: f64, 0.0,
    /// High values give more variation in details.
    turbulence: f64, 1.0,
    /// Width of the generated buffer.
    width: i64, 1024,
    /// X coordinate start of the generated buffer.
    x: i64, 0,
    /// Y coordinate start of the generated buffer.
    y: i64, 0,
  ),
);

gegl_operation!(
  struct_name: SimplexNoise,
  gegl_name: "simplex-noise",
  append_crop: true,
  values: (
    /// The number of noise octaves.
    iterations: i64, 1,
    /// The scale of the noise function.
    scale: f64, 1.0,
    /// The random seed for the noise function.
    seed: f64, 1.0,
  ),
);

gegl_operation!(
  struct_name: Softglow,
  gegl_name: "softglow",
  append_crop: false,
  values: (
    /// Brightness intensity.
    brightness: f64, 0.3,
    /// Glow radius.
    glow_radius: f64, 10.0,
    /// Sharpness of the highlights.
    sharpness: f64, 0.85,
  ),
);

gegl_operation!(
  struct_name: Spiral,
  gegl_name: "spiral",
  append_crop: true,
  values: (
    /// The area balance between the two colors.
    balance: f64, 0.0,
    /// The logarithmic spiral base.
    base: f64, 2.0,
    /// The primary color.
    color1: String, "#000".to_string(),
    /// The secondary color.
    color2: String, "#fff".to_string(),
    /// The spiral swirl direction.
    direction: SpiralDirection, SpiralDirection::Clockwise,
    /// The height of the generated buffer.
    height: i64, 768,
    /// The spiral radius.
    radius: f64, 100.0,
    /// The spiral rotation.
    rotation: f64, 0.0,
    /// The spiral type.
    spiral_type: SpiralType, SpiralType::Linear,
    /// The X origin coordinate.
    x: f64, 0.5,
    /// The Y origin coordinate.
    y: f64, 0.5,
    /// The width of the generated buffer.
    width: i64, 1024,
  ),
);

gegl_operation!(
  struct_name: StereographicProjection,
  gegl_name: "stereographic-projection",
  append_crop: false,
  values: (
    /// Output/rendering height in pixels, -1 for input height.
    height: i64, -1,
    /// Do the inverse mapping.
    inverse: bool, false,
    /// Horizontal camera panning.
    pan: f64, 0.0,
    /// Image resampling method to use.
    sampler_type: StereographicProjectionSamplerType, StereographicProjectionSamplerType::Nearest,
    /// Spin angle around camera axis.
    spin: f64, 0.,
    /// Vertical camera panning.
    tilt: f64, 90.,
    /// Output/rendering width in pixels, -1 for input width.
    width: i64, -1,
    /// Zoom level.
    zoom: f64, 100.0,
  ),
);

gegl_operation!(
  struct_name: TileGlass,
  gegl_name: "tile-glass",
  append_crop: false,
  values: (
    /// Tile height.
    tile_height: i64, 25,
    /// Tile width.
    tile_width: i64, 25,
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
    /// How to fill superpixels.
    fill: WaterpixelsFill, WaterpixelsFill::Average,
    /// Spatial regularization, trade-off between superpixel regularity and
    /// adherence to object boundaries.
    regularization: i64, 0,
    /// Superpixels size.
    size: i64, 32,
    /// Gradient smoothness.
    smoothness: f64, 1.0,
  ),
);

gegl_operation!(
  struct_name: Waves,
  gegl_name: "waves",
  append_crop: true,
  values: (
    /// Amplitude of the wave ripples.
    amplitude: f64, 25.0,
    /// Aspect ratio.
    aspect: f64, 1.0,
    /// Limit deformation in the image area.
    clamp: bool, false,
    /// Period/wavelength of the ripples.
    period: f64, 100.0,
    /// Phase shift of the waves.
    phi: f64, 0.0,
    /// Mathematical method for reconstructing pixel values.
    sampler_type: WavesSamplerType, WavesSamplerType::Cubic,
    /// Center X coordinate to start the waves from.
    x: f64, 0.5,
    /// Center Y coordinate to start the waves from.
    y: f64, 0.5,
  ),
);
