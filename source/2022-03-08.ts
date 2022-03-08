import {
  Crop,
  Generic,
  FocusBlur,
  Newsprint,
  StereographicProjection,
} from './gegl/exports.js';
import Project from './project.js';

const [width, height] = [1920, 1080];

const project: Project = {
  createInputImage: false,
  name: '2022-03-08',
  operations: [
    new Generic('gegl:diffraction-patterns', {
      height,
      width,
    }),
    new Crop({height, width}),
    new Generic('gegl:tile-seamless'),
    new StereographicProjection(),
    new Newsprint({
      colorModel: 'cmyk',
      period: 4,
    }),
    new FocusBlur({
      blurRadius: 9.72,
      blurType: 'lens',
      focus: 0,
      highlightFactor: 0.924,
      radius: 1.173,
      shape: 'circle',
    }),
  ],
  resolution: {
    width,
    height,
  },
};

export default project;
