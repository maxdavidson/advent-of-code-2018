import { maxScore } from '.';

const PLAYER_COUNT = 491;
const MAX_MARBLE_VALUE = 71058;

test('test cases', () => {
  expect(maxScore(9, 25)).toBe(32);
  expect(maxScore(10, 1618)).toBe(8317);
  expect(maxScore(13, 7999)).toBe(146373);
  expect(maxScore(17, 1104)).toBe(2764);
  expect(maxScore(21, 6111)).toBe(54718);
  expect(maxScore(30, 5807)).toBe(37305);
});

test('part1', () => {
  expect(maxScore(PLAYER_COUNT, MAX_MARBLE_VALUE)).toBe(361_466);
});

test('part2', () => {
  expect(maxScore(PLAYER_COUNT, 100 * MAX_MARBLE_VALUE)).toBe(2_945_918_550);
});
