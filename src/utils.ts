import fs from 'fs';
import readline from 'readline';
import stream from 'stream';

export function readFile(path: string) {
  return fs.promises.readFile(path, { encoding: 'utf8' });
}

export function readLines(path: string): AsyncIterable<string> {
  const lineReader = readline.createInterface({
    input: fs.createReadStream(path),
  });

  // Async iteration support was added to readline in
  // https://github.com/nodejs/node/commit/2a7432dadec08bbe7063d84f1aa4a6396807305c
  if (Symbol.asyncIterator in lineReader) {
    // @ts-ignore
    return lineReader;
  }

  const readableStream = new stream.Readable({
    objectMode: true,
    read() {
      lineReader.resume();
    },
    destroy(err, cb) {
      lineReader.removeListener('line', handleLine);
      lineReader.removeListener('close', handleClose);
      lineReader.close();
      cb(err);
    },
  });

  function handleLine(input: string) {
    // Pause the line reader if we can't push any more lines
    if (!readableStream.push(input)) {
      lineReader.pause();
    }
  }

  function handleClose() {
    // Close the readable stream by pushing an EOF-signaling `null`
    readableStream.push(null);
  }

  lineReader.addListener('line', handleLine);
  lineReader.addListener('close', handleClose);

  return readableStream;
}

export async function drain<T>(asyncIterable: AsyncIterable<T>): Promise<Array<T>> {
  const items = [] as T[];

  for await (const item of asyncIterable) {
    items.push(item);
  }

  return items;
}

export function* matches(pattern: RegExp, text: string) {
  const originalLastIndex = pattern.lastIndex;
  let currentLastIndex = originalLastIndex;
  let result: RegExpExecArray | null;

  while ((result = pattern.exec(text)) !== null) {
    // Since global regexes are stateful, we need to keep track
    // of the lastIndex here and restore it between invocations.
    currentLastIndex = pattern.lastIndex;
    pattern.lastIndex = originalLastIndex;

    yield result!;

    // Non-global patterns will always produce the same match, so exit after the first one
    if (!pattern.global) {
      return;
    }

    pattern.lastIndex = currentLastIndex;
  }
}

const intRegex = /-?\d+/g;

export function* bigints(input: string) {
  for (const [result] of matches(intRegex, input)) {
    try {
      yield BigInt(result);
    } catch {}
  }
}

export function* cycle<T>(createIterable: () => Iterable<T>) {
  for (;;) {
    yield* createIterable();
  }
}

export function* split(input: string, separator: string): Iterable<string> {
  // Special case for empty strings, to avoid infinite loop
  if (separator === '') {
    yield* input;
    return;
  }

  let currentIndex = 0;
  let nextIndex: number;

  while ((nextIndex = input.indexOf(separator, currentIndex)) !== -1) {
    yield input.slice(currentIndex, nextIndex);
    currentIndex = nextIndex + separator.length;
  }

  yield input.slice(currentIndex);
}

export function lines(input: string, lineEnding = '\n'): Iterable<string> {
  return split(input, lineEnding);
}

export function chars(input: string): Iterable<string> {
  return input;
}

function identity<T>(value: T) {
  return value;
}

export function unique<T>(iterable: Iterable<T>) {
  return uniqueBy(iterable, identity);
}

export function* uniqueBy<T, U, This = undefined>(
  iterable: Iterable<T>,
  computeKey: (this: This, value: T) => U,
  thisArg?: This,
) {
  const seen = new Set();

  for (const value of iterable) {
    const key = computeKey.call(thisArg!, value);
    if (!seen.has(key)) {
      seen.add(key);
      yield value;
    }
  }
}

export function maxBy<T>(iterable: Iterable<T>, getValue: (value: T) => number) {
  let maxValue = -Infinity;
  let maxItem: T | undefined = undefined;

  for (const item of iterable) {
    const value = getValue(item);
    if (maxValue < value) {
      maxValue = value;
      maxItem = item;
    }
  }

  return maxItem;
}

export function* range(fromInclusive: number, toExclusive: number) {
  for (let i = fromInclusive; i < toExclusive; ++i) {
    yield i;
  }
}

export function sum(array: ArrayLike<number>): number {
  let sum = 0;
  for (let i = 0; i < array.length; ++i) {
    sum += array[i];
  }
  return sum;
}

export function createComparator<T>(extract: (value: T) => number) {
  return (valueA: T, valueB: T) => extract(valueA) - extract(valueB);
}

const createIterator = <T>(obj: Iterable<T>) => obj[Symbol.iterator]();
const getNext = <T>(it: Iterator<T>) => it.next();
const getValue = <T>(result: IteratorResult<T>) => result.value;
const isDone = <T>(result: IteratorResult<T>) => result.done;

class ZipIterator<Ts extends unknown[]> implements Iterator<Ts> {
  constructor(private readonly iterators: { [K in keyof Ts]: Iterator<Ts[K]> }) {}

  next() {
    const results = this.iterators.map(getNext);
    return {
      value: results.map(getValue) as Ts,
      done: results.some(isDone),
    };
  }
}

export function zip<Ts extends unknown[]>(...iterables: { [K in keyof Ts]: Iterable<Ts[K]> }): Iterable<Ts> {
  return {
    [Symbol.iterator]() {
      return new ZipIterator<Ts>(Array.from(iterables, createIterator) as any);
    },
  };
}

export type Tuple<T, N extends number> = {
  0: [];
  1: [T];
  2: [T, T];
  3: [T, T, T];
  4: [T, T, T, T];
  5: [T, T, T, T, T];
  [fallback: number]: T[];
}[N];

function* combinationsHelper<T, N extends number>(
  array: ReadonlyArray<T>,
  length: N,
  startIndex: number,
): IterableIterator<Tuple<T, N>> {
  if (length <= 0) {
    yield [];
  } else {
    for (let fixedIndex = startIndex; fixedIndex < array.length - length + 1; ++fixedIndex) {
      for (const tail of combinationsHelper(array, length - 1, fixedIndex + 1)) {
        yield [array[fixedIndex], ...tail];
      }
    }
  }
}

export function combinations<T, N extends number>(iterable: Iterable<T>, length: N) {
  const array = Array.from(iterable);
  return combinationsHelper(array, length, 0);
}

export function* cartesianProduct<Tuple extends unknown[]>(
  ...iterables: { [N in keyof Tuple]: Iterable<Tuple[N]> }
): IterableIterator<Tuple> {
  if (iterables.length === 0) {
    yield ([] as unknown) as Tuple;
  } else {
    const [headIterable, ...tailIterables] = iterables;
    for (const head of headIterable) {
      for (const tail of cartesianProduct(...tailIterables)) {
        yield ([head, ...tail] as unknown) as Tuple;
      }
    }
  }
}
