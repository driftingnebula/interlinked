import {BaseOperation} from './base.js';

export interface CellNoiseParameters {
  iterations: number;
  palettize: boolean;
  rank: number;
  scale: number;
  seed: number;
  shape: number;
}

export class CellNoise extends BaseOperation<CellNoiseParameters> {
  public static default: CellNoiseParameters = {
    iterations: 1,
    palettize: false,
    rank: 1,
    scale: 1,
    seed: 0,
    shape: 2,
  };

  public get default() {
    return CellNoise.default;
  }

  public appendCrop = true;
  public name = 'gegl:cell-noise';

  constructor(parameters?: Partial<CellNoiseParameters>) {
    super({...CellNoise.default, ...parameters});
  }
}
