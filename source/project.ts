import {BaseOperation} from './gegl/exports.js';

export default interface Project {
  createInputImage: boolean;
  name: string;
  operations: Array<InstanceType<typeof BaseOperation>>;
  resetAlpha: boolean;
  resolution: {
    height: number;
    width: number;
  };
}
