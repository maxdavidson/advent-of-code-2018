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

function overlaps(claimA: Claim, claimB: Claim) {
  return (
    claimA.left < claimB.left + claimB.width &&
    claimA.left + claimA.width > claimB.left &&
    claimA.top < claimB.top + claimB.height &&
    claimA.top + claimA.height > claimB.top
  );
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

  for (const overlapCount of map.values()) {
    if (overlapCount >= 2) {
      sum += 1;
    }
  }

  return sum;
}

export function part2(input: string) {
  // Pre-collect the claims, since we need to access them very frequently
  const claimsArray = Array.from(claims(input));

  claimsLoop: for (let i = 0; i < claimsArray.length; ++i) {
    for (let j = 0; j < claimsArray.length; ++j) {
      if (i != j && overlaps(claimsArray[i], claimsArray[j])) {
        continue claimsLoop;
      }
    }

    return claimsArray[i].id;
  }

  throw new Error('No solution found!');
}
