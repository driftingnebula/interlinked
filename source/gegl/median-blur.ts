import {BaseOperation} from './base.js';

export interface MedianBlurParameters {
  abyssPolicy: 'none' | 'clamp';
  alphaPercentile: number;
  highPrecision: boolean;
  neighborhood: 'square' | 'circle' | 'diamond';
  percentile: number;
  radius: number;
}

export class MedianBlur extends BaseOperation<MedianBlurParameters> {
  public static default: MedianBlurParameters = {
    abyssPolicy: 'clamp',
    alphaPercentile: 50,
    highPrecision: false,
    neighborhood: 'circle',
    percentile: 50,
    radius: 3,
  };

  public get default() {
    return MedianBlur.default;
  }

  public appendCrop = false;
  public name = 'gegl:median-blur';

  constructor(parameters?: Partial<MedianBlurParameters>) {
    super({...MedianBlur.default, ...parameters});
  }
}
