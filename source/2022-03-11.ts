import {
  Cartoon,
  Crop,
  Generic,
  Newsprint,
  Plasma,
  Waterpixels,
} from './gegl/exports.js';
import Project from './project.js';

const [width, height] = [3840, 2160];

const project: Project = {
  createInputImage: true,
  name: '2022-03-11',
  operations: [
    new Plasma({
      seed: 168_139_081,
      turbulence: 1.5,
      height,
      width,
    }),
    new Cartoon({
      maskRadius: 40,
      pctBlack: 0.2,
    }),
    new Waterpixels({
      size: 64,
    }),
    new Cartoon({
      maskRadius: 40,
      pctBlack: 0.2,
    }),
    new Newsprint({
      colorModel: 'rgb',
      pattern2: 'circle',
      pattern3: 'cross',
      pattern4: 'circle',
      period2: 15,
      period3: 150,
      period4: 30,
      turbulence: 0.9,
    }),
    new Generic('gegl:bloom', {
      radius: 7.48,
      strength: 115.29,
      threshold: 65.88,
    }),
    new Generic('gegl:noise-pick', {
      repeat: 5,
      seed: 0,
    }),
    new Crop({height, width}),
  ],
  resetAlpha: false,
  resolution: {
    width,
    height,
  },
};

export default project;
