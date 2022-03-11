import {BaseOperation} from './base.js';

export interface NoisePickParameters {
  pctRandom: number;
  repeat: number;
  seed: number;
}

export class NoisePick extends BaseOperation<NoisePickParameters> {
  public static default: NoisePickParameters = {
    pctRandom: 50,
    repeat: 1,
    seed: 0,
  };

  public get default() {
    return NoisePick.default;
  }

  public appendCrop = true;
  public name = 'gegl:noise-pick';

  constructor(parameters?: Partial<NoisePickParameters>) {
    super({...NoisePick.default, ...parameters});
  }
}
