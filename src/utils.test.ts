import { matches } from './utils';

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
