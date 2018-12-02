import { chars, combinations, lines, unique, zip } from '../utils';

export function part1(input: string) {
  let twos = 0;
  let threes = 0;

  for (const line of lines(input)) {
    const charFrequencies = new Map<string, number>();

    for (const char of chars(line)) {
      charFrequencies.set(char, (charFrequencies.get(char) || 0) + 1);
    }

    for (const uniqueFrequency of unique(charFrequencies.values())) {
      if (uniqueFrequency === 2) {
        twos++;
      } else if (uniqueFrequency === 3) {
        threes++;
      }
    }
  }

  return twos * threes;
}

export function part2(input: string) {
  for (const [lineA, lineB] of combinations(lines(input), 2)) {
    if (lineA.length !== lineB.length) {
      continue;
    }

    let commonChars = '';

    for (const [charA, charB] of zip(lineA, lineB)) {
      if (charA === charB) {
        commonChars += charA;
      }
    }

    if (commonChars.length === lineA.length - 1) {
      return commonChars;
    }
  }

  throw new Error('No solution found!');
}
