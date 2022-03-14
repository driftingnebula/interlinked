import {
  Cartoon,
  DiffractionPatterns,
  Generic,
  MedianBlur,
  Mirrors,
} from '../../gegl/exports.js';
import Project from '../../project.js';

const [width, height] = [3840, 2160];

const project: Project = {
  createInputImage: false,
  name: '2022-03-14',
  operations: [
    new DiffractionPatterns({
      height,
      width,
    }),
    new Mirrors({
      nSegs: 7,
      oX: 0.347,
      oY: 0.1,
      rAngle: 13,
      trimX: 0.051,
      trimY: 0.253,
    }),
    new Cartoon({
      maskRadius: 50,
      pctBlack: 1,
    }),
    new MedianBlur({
      radius: 5,
    }),
    new Generic('gegl:edge', {
      algorithm: 'prewitt',
      amount: 10,
      borderBehavior: 'clamp',
    }),
    new MedianBlur({
      radius: 5,
    }),
  ],
  resetAlpha: false,
  resolution: {
    width,
    height,
  },
};

export default project;
