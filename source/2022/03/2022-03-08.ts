import {
  Crop,
  DiffractionPatterns,
  FocusBlur,
  Newsprint,
  StereographicProjection,
  TileSeamless,
} from '../../gegl/exports.js';
import Project from '../../project.js';

const [width, height] = [1920, 1080];

const project: Project = {
  createInputImage: false,
  name: '2022-03-08',
  operations: [
    new DiffractionPatterns({
      height,
      width,
    }),
    new Crop({height, width}),
    new TileSeamless(),
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
  resetAlpha: false,
  resolution: {
    width,
    height,
  },
};

export default project;
