import {
  DiffractionPatterns,
  Generic,
  MedianBlur,
  Mirrors,
  Newsprint,
  Waterpixels,
} from '../../gegl/exports.js';
import Project from '../../project.js';

const [width, height] = [3840, 2160];

const project: Project = {
  createInputImage: false,
  name: '2022-03-12',
  operations: [
    new DiffractionPatterns({
      brightness: 0.382,
      polarization: 0.67,
      scattering: 53.21,
      height,
      width,
    }),
    new Waterpixels({
      smoothness: 2,
    }),
    new Generic('gegl:edge-neon', {
      amount: 0.5,
      radius: 3,
    }),
    new Mirrors({
      oX: 0.365,
      oY: 0.694,
      trimX: 0.375,
      trimY: 0.375,
    }),
    new Newsprint({
      turbulence: 0.996,
    }),
    new MedianBlur({
      neighborhood: 'diamond',
    }),
  ],
  resetAlpha: false,
  resolution: {
    width,
    height,
  },
};

export default project;
