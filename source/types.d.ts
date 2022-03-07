import {BaseOperation} from './gegl/exports.js';

declare global {
  interface Project {
    name: string;
    operations: Array<InstanceType<typeof BaseOperation>>;
    resolution: {
      height: number;
      width: number;
    };
  }
}
