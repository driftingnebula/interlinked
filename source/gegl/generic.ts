import {BaseOperation} from './base.js';

export type GenericParameters = Record<string, number | string>;

export class Generic extends BaseOperation<GenericParameters> {
  public appendCrop: boolean;
  public name: string;

  constructor(
    name: string,
    parameters?: GenericParameters,
    appendCrop?: boolean,
  ) {
    super(parameters ?? {});
    this.name = name;
    this.appendCrop = appendCrop ?? false;
  }
}
