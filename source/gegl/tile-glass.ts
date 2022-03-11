import {BaseOperation} from './base.js';

export interface TileGlassParameters {
  tileHeight: number;
  tileWidth: number;
}

export class TileGlass extends BaseOperation<TileGlassParameters> {
  public static default: TileGlassParameters = {
    tileHeight: 25,
    tileWidth: 25,
  };

  public get default() {
    return TileGlass.default;
  }

  public appendCrop = false;
  public name = 'gegl:tile-glass';

  constructor(parameters?: Partial<TileGlassParameters>) {
    super({...TileGlass.default, ...parameters});
  }
}
