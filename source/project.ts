import {BaseOperation} from './gegl/exports.js';

export default interface Project {
  name: string;
  operations: Array<InstanceType<typeof BaseOperation>>;
  resolution: {
    height: number;
    width: number;
  };
}
