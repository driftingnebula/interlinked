import {BaseOperation} from './base.js';

export interface CartoonParameters {
  maskRadius: number;
  pctBlack: number;
}

export class Cartoon extends BaseOperation<CartoonParameters> {
  public static default: CartoonParameters = {
    maskRadius: 7,
    pctBlack: 0.2,
  };

  public get default() {
    return Cartoon.default;
  }

  public appendCrop = true;
  public name = 'gegl:cartoon';

  constructor(parameters?: Partial<CartoonParameters>) {
    super({...Cartoon.default, ...parameters});
  }
}
