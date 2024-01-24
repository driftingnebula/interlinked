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
