import {
  Bloom,
  Cartoon,
  Crop,
  FocusBlur,
  Generic,
  Mirrors,
  Newsprint,
  Waves,
} from '../../gegl/exports.js';
import Project from '../../project.js';

const [width, height] = [3840, 2160];

const project: Project = {
  createInputImage: false,
  name: '2022-03-15',
  operations: [
    new Generic('gegl:spiral', {
      balance: 0.424,
      color1: '#3078d2',
      color2: '#ff00dc',
      direction: 'cw',
      height,
      radius: 918.7,
      type: 'linear',
      width,
      x: 0.307,
      y: 0.542,
    }),
    new Crop({
      height,
      width,
    }),
    new Cartoon({
      maskRadius: 50,
      pctBlack: 1,
    }),
    new Newsprint({
      colorModel: 'rgb',
      pattern2: 'line',
      pattern3: 'diamond',
      pattern4: 'pssquare',
      period2: 105.96,
      period3: 30.46,
      period4: 125.83,
      turbulence: 0.182,
    }),
    new Bloom({
      radius: 2.08,
      softness: 87.94,
      strength: 165.29,
    }),
    new Cartoon({
      maskRadius: 50,
      pctBlack: 1,
    }),
    new Waves({
      amplitude: 65,
      clamp: true,
      period: 500,
      phi: 0.5,
      x: 0,
      y: 1,
    }),
    new Mirrors({
      rAngle: 30,
      nSegs: 3,
      oX: 0.312,
      oY: 1,
      trimX: 0.162,
      trimY: 0.031,
    }),
    new FocusBlur({
      blurRadius: 5.4,
      blurType: 'lens',
      focus: 0.154,
      highlightFactor: 0.75,
      midpoint: 0.38,
      radius: 0.802,
    }),
  ],
  resetAlpha: false,
  resolution: {
    width,
    height,
  },
};

export default project;
