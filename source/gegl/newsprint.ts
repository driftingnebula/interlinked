import {BaseOperation} from './base.js';

type NewsprintPattern = 'line' | 'circle' | 'diamond' | 'pssquare' | 'cross';

export interface NewsprintParameters {
  aaSamples: number;
  angle: number;
  angle2: number;
  angle3: number;
  angle4: number;
  angleboost: number;
  blackPullout: number;
  blocksize: number;
  colorModel: 'black-on-white' | 'cmyk' | 'rgb' | 'white-on-black';
  pattern: NewsprintPattern;
  pattern2: NewsprintPattern;
  pattern3: NewsprintPattern;
  pattern4: NewsprintPattern;
  period: number;
  period2: number;
  period3: number;
  period4: number;
  turbulence: number;
}

export class Newsprint extends BaseOperation<NewsprintParameters> {
  public static default: NewsprintParameters = {
    aaSamples: 16,
    angle: 75,
    angle2: 15,
    angle3: 45,
    angle4: 0,
    angleboost: 0,
    blackPullout: 1,
    blocksize: -1,
    colorModel: 'black-on-white',
    pattern: 'line',
    pattern2: 'line',
    pattern3: 'line',
    pattern4: 'line',
    period: 12,
    period2: 12,
    period3: 12,
    period4: 12,
    turbulence: 0,
  };

  public appendCrop = false;
  public name = 'gegl:newsprint';

  constructor(parameters?: Partial<NewsprintParameters>) {
    super({...Newsprint.default, ...parameters});
  }
}
