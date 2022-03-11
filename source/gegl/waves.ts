import {BaseOperation} from './base.js';

export interface WavesParameters {
  amplitude: number;
  aspect: number;
  clamp: boolean;
  period: number;
  phi: number;
  samplerType: 'nearest' | 'linear' | 'cubic' | 'nohalo' | 'lohalo';
  x: number;
  y: number;
}

export class Waves extends BaseOperation<WavesParameters> {
  public static default: WavesParameters = {
    amplitude: 25,
    aspect: 1,
    clamp: false,
    period: 100,
    phi: 0,
    samplerType: 'cubic',
    x: 0.5,
    y: 0.5,
  };

  public get default() {
    return Waves.default;
  }

  public appendCrop = true;
  public name = 'gegl:waves';

  constructor(parameters?: Partial<WavesParameters>) {
    super({...Waves.default, ...parameters});
  }
}
