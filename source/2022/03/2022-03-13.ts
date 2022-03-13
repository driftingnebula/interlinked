import {
  Cartoon,
  Mirrors,
  Oilify,
  Plasma,
  StereographicProjection,
  Waterpixels,
} from '../../gegl/exports.js';
import Project from '../../project.js';

const [width, height] = [3840, 2160];

const project: Project = {
  createInputImage: false,
  name: '2022-03-13',
  operations: [
    new Plasma({
      seed: 65_198_886,
      height,
      width,
    }),
    new Cartoon({
      maskRadius: 50,
      pctBlack: 1,
    }),
    new Waterpixels({
      size: 64,
    }),
    new Oilify({
      maskRadius: 8,
    }),
    new Mirrors({
      nSegs: 8,
      oX: 0.829,
      oY: 0.812,
      trimX: 0.325,
      trimY: 0.09,
    }),
    new StereographicProjection({
      tilt: -73.42,
    }),
    new Oilify({
      maskRadius: 8,
    }),
  ],
  resetAlpha: false,
  resolution: {
    width,
    height,
  },
};

export default project;
