import {BaseOperation} from './base.js';

export interface MosaicParameters {
  antialiasing: boolean;
  colorAveraging: boolean;
  colorVariation: number;
  jointsColor: string;
  lightColor: string;
  lightDir: number;
  seed: number;
  tileAllowSplit: boolean;
  tileHeight: number;
  tileNeatness: number;
  tileSize: number;
  tileSpacing: number;
  tileSurface: boolean;
  tileType: 'squares' | 'hexagons' | 'octagons' | 'triangles';
}

export class Mosaic extends BaseOperation<MosaicParameters> {
  public static default: MosaicParameters = {
    antialiasing: true,
    colorAveraging: true,
    colorVariation: 0.2,
    jointsColor: '#000',
    lightColor: '#fff',
    lightDir: 135,
    seed: 0,
    tileAllowSplit: true,
    tileHeight: 4,
    tileNeatness: 0.65,
    tileSize: 15,
    tileSpacing: 1,
    tileSurface: false,
    tileType: 'hexagons',
  };

  public get default() {
    return Mosaic.default;
  }

  public appendCrop = false;
  public name = 'gegl:mosaic';

  constructor(parameters?: Partial<MosaicParameters>) {
    super({...Mosaic.default, ...parameters});
  }
}
