import {BaseOperation} from './base.js';

export type GenericParameters = Record<string, number | string | boolean>;

export class Generic extends BaseOperation<GenericParameters> {
  public get default() {
    return {};
  }

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
