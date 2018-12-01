import { matches } from '../utils';

function* numbers(input: string) {
  for (const [result] of matches(/(-|\+)?\d+/g, input)) {
    try {
      yield BigInt(result);
    } catch {}
  }
}

function* cycle<T>(createIterable: () => Iterable<T>) {
  for (;;) {
    yield* createIterable();
  }
}

export function part1(input: string): string {
  let sum = 0n;

  for (const number of numbers(input)) {
    sum += number;
  }
  
  return sum.toString();
}

export function part2(input: string): string {
  let sum = 0n;
  const seen = new Set<bigint>([sum]);
  
  for (const number of cycle(() => numbers(input))) {
    sum += number;

    if (seen.has(sum)) {
      return sum.toString();
    }

    seen.add(sum);
  }

  throw new Error('No solution found!');
}