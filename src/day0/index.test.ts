import { task1, task2 } from '.';

test('task1', async () => {
  const result = await task1();
  expect(result).toBe(512);
});

test('task2', async () => {
  const result = await task2();
  expect(result).toBe(23285);
});
