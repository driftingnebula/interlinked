import {BaseOperation} from './base.js';

export interface DiffractionPatternsParameters {
  blueContours: number;
  blueFrequency: number;
  blueSedges: number;
  brightness: number;
  greenContours: number;
  greenFrequency: number;
  greenSedges: number;
  height: number;
  polarization: number;
  redContours: number;
  redFrequency: number;
  redSedges: number;
  scattering: number;
  width: number;
}

export class DiffractionPatterns extends BaseOperation<DiffractionPatternsParameters> {
  public static default: DiffractionPatternsParameters = {
    blueContours: 0.97,
    blueFrequency: 1.12,
    blueSedges: 0.64,
    brightness: 0.07,
    greenContours: 0.82,
    greenFrequency: 1.22,
    greenSedges: 0.68,
    height: 200,
    polarization: -0.47,
    redContours: 0.82,
    redFrequency: 0.81,
    redSedges: 0.61,
    scattering: 37.13,
    width: 200,
  };

  public get default() {
    return DiffractionPatterns.default;
  }

  public appendCrop = true;
  public name = 'gegl:diffraction-patterns';

  constructor(parameters?: Partial<DiffractionPatternsParameters>) {
    super({...DiffractionPatterns.default, ...parameters});
  }
}
