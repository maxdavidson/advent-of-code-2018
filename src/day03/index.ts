import { matches } from '../utils';

const pattern = /^#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)$/gm;

interface Claim {
  id: number;
  left: number;
  top: number;
  width: number;
  height: number;
}

function* claims(input: string): IterableIterator<Claim> {
  for (const [_, id, left, top, width, height] of matches(pattern, input)) {
    yield {
      id: parseInt(id, 10),
      left: parseInt(left, 10),
      top: parseInt(top, 10),
      width: parseInt(width, 10),
      height: parseInt(height, 10),
    };
  }
}

function* claimCoordinates({ left, top, width, height }: Claim) {
  for (let x = left; x < left + width; ++x) {
    for (let y = top; y < top + height; ++y) {
      yield { x, y };
    }
  }
}

export function part1(input: string) {
  const map = new Map<string, number>();

  for (const claim of claims(input)) {
    for (const { x, y } of claimCoordinates(claim)) {
      const key = `${x},${y}`;
      map.set(key, (map.get(key) || 0) + 1);
    }
  }

  let sum = 0;

  for (const overlaps of map.values()) {
    if (overlaps >= 2) {
      sum += 1;
    }
  }

  return sum;
}

export function part2(input: string) {
  const map = new Map<string, Set<number>>();

  for (const claim of claims(input)) {
    for (const { x, y } of claimCoordinates(claim)) {
      const key = `${x},${y}`;
      if (!map.has(key)) {
        map.set(key, new Set());
      }
      map.get(key)!.add(claim.id);
    }
  }

  claim: for (const claim of claims(input)) {
    for (const { x, y } of claimCoordinates(claim)) {
      const key = `${x},${y}`;

      if (map.get(key)!.size > 1) {
        continue claim;
      }
    }

    return claim.id;
  }

  throw new Error('No solution found!');
}
