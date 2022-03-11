import {BaseOperation} from './base.js';

export interface MazeParameters {
  algorithmType: 'depth-first' | 'prim';
  bgColor: string;
  fgColor: string;
  seed: number;
  tileable: boolean;
  x: number;
  y: number;
}

export class Maze extends BaseOperation<MazeParameters> {
  public static default: MazeParameters = {
    algorithmType: 'depth-first',
    bgColor: '#fff',
    fgColor: '#000',
    seed: 0,
    tileable: false,
    x: 16,
    y: 16,
  };

  public get default() {
    return Maze.default;
  }

  public appendCrop = false;
  public name = 'gegl:maze';

  constructor(parameters?: Partial<MazeParameters>) {
    super({...Maze.default, ...parameters});
  }
}
