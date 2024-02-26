//! Projects made in 2022.

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

/// The project made on 2022-03-07.
pub fn day_2022_03_07() -> Project {
  Project {
    create_input_image: false,
    name: "2022-03-07".to_string(),
    operations: vec![
      Plasma::default()
        .with_height(1080)
        .with_seed(2_000_111_903.0)
        .with_turbulence(1.0)
        .with_width(1920)
        .boxed(),
      Mosaic::default()
        .with_color_variation(1.0)
        .with_tile_height(5.0)
        .with_tile_neatness(1.0)
        .with_tile_size(116.53)
        .with_tile_surface(true)
        .with_tile_type(MosaicTileType::Triangles)
        .boxed(),
      Waves::default()
        .with_amplitude(2.9)
        .with_clamp(true)
        .with_sampler_type(WavesSamplerType::Cubic)
        .boxed(),
      Waves::default()
        .with_amplitude(17.3)
        .with_clamp(true)
        .with_sampler_type(WavesSamplerType::Cubic)
        .boxed(),
      Mirrors::default()
        .with_o_x(1.0)
        .with_o_y(0.353)
        .with_n_segs(2)
        .boxed(),
      Cartoon::default().boxed(),
      Waterpixels::default()
        .with_fill(WaterpixelsFill::Average)
        .with_size(32)
        .with_smoothness(1.0)
        .boxed(),
      Mirrors::default()
        .with_o_x(0.01)
        .with_o_y(0.01)
        .with_n_segs(5)
        .with_r_angle(342.0)
        .boxed(),
      MedianBlur::default().boxed(),
    ],
    resolution: (1920, 1080),
    turn_off_alpha: false,
  }
}

/// The project made on 2022-03-08.
pub fn day_2022_03_08() -> Project {
  Project {
    create_input_image: false,
    name: "2022-03-08".to_string(),
    operations: vec![
      DiffractionPatterns::default()
        .with_height(1080)
        .with_width(1920)
        .boxed(),
      TileSeamless::default().boxed(),
      StereographicProjection::default().boxed(),
      Newsprint::default()
        .with_color_model(NewsprintColorModel::Cmyk)
        .with_period(4.0)
        .boxed(),
      FocusBlur::default()
        .with_blur_radius(9.72)
        .with_blur_type(FocusBlurType::Lens)
        .with_focus(0.0)
        .with_highlight_factor(0.924)
        .with_radius(1.173)
        .with_shape(FocusBlurShape::Circle)
        .boxed(),
    ],
    resolution: (1920, 1080),
    turn_off_alpha: false,
  }
}
