# driftingnebula

> Generative art with GIMP, GEGL and ImageMagick.

## Usage

To run the generations you'll need to have [GEGL 0.4], [git], [ImageMagick 7], [NodeJS] and [pnpm] installed. Then in a terminal, you can do the following things.

[GEGL 0.4]: https://gegl.org
[git]: https://git-scm.com
[ImageMagick 7]: https://imagemagick.org/
[NodeJS]: https://nodejs.org
[pnpm]: https://pnpm.io

```sh
# Download the repository.
git clone https://github.com/driftingnebula/driftingnebula
cd driftingnebula

# Install the NodeJS dependencies.
pnpm install

# Run the generations. See the "output" directory for files.
pnpm start

# Run the generations but don't render anything.
pnpm start -- --no-render

# Run the generations and explicitly include all the default
# parameters for GEGL operations.
pnpm start -- --include-defaults

# Lint the code and run tests.
pnpm test
```
