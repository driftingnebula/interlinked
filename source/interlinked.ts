import fsp from 'node:fs/promises';
import path from 'node:path';
import {performance} from 'node:perf_hooks';

import {execa} from 'execa';
import meow from 'meow';

import {Crop} from './gegl/exports.js';
import Project from './project.js';

import p2022 from './2022/projects.js';

async function main(): Promise<void> {
  const cli = meow(
    `
    Options
      --filter <name>    Only render projects starting with <name>.
      --include-defaults Include default GEGL operation parameters.
      --no-render        Don't render any images.
    `,
    {
      flags: {
        filter: {
          default: '',
          type: 'string',
        },
        includeDefaults: {
          default: false,
          type: 'boolean',
        },
        render: {
          default: true,
          type: 'boolean',
        },
      },
      importMeta: import.meta,
    },
  );

  const includeDefaults = cli.flags.includeDefaults;
  const noRender = !cli.flags.render;

  const projects: Project[] = [...p2022].filter((project) =>
    project.name.startsWith(cli.flags.filter),
  );

  for (const {
    createInputImage,
    name,
    operations,
    resolution,
    resetAlpha,
  } of projects) {
    const dataStart = performance.now();
    const {width, height} = resolution;

    const baseDir = path.resolve(`./output/${name}`);
    await fsp.mkdir(baseDir, {recursive: true});

    console.log(`# ${name}`);
    console.log(`* ${width}x${height}`);
    console.log(`* ${operations.length} operations`);

    const graph = operations.flatMap((operation) => {
      const graph = operation.graph(includeDefaults);
      if (operation.appendCrop) {
        graph.push(...new Crop({height, width}).graph(includeDefaults));
      }

      return graph;
    });

    const prettyGraph = graph.map((operation) =>
      operation.startsWith('gegl:') ? `\n${operation}\n` : `  ${operation}\n`,
    );

    const graphFile = `${name}.txt`;
    const outputFile = `${name}.png`;
    const compressedFile = `${outputFile.slice(0, -4)}.jpeg`;

    console.log(`* Writing ${graphFile}`);
    await fsp.writeFile(
      path.join(baseDir, graphFile),
      prettyGraph.join('').trimStart(),
    );

    if (noRender) {
      console.log(`* Skipped ${outputFile}`);
    } else {
      const fullOutputFile = path.join(baseDir, outputFile);
      if (createInputImage) {
        await execa('convert', [
          '-size',
          `${width}x${height}`,
          'xc:white',
          fullOutputFile,
        ]);
      }

      console.log(`* Writing ${outputFile}`);
      await execa('gegl', [
        ...(createInputImage ? ['-i', fullOutputFile] : []),
        '-o',
        fullOutputFile,
        '--',
        ...graph,
      ]);

      if (resetAlpha) {
        await execa('convert', [
          fullOutputFile,
          '-alpha',
          'Off',
          fullOutputFile,
        ]);
      }

      console.log(`* Writing ${compressedFile}`);
      await execa('convert', [
        fullOutputFile,
        '-quality',
        '92',
        path.join(baseDir, compressedFile),
      ]);
    }

    const time = (performance.now() - dataStart).toFixed(2);
    console.log(`* Generated in ${time}ms`);
  }
}

void main();
