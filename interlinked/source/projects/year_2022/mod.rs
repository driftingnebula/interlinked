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

/// The project made on 2022-03-09.
pub fn day_2022_03_09() -> Project {
  Project {
    create_input_image: true,
    name: "2022-03-09".to_string(),
    operations: vec![
      Maze::default().boxed(),
      TileGlass::default().boxed(),
      Waterpixels::default().boxed(),
      Newsprint::default()
        .with_angle2(-55.4)
        .with_angle3(60.77)
        .with_angle4(103.55)
        .with_color_model(NewsprintColorModel::Rgb)
        .boxed(),
      Waves::default()
        .with_amplitude(5.9)
        .with_clamp(true)
        .boxed(),
      Oilify::default().boxed(),
      TileSeamless::default().boxed(),
      MedianBlur::default().with_percentile(2.35).boxed(),
      Mirrors::default()
        .with_n_segs(3)
        .with_o_x(0.1)
        .with_o_y(0.2)
        .with_r_angle(330.0)
        .boxed(),
      FocusBlur::default()
        .with_blur_radius(33.57)
        .with_blur_type(FocusBlurType::Lens)
        .with_focus(0.111)
        .with_highlight_factor(0.529)
        .with_radius(1.173)
        .boxed(),
    ],
    resolution: (1920, 1080),
    turn_off_alpha: true,
  }
}

/// The project made on 2022-03-10.
pub fn day_2022_03_10() -> Project {
  Project {
    create_input_image: false,
    name: "2022-03-10".to_string(),
    operations: vec![
      CellNoise::default()
        .with_scale(0.5)
        .with_seed(2_762_328_325.0)
        .boxed(),
      Newsprint::default()
        .with_angle4(75.85)
        .with_color_model(NewsprintColorModel::Rgb)
        .with_pattern2(NewsprintPattern::Circle)
        .with_pattern4(NewsprintPattern::Cross)
        .with_period2(42.38)
        .with_period3(0.0)
        .with_period4(135.1)
        .with_turbulence(0.454)
        .boxed(),
      Waves::default()
        .with_amplitude(67.6)
        .with_clamp(true)
        .with_period(514.8)
        .with_phi(-0.529)
        .with_sampler_type(WavesSamplerType::Cubic)
        .with_x(-0.25)
        .with_y(-0.75)
        .boxed(),
      Bloom::default()
        .with_radius(20.0)
        .with_softness(57.0)
        .with_strength(90.0)
        .with_threshold(10.0)
        .boxed(),
    ],
    resolution: (3840, 2160),
    turn_off_alpha: false,
  }
}

/// The project made on 2022-03-11.
pub fn day_2022_03_11() -> Project {
  Project {
    create_input_image: true,
    name: "2022-03-11".to_string(),
    operations: vec![
      Plasma::default()
        .with_seed(168_139_081.0)
        .with_turbulence(1.5)
        .with_height(2160)
        .with_width(3840)
        .boxed(),
      Cartoon::default()
        .with_mask_radius(40.0)
        .with_pct_black(0.2)
        .boxed(),
      Waterpixels::default().with_size(64).boxed(),
      Cartoon::default()
        .with_mask_radius(40.0)
        .with_pct_black(0.2)
        .boxed(),
      Newsprint::default()
        .with_color_model(NewsprintColorModel::Rgb)
        .with_pattern2(NewsprintPattern::Circle)
        .with_pattern3(NewsprintPattern::Cross)
        .with_pattern4(NewsprintPattern::Circle)
        .with_period2(15.0)
        .with_period3(150.0)
        .with_period4(30.0)
        .with_turbulence(0.9)
        .boxed(),
      Bloom::default()
        .with_radius(7.48)
        .with_strength(115.29)
        .with_threshold(65.88)
        .boxed(),
      NoisePick::default().with_repeat(5).with_seed(0.0).boxed(),
    ],
    resolution: (3840, 2160),
    turn_off_alpha: false,
  }
}

/// The project made on 2022-03-12.
pub fn day_2022_03_12() -> Project {
  Project {
    create_input_image: false,
    name: "2022-03-12".to_string(),
    operations: vec![
      DiffractionPatterns::default()
        .with_brightness(0.382)
        .with_polarization(0.67)
        .with_scattering(53.21)
        .with_height(2160)
        .with_width(3840)
        .boxed(),
      Waterpixels::default().with_smoothness(2.0).boxed(),
      EdgeNeon::default()
        .with_amount(0.5)
        .with_radius(3.0)
        .boxed(),
      Mirrors::default()
        .with_o_x(0.365)
        .with_o_y(0.694)
        .with_trim_x(0.375)
        .with_trim_y(0.375)
        .boxed(),
      Newsprint::default().with_turbulence(0.996).boxed(),
      MedianBlur::default()
        .with_neighborhood(MedianBlurNeighborhood::Diamond)
        .boxed(),
    ],
    resolution: (3840, 2160),
    turn_off_alpha: false,
  }
}

