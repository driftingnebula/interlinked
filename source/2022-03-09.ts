import {
  Crop,
  FocusBlur,
  Generic,
  MedianBlur,
  Mirrors,
  Newsprint,
  TileSeamless,
  Waterpixels,
  Waves,
} from './gegl/exports.js';
import Project from './project.js';

const [width, height] = [1920, 1080];

const project: Project = {
  createInputImage: true,
  name: '2022-03-09',
  operations: [
    new Generic('gegl:maze'),
    new Generic('gegl:tile-glass'),
    new Waterpixels(),
    new Newsprint({
      angle2: -55.4,
      angle3: 60.77,
      angle4: 103.55,
      colorModel: 'rgb',
    }),
    new Waves({
      amplitude: 5.9,
      clamp: true,
    }),
    new Crop({height, width}),
    new Generic('gegl:oilify'),
    new TileSeamless(),
    new MedianBlur({
      percentile: 2.35,
    }),
    new Mirrors({
      nSegs: 3,
      oX: 0.1,
      oY: 0.2,
      rAngle: 330,
    }),
    new FocusBlur({
      blurRadius: 33.57,
      blurType: 'lens',
      focus: 0.111,
      highlightFactor: 0.529,
      radius: 1.173,
    }),
  ],
  resetAlpha: true,
  resolution: {
    width,
    height,
  },
};

export default project;
