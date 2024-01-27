//! Additional enums for the operations.

use crate::gegl_enum;

gegl_enum!(
  /// The shape for [`FocusBlur`][super::FocusBlur].
  FocusBlurShape,
  Circle => "circle",
  Square => "square",
  Diamond => "diamond",
  Horizontal => "horizontal",
  Vertical => "vertical",
);

gegl_enum!(
  /// The type for [`FocusBlur`][super::FocusBlur].
  FocusBlurType,
  Gaussian => "gaussian",
  Lens => "lens",
);

gegl_enum!(
  /// The algorithm type for [`Maze`][super::Maze].
  MazeAlgorithmType,
  DepthFirst => "depth-first",
  Prim => "prim",
);

gegl_enum!(
  /// The abyss policy for [`MedianBlur`][super::MedianBlur].
  MedianBlurAbyssPolicy,
  None => "none",
  Clamp => "clamp",
);

gegl_enum!(
  /// The neighborhood for [`MedianBlur`][super::MedianBlur].
  MedianBlurNeighborhood,
  Square => "square",
  Circle => "circle",
  Diamond => "diamond",
);

gegl_enum!(
  /// The tile type for [`Mosaic`][super::Mosaic].
  MosaicTileType,
  Squares => "squares",
  Hexagons => "hexagons",
  Octagons => "octagons",
  Triangles => "triangles",
);

gegl_enum!(
  /// The pattern for [`Newsprint`][super::Newsprint].
  NewsprintPattern,
  Line => "line",
  Circle => "circle",
  Diamond => "diamond",
  Pssquare => "pssquare",
  Cross => "cross",
);

gegl_enum!(
  /// The color model for [`Newsprint`][super::Newsprint].
  NewsprintColorModel,
  BlackOnWhite => "black-on-white",
  Cmyk => "cmyk",
  Rgb => "rgb",
  WhiteOnBlack => "white-on-black",
);

gegl_enum!(
  /// The sampler type for [`StereographicProjection`][super::StereographicProjection].
  StereographicProjectionSamplerType,
  Nearest => "nearest",
  Linear => "linear",
  Cubic => "cubic",
  Nohalo => "nohalo",
  Lohalo => "lohalo",
);

gegl_enum!(
  /// The fill for [`Waterpixels`][super::Waterpixels].
  WaterpixelsFill,
  Average => "average",
  Random => "random",
);

gegl_enum!(
  /// The sampler type for [`Waves`][super::Waves].
  WavesSamplerType,
  Nearest => "nearest",
  Linear => "linear",
  Cubic => "cubic",
  Nohalo => "nohalo",
  Lohalo => "lohalo",
);
