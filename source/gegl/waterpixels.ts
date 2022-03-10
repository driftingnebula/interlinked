import {BaseOperation} from './base.js';

export interface WaterpixelsParameters {
  fill: 'average' | 'random';
  regularization: number;
  size: number;
  smoothness: number;
}

export class Waterpixels extends BaseOperation<WaterpixelsParameters> {
  public static default: WaterpixelsParameters = {
    fill: 'average',
    regularization: 0,
    size: 32,
    smoothness: 1,
  };

  public get default() {
    return Waterpixels.default;
  }

  public appendCrop = false;
  public name = 'gegl:waterpixels';

  constructor(parameters?: Partial<WaterpixelsParameters>) {
    super({...Waterpixels.default, ...parameters});
  }
}
