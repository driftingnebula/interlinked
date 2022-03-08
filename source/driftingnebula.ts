import fsp from 'node:fs/promises';
import path from 'node:path';
import {performance} from 'node:perf_hooks';
import process from 'node:process';

import {execa} from 'execa';

import {Crop} from './gegl/exports.js';
import Project from './project.js';

import d2022_03_06 from './2022-03-06.js';
import d2022_03_07 from './2022-03-07.js';
import d2022_03_08 from './2022-03-08.js';

async function main(): Promise<void> {
  const noRender = process.argv.includes('--no-render');
  const includeDefaults = process.argv.includes('--include-defaults');

  const projects: Project[] = [d2022_03_06, d2022_03_07, d2022_03_08];

  for (const {name, operations, resolution} of projects) {
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

    console.log(`* Writing ${graphFile}`);
    await fsp.writeFile(
      path.join(baseDir, graphFile),
      prettyGraph.join('').trimStart(),
    );

    if (noRender) {
      console.log(`* Skipped ${outputFile}`);
    } else {
      console.log(`* Writing ${outputFile}`);
      await execa('gegl', [
        '-o',
        path.join(baseDir, outputFile),
        '--',
        ...graph,
      ]);
    }

    const time = (performance.now() - dataStart).toFixed(2);
    console.log(`* Generated in ${time}ms`);
  }
}

void main();
