import path from 'path';
import { part1, part2 } from '.';
import { readFile } from '../utils';

let input: string;

beforeAll(async () => {
  input = await readFile(path.resolve(__dirname, 'input'));
});

test('part1', () => {
  expect(part1('#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2')).toBe(4);
  expect(part1(input)).toBe(121_163);
});

test('part2', () => {
  expect(part2('#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2')).toBe(3);
  expect(part2(input)).toBe(943);
});
