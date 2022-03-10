import {BaseOperation} from './base.js';

export type TileSeamlessParameters = Record<string, unknown>;

export class TileSeamless extends BaseOperation<TileSeamlessParameters> {
  public get default() {
    return {};
  }

  public appendCrop = false;
  public name = 'gegl:tile-seamless';

  constructor() {
    super({});
  }
}
