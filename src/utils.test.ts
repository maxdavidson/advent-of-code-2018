import { matches, combinations, split, unique, uniqueBy, zip, chars } from './utils';

test('matches', () => {
  const regex = /\d/g;
  const testStr = '123abc';

  expect(regex.lastIndex).toBe(0);
  expect(Array.from(matches(regex, testStr), arr => arr[0])).toEqual(['1', '2', '3']);
  expect(regex.lastIndex).toBe(0);

  const firstIt = matches(regex, testStr);
  const secondIt = matches(regex, testStr);

  expect(firstIt.next().value[0]).toBe('1');
  expect(firstIt.next().value[0]).toBe('2');
  expect(regex.lastIndex).toBe(0);

  expect(secondIt.next().value[0]).toBe('1');
  expect(regex.lastIndex).toBe(0);

  expect(secondIt.return!().done).toBe(true);
  expect(regex.lastIndex).toBe(0);

  expect(() => {
    secondIt.throw!(new RangeError());
  }).toThrow(RangeError);
  expect(regex.lastIndex).toBe(0);
});

test('combinations', () => {
  expect(new Set(combinations(chars('ABC'), 2))).toEqual(new Set([['A', 'B'], ['A', 'C'], ['B', 'C']]));
  expect(new Set(combinations(chars('ABC'), 3))).toEqual(new Set([['A', 'B', 'C']]));

  expect(new Set(combinations(chars('ABCDE'), 3))).toEqual(
    new Set([
      ['A', 'B', 'C'],
      ['A', 'B', 'D'],
      ['A', 'B', 'E'],
      ['A', 'C', 'D'],
      ['A', 'C', 'E'],
      ['A', 'D', 'E'],
      ['B', 'C', 'D'],
      ['B', 'C', 'E'],
      ['B', 'D', 'E'],
      ['C', 'D', 'E'],
    ]),
  );
});

test('unique', () => {
  expect(Array.from(unique(['a', 'a', 'b']))).toEqual(['a', 'b']);
});

test('uniqueBy', () => {
  expect(Array.from(uniqueBy(['aa', 'ab', 'b'], value => value.length))).toEqual(['aa', 'b']);
});

test('zip', () => {
  expect(Array.from(zip('AB', 'CD'))).toEqual([['A', 'C'], ['B', 'D']]);
});

test('split', () => {
  const testStr = 'hello world';

  expect(Array.from(split(testStr, ''))).toEqual(testStr.split(''));
  expect(Array.from(split(testStr, 'l'))).toEqual(testStr.split('l'));
});
