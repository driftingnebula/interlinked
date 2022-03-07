import {BaseOperation} from './base.js';

export interface MirrorsParameters {
  clip: boolean;
  cX: number;
  cY: number;
  inputScale: number;
  mAngle: number;
  nSegs: number;
  outputScale: number;
  oX: number;
  oY: number;
  rAngle: number;
  trimX: number;
  trimY: number;
  warp: boolean;
}

export class Mirrors extends BaseOperation<MirrorsParameters> {
  public static default: MirrorsParameters = {
    clip: true,
    cX: 0.5,
    cY: 0.5,
    inputScale: 100,
    mAngle: 0,
    nSegs: 6,
    outputScale: 1,
    oX: 0,
    oY: 0,
    rAngle: 0,
    trimX: 0,
    trimY: 0,
    warp: true,
  };

  public appendCrop = false;
  public name = 'gegl:mirrors';

  constructor(parameters?: Partial<MirrorsParameters>) {
    super({...Mirrors.default, ...parameters});
  }
}
