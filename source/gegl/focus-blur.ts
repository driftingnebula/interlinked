import {BaseOperation} from './base.js';

export interface FocusBlurParameters {
  aspectRatio: number;
  blurRadius: number;
  blurType: 'gaussian' | 'lens';
  focus: number;
  highlightFactor: number;
  highlightThresholdHigh: number;
  highlightThresholdLow: number;
  highQuality: boolean;
  midpoint: number;
  radius: number;
  rotation: number;
  shape: 'circle' | 'square' | 'diamond' | 'horizontal' | 'vertical';
  x: number;
  y: number;
}

export class FocusBlur extends BaseOperation<FocusBlurParameters> {
  public static default: FocusBlurParameters = {
    aspectRatio: 0,
    blurRadius: 25,
    blurType: 'gaussian',
    focus: 0.25,
    highlightFactor: 0,
    highlightThresholdHigh: 1,
    highlightThresholdLow: 0,
    highQuality: false,
    midpoint: 0.5,
    radius: 0.75,
    rotation: 0,
    shape: 'circle',
    x: 0.5,
    y: 0.5,
  };

  public get default() {
    return FocusBlur.default;
  }

  public appendCrop = false;
  public name = 'gegl:focus-blur';

  constructor(parameters?: Partial<FocusBlurParameters>) {
    super({...FocusBlur.default, ...parameters});
  }
}
