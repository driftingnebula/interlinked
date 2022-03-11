import {
  FocusBlur,
  Mirrors,
  Newsprint,
  SimplexNoise,
  Softglow,
  StereographicProjection,
} from '../../gegl/exports.js';
import Project from '../../project.js';

const [width, height] = [1920, 1080];

const project: Project = {
  createInputImage: false,
  name: '2022-03-06',
  operations: [
    new SimplexNoise({scale: 4, seed: 2_071_140_406}),
    new Newsprint({
      colorModel: 'rgb',
      pattern2: 'line',
      period2: 200,
      angle2: 15,
      pattern3: 'line',
      period3: 200,
      angle3: 45,
      pattern4: 'line',
      period4: 200,
      angle4: 0,
    }),
    new Mirrors(),
    new Softglow(),
    new Newsprint(),
    new StereographicProjection({tilt: 123}),
    new FocusBlur({
      blurType: 'gaussian',
      blurRadius: 11.5,
      radius: 0.9,
      midpoint: 0.6,
    }),
    new Newsprint({
      colorModel: 'rgb',
      pattern2: 'diamond',
      period2: 200,
      angle2: 0,
      pattern3: 'diamond',
      period3: 200,
      angle3: 35,
      pattern4: 'diamond',
      period4: 200,
      angle4: 55,
    }),
    new FocusBlur({
      blurType: 'gaussian',
      blurRadius: 11.5,
      radius: 0.9,
      midpoint: 0.6,
    }),
  ],
  resetAlpha: false,
  resolution: {
    width,
    height,
  },
};

export default project;
