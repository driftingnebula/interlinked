import {BaseOperation} from './base.js';

export interface CropParameters {
  height: number;
  resetOrigin: boolean;
  width: number;
  x: number;
  y: number;
}

export class Crop extends BaseOperation<CropParameters> {
  public static default: CropParameters = {
    height: 0,
    resetOrigin: false,
    width: 0,
    x: 0,
    y: 0,
  };

  public get default() {
    return Crop.default;
  }

  public appendCrop = false;
  public name = 'gegl:crop';

  constructor(parameters?: Partial<CropParameters>) {
    super({...Crop.default, ...parameters});
  }
}
