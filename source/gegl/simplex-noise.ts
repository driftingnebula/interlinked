import {BaseOperation} from './base.js';

export interface SimplexNoiseParameters {
  iterations: number;
  scale: number;
  seed: number;
}

export class SimplexNoise extends BaseOperation<SimplexNoiseParameters> {
  public static default: SimplexNoiseParameters = {
    iterations: 1,
    scale: 1,
    seed: 1,
  };

  public appendCrop = true;
  public name = 'gegl:simplex-noise';

  constructor(parameters?: Partial<SimplexNoiseParameters>) {
    super({...SimplexNoise.default, ...parameters});
  }
}
