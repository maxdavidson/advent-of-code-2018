import path from 'path';
import { readLines, readFileAsText } from '../utils';

export const inputPath = path.resolve(__dirname, './input.txt');

export async function task1() {
  let count = 0;

  for await (const _ of readLines(inputPath)) {
    count += 1;
  }

  return count;
}

export async function task2() {
  const file = await readFileAsText(inputPath);

  return file.length;
}
