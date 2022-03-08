import {BaseOperation} from './base.js';

export interface SoftglowParameters {
  brightness: number;
  glowRadius: number;
  sharpness: number;
}

export class Softglow extends BaseOperation<SoftglowParameters> {
  public static default: SoftglowParameters = {
    brightness: 0.3,
    glowRadius: 10,
    sharpness: 0.85,
  };

  public get default() {
    return Softglow.default;
  }

  public appendCrop = false;
  public name = 'gegl:softglow';

  constructor(parameters?: Partial<SoftglowParameters>) {
    super({...Softglow.default, ...parameters});
  }
}
