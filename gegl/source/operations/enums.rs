//! Additional enums for the operations.

use crate::gegl_enum;

gegl_enum!(
  FocusBlurShape,
  Circle => "circle",
  Square => "square",
  Diamond => "diamond",
  Horizontal => "horizontal",
  Vertical => "vertical",
);

gegl_enum!(
  FocusBlurType,
  Gaussian => "gaussian",
  Lens => "lens",
);

gegl_enum!(
  MazeAlgorithmType,
  DepthFirst => "depth-first",
  Prim => "prim",
);

gegl_enum!(
  MedianBlurAbyssPolicy,
  None => "none",
  Clamp => "clamp",
);

gegl_enum!(
  MedianBlurNeighborhood,
  Square => "square",
  Circle => "circle",
  Diamond => "diamond",
);

gegl_enum!(
  MosaicTileType,
  Squares => "squares",
  Hexagons => "hexagons",
  Octagons => "octagons",
  Triangles => "triangles",
);

gegl_enum!(
  NewsprintPattern,
  Line => "line",
  Circle => "circle",
  Diamond => "diamond",
  Pssquare => "pssquare",
  Cross => "cross",
);

gegl_enum!(
  NewsprintColorModel,
  BlackOnWhite => "black-on-white",
  Cmyk => "cmyk",
  Rgb => "rgb",
  WhiteOnBlack => "white-on-black",
);
