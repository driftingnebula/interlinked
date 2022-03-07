export abstract class BaseOperation<P> {
  public abstract get default(): P;

  public parameters: P;

  /**
   * Some GEGL operations will run infinitely unless you limit the buffer in
   * some way, so all operations must indicate whether or not they should be
   * followed by a crop operation.
   */
  public abstract appendCrop: boolean;

  /** The GEGL operation name, starting with `gegl:`. */
  public abstract name: string;

  constructor(parameters: P) {
    this.parameters = parameters;
  }

  public graph(includeDefaults = false): string[] {
    const defaults = this.default;
    const graph: string[] = [this.name];

    for (const [key, value] of Object.entries(this.parameters)) {
      if (
        includeDefaults &&
        key in defaults &&
        (defaults as Record<string, any>)[key] === value
      ) {
        continue;
      }

      const kebabCasedKey = key.replace(/([A-Z])/g, '-$1').toLowerCase();
      graph.push(`${kebabCasedKey}=${value as string}`);
    }

    return graph;
  }
}
