import {BaseOperation} from './base.js';

export interface OilifyParameters {
  exponent: number;
  intensities: number;
  maskRadius: number;
  useInten: boolean;
}

export class Oilify extends BaseOperation<OilifyParameters> {
  public static default: OilifyParameters = {
    exponent: 8,
    intensities: 128,
    maskRadius: 4,
    useInten: true,
  };

  public get default() {
    return Oilify.default;
  }

  public appendCrop = false;
  public name = 'gegl:oilify';

  constructor(parameters?: Partial<OilifyParameters>) {
    super({...Oilify.default, ...parameters});
  }
}