/// The project made on 2022-03-13.
pub fn day_2022_03_13() -> Project {
  Project {
    create_input_image: false,
    name: "2022-03-13".to_string(),
    operations: vec![
      Plasma::default()
        .with_seed(65_198_886.0)
        .with_height(2160)
        .with_width(3840)
        .boxed(),
      Cartoon::default()
        .with_mask_radius(50.0)
        .with_pct_black(1.0)
        .boxed(),
      Waterpixels::default().with_size(64).boxed(),
      Oilify::default().with_mask_radius(8).boxed(),
      Mirrors::default()
        .with_n_segs(8)
        .with_o_x(0.829)
        .with_o_y(0.812)
        .with_trim_x(0.325)
        .with_trim_y(0.09)
        .boxed(),
      StereographicProjection::default().with_tilt(-73.42).boxed(),
      Oilify::default().with_mask_radius(8).boxed(),
    ],
    resolution: (3840, 2160),
    turn_off_alpha: false,
  }
}

/// The project made on 2022-03-14.
pub fn day_2022_03_14() -> Project {
  Project {
    create_input_image: false,
    name: "2022-03-14".to_string(),
    operations: vec![
      DiffractionPatterns::default()
        .with_height(2160)
        .with_width(3840)
        .boxed(),
      Mirrors::default()
        .with_n_segs(7)
        .with_o_x(0.347)
        .with_o_y(0.1)
        .with_r_angle(13.0)
        .with_trim_x(0.051)
        .with_trim_y(0.253)
        .boxed(),
      Cartoon::default()
        .with_mask_radius(50.0)
        .with_pct_black(1.0)
        .boxed(),
      MedianBlur::default().with_radius(5.0).boxed(),
      Edge::default()
        .with_algorithm(EdgeAlgorithm::PrewittCompass)
        .with_amount(10.0)
        .with_border_behavior(AbyssPolicy::Clamp)
        .boxed(),
      MedianBlur::default().with_radius(5.0).boxed(),
    ],
    resolution: (3840, 2160),
    turn_off_alpha: false,
  }
}

/// The project made on 2022-03-15.
pub fn day_2022_03_15() -> Project {
  Project {
    create_input_image: false,
    name: "2022-03-15".to_string(),
    operations: vec![
      Spiral::default()
        .with_balance(0.424)
        .with_color1("#3078d2".to_string())
        .with_color2("#ff00dc".to_string())
        .with_direction(SpiralDirection::Clockwise)
        .with_height(2160)
        .with_radius(918.7)
        .with_spiral_type(SpiralType::Linear)
        .with_width(3840)
        .with_x(0.307)
        .with_y(0.542)
        .boxed(),
      Cartoon::default()
        .with_mask_radius(50.0)
        .with_pct_black(1.0)
        .boxed(),
      Newsprint::default()
        .with_color_model(NewsprintColorModel::Rgb)
        .with_pattern2(NewsprintPattern::Line)
        .with_pattern3(NewsprintPattern::Diamond)
        .with_pattern4(NewsprintPattern::Pssquare)
        .with_period2(105.96)
        .with_period3(30.46)
        .with_period4(125.83)
        .with_turbulence(0.182)
        .boxed(),
      Bloom::default()
        .with_radius(2.08)
        .with_softness(87.94)
        .with_strength(165.29)
        .boxed(),
      Cartoon::default()
        .with_mask_radius(50.0)
        .with_pct_black(1.0)
        .boxed(),
      Waves::default()
        .with_amplitude(65.0)
        .with_clamp(true)
        .with_period(500.0)
        .with_phi(0.5)
        .with_x(0.0)
        .with_y(0.1)
        .boxed(),
      Mirrors::default()
        .with_r_angle(30.0)
        .with_n_segs(3)
        .with_o_x(0.312)
        .with_o_y(1.0)
        .with_trim_x(0.162)
        .with_trim_y(0.031)
        .boxed(),
      FocusBlur::default()
        .with_blur_radius(5.4)
        .with_blur_type(FocusBlurType::Lens)
        .with_focus(0.154)
        .with_highlight_factor(0.75)
        .with_midpoint(0.38)
        .with_radius(0.802)
        .boxed(),
    ],
    resolution: (3840, 2160),
    turn_off_alpha: false,
  }
}
