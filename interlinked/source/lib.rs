//! # Interlinked.
//!
//! > **Generative art with GIMP, GEGL and ImageMagick.**

mod cli;
pub mod projects;
pub mod utilities;

use gegl::GeglOperation;

pub use {cli::*, projects::all_projects};

/// An individual project with all the operations to generate it.
#[derive(Debug)]
pub struct Project {
  /// Whether to start from an empty input image.
  ///
  /// Some operations require an input buffer to start from so this will create
  /// an empty image with `imagemagick` and then use that as the input.
  pub create_input_image: bool,

  /// The name of this project.
  pub name: String,

  /// The list of operations.
  pub operations: Vec<Box<dyn GeglOperation>>,

  /// The resolution of the image as a tuple of width and height.
  pub resolution: (i64, i64),

  /// Whether to explicitly turn off the image's alpha channel after finishing
  /// rendering the operations.
  ///
  /// TODO: Explain why this exists after I've figured it out myself again. x)
  pub turn_off_alpha: bool,
}
