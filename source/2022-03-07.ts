import {
  Cartoon,
  Crop,
  MedianBlur,
  Mirrors,
  Mosaic,
  Plasma,
  Waterpixels,
  Waves,
} from './gegl/exports.js';
import Project from './project.js';

const [width, height] = [1920, 1080];

const project: Project = {
  createInputImage: false,
  name: '2022-03-07',
  operations: [
    new Plasma({
      height,
      seed: 2_000_111_903,
      turbulence: 1,
      width,
    }),
    new Mosaic({
      colorVariation: 1,
      tileHeight: 5,
      tileNeatness: 1,
      tileSize: 116.53,
      tileSurface: true,
      tileType: 'triangles',
    }),
    new Waves({
      amplitude: 2.9,
      clamp: true,
      samplerType: 'cubic',
    }),
    new Waves({
      amplitude: 17.3,
      clamp: true,
      samplerType: 'cubic',
    }),
    new Mirrors({
      oX: 1,
      oY: 0.353,
      nSegs: 2,
    }),
    new Cartoon(),
    new Crop({height, width}),
    new Waterpixels({
      fill: 'average',
      size: 32,
      smoothness: 1,
    }),
    new Mirrors({
      oX: 0.01,
      oY: 0.01,
      nSegs: 5,
      rAngle: 342,
    }),
    new MedianBlur(),
  ],
  resetAlpha: false,
  resolution: {
    width,
    height,
  },
};

export default project;
