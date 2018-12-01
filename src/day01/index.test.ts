import path from 'path';
import { part1, part2 } from '.';
import { readFileAsText } from '../utils';

let input: string;

beforeAll(async () => {
  input = await readFileAsText(path.resolve(__dirname, 'input'));
});

test('part1', async () => {
  expect(part1('+1, +1, +1')).toBe('3');
  expect(part1('+1, +1, -2')).toBe('0');
  expect(part1('-1, -2, -3')).toBe('-6');
  expect(part1(input)).toBe('439');
});

test('part2', async () => {
  expect(part2('+1, -1')).toBe('0');
  expect(part2('+3, +3, +4, -2, -4')).toBe('10');
  expect(part2('-6, +3, +8, +5, -6')).toBe('5');
  expect(part2('+7, +7, -2, -7, -4')).toBe('14');
  expect(part2(input)).toBe('124645');
});
