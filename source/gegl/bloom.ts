import {BaseOperation} from './base.js';

export interface BloomParameters {
  limitExposure: boolean;
  radius: number;
  softness: number;
  strength: number;
  threshold: number;
}

export class Bloom extends BaseOperation<BloomParameters> {
  public static default: BloomParameters = {
    limitExposure: false,
    radius: 10,
    softness: 25,
    strength: 50,
    threshold: 50,
  };

  public get default() {
    return Bloom.default;
  }

  public appendCrop = false;
  public name = 'gegl:bloom';

  constructor(parameters?: Partial<BloomParameters>) {
    super({...Bloom.default, ...parameters});
  }
}
