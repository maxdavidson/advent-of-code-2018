import path from 'path';
import { part1, part2 } from '.';
import { readFile } from '../utils';

let input: string;

beforeAll(async () => {
  input = await readFile(path.resolve(__dirname, 'input'));
});

const testInput = `
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up`;

test('part1', () => {
  expect(part1(testInput)).toBe(240);
  expect(part1(input)).toBe(21956);
});

test('part2', () => {
  expect(part2(testInput)).toBe(4455);
  expect(part2(input)).toBe(134511);
});
