import {BaseOperation} from './base.js';

export interface PlasmaParameters {
  height: number;
  seed: number;
  turbulence: number;
  width: number;
  x: number;
  y: number;
}

export class Plasma extends BaseOperation<PlasmaParameters> {
  public static default: PlasmaParameters = {
    height: 768,
    seed: 0,
    turbulence: 1,
    width: 1024,
    x: 0,
    y: 0,
  };

  public get default() {
    return Plasma.default;
  }

  public appendCrop = false;
  public name = 'gegl:plasma';

  constructor(parameters?: Partial<PlasmaParameters>) {
    super({...Plasma.default, ...parameters});
  }
}
