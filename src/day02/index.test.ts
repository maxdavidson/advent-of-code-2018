import path from 'path';
import { part1, part2 } from '.';
import { readFile } from '../utils';

let input: string;

beforeAll(async () => {
  input = await readFile(path.resolve(__dirname, 'input'));
});

test('part1', () => {
  expect(part1('abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab')).toBe(12);
  expect(part1(input)).toBe(6175);
});

test('part2', () => {
  expect(part2('abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz')).toBe('fgij');
  expect(part2(input)).toBe('asgwjcmzredihqoutcylvzinx');
});
