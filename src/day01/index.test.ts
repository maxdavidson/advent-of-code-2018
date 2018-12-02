import path from 'path';
import { part1, part2 } from '.';
import { readFile } from '../utils';

let input: string;

beforeAll(async () => {
  input = await readFile(path.resolve(__dirname, 'input'));
});

test('part1', () => {
  expect(part1('+1, +1, +1')).toBe(3n);
  expect(part1('+1, +1, -2')).toBe(0n);
  expect(part1('-1, -2, -3')).toBe(-6n);
  expect(part1(input)).toBe(439n);
});

test('part2', () => {
  expect(part2('+1, -1')).toBe(0n);
  expect(part2('+3, +3, +4, -2, -4')).toBe(10n);
  expect(part2('-6, +3, +8, +5, -6')).toBe(5n);
  expect(part2('+7, +7, -2, -7, -4')).toBe(14n);
  expect(part2(input)).toBe(124_645n);
});
