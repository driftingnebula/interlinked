import {Bloom, CellNoise, Crop, Newsprint, Waves} from '../../gegl/exports.js';
import Project from '../../project.js';

const [width, height] = [3840, 2160];

const project: Project = {
  createInputImage: false,
  name: '2022-03-10',
  operations: [
    new CellNoise({
      scale: 0.5,
      seed: 2_762_328_325,
    }),
    new Crop({height, width}),
    new Newsprint({
      angle4: 75.85,
      colorModel: 'rgb',
      pattern2: 'circle',
      pattern4: 'cross',
      period2: 42.38,
      period3: 0,
      period4: 135.1,
      turbulence: 0.454,
    }),
    new Waves({
      amplitude: 67.6,
      clamp: true,
      period: 514.8,
      phi: -0.529,
      samplerType: 'cubic',
      x: -0.25,
      y: -0.75,
    }),
    new Crop({height, width}),
    new Bloom({
      radius: 20,
      softness: 57,
      strength: 90,
      threshold: 10,
    }),
  ],
  resetAlpha: false,
  resolution: {
    width,
    height,
  },
};

export default project;
