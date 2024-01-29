//! The project made on 2022-03-06.

use {crate::Project, gegl::*};

/// The project made on 2022-03-06.
pub fn day_2022_03_06() -> Project {
  Project {
    create_input_image: false,
    name: "2022-03-06".to_string(),
    operations: vec![
      SimplexNoise::default()
        .with_scale(4.0)
        .with_seed(2_071_140_406.0)
        .boxed(),
      Newsprint::default()
        .with_color_model(NewsprintColorModel::Rgb)
        .with_pattern2(NewsprintPattern::Line)
        .with_period2(200.0)
        .with_angle2(15.0)
        .with_pattern3(NewsprintPattern::Line)
        .with_period3(200.0)
        .with_angle3(45.0)
        .with_pattern4(NewsprintPattern::Line)
        .with_period4(200.0)
        .with_angle4(0.0)
        .boxed(),
      Mirrors::default().boxed(),
      Softglow::default().boxed(),
      Newsprint::default().boxed(),
      StereographicProjection::default().with_tilt(123.0).boxed(),
      FocusBlur::default()
        .with_blur_radius(11.5)
        .with_blur_type(FocusBlurType::Gaussian)
        .with_midpoint(0.6)
        .with_radius(0.9)
        .boxed(),
      Newsprint::default()
        .with_color_model(NewsprintColorModel::Rgb)
        .with_pattern2(NewsprintPattern::Diamond)
        .with_period2(200.0)
        .with_angle2(0.0)
        .with_pattern3(NewsprintPattern::Diamond)
        .with_period3(200.0)
        .with_angle3(35.0)
        .with_pattern4(NewsprintPattern::Diamond)
        .with_period4(200.0)
        .with_angle4(55.0)
        .boxed(),
      FocusBlur::default()
        .with_blur_radius(11.5)
        .with_blur_type(FocusBlurType::Gaussian)
        .with_midpoint(0.6)
        .with_radius(0.9)
        .boxed(),
    ],
    resolution: (1920, 1080),
    turn_off_alpha: false,
  }
}
