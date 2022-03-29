import {
  Bloom,
  Cartoon,
  FocusBlur,
  Mirrors,
  Oilify,
  Plasma,
} from '../../gegl/exports.js';
import Project from '../../project.js';

const [width, height] = [3840, 2160];

const project: Project = {
  createInputImage: false,
  name: '2022-03-29',
  operations: [
    new Plasma({
      height,
      seed: 3_474_742_930,
      turbulence: 2,
      width,
    }),
    new Cartoon({
      maskRadius: 50,
      pctBlack: 1,
    }),
    new FocusBlur({
      blurType: 'lens',
    }),
    new Mirrors({
      oX: 0.406,
      oY: 0.588,
    }),
    new Oilify({
      maskRadius: 8,
    }),
    new Bloom({
      radius: 2.08,
      softness: 87.94,
      strength: 165.29,
    }),
  ],
  resetAlpha: false,
  resolution: {
    width,
    height,
  },
};

export default project;
