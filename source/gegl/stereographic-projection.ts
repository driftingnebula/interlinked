import {BaseOperation} from './base.js';

export interface StereographicProjectionParameters {
  height: number;
  inverse: boolean;
  pan: number;
  samplerType: 'nearest' | 'linear' | 'cubic' | 'nohalo' | 'lohalo';
  spin: number;
  tilt: number;
  width: number;
  zoom: number;
}

export class StereographicProjection extends BaseOperation<StereographicProjectionParameters> {
  public static default: StereographicProjectionParameters = {
    height: -1,
    inverse: false,
    pan: 0,
    samplerType: 'nearest',
    spin: 0,
    tilt: 90,
    width: -1,
    zoom: 100,
  };

  public get default() {
    return StereographicProjection.default;
  }

  public appendCrop = false;
  public name = 'gegl:stereographic-projection';

  constructor(parameters?: Partial<StereographicProjectionParameters>) {
    super({...StereographicProjection.default, ...parameters});
  }
}
