import { bigints, cycle } from '../utils';

export function part1(input: string) {
  let sum = 0n;

  for (const num of bigints(input)) {
    sum += num;
  }
  
  return sum;
}

export function part2(input: string) {
  let sum = 0n;
  const seen = new Set([sum]);
  
  for (const num of cycle(() => bigints(input))) {
    sum += num;

    if (seen.has(sum)) {
      return sum;
    }

    seen.add(sum);
  }

  throw new Error('No solution found!');
}