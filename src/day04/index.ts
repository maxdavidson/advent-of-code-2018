import { lines, maxBy, sum, createComparator } from '../utils';

const enum ActionType {
  BeginShift,
  FallAsleep,
  WakeUp,
}

type Action =
  | { type: ActionType.BeginShift; date: Date; id: number }
  | { type: ActionType.FallAsleep; date: Date }
  | { type: ActionType.WakeUp; date: Date };

function* entries(input: string): IterableIterator<Action> {
  for (const line of lines(input)) {
    const date = new Date(line.slice(1, 17));
    const type = line.slice(19, 24);

    switch (type) {
      case 'Guard':
        const id = parseInt(line.slice(26, line.indexOf(' ', 26)), 10);
        yield { type: ActionType.BeginShift, date, id };
        break;
      case 'falls':
        yield { type: ActionType.FallAsleep, date };
        break;
      case 'wakes':
        yield { type: ActionType.WakeUp, date };
        break;
    }
  }
}

const dateComparator = createComparator<Action>(action => action.date.getTime());

function computeSleepStats(input: string) {
  const actions = Array.from(entries(input)).sort(dateComparator);
  const sleepStats = new Map<number, Uint32Array>();

  let currentGuardId: number | undefined = undefined;
  let fellAsleepDate: Date | undefined = undefined;

  for (const action of actions) {
    switch (action.type) {
      case ActionType.BeginShift: {
        fellAsleepDate = undefined;
        currentGuardId = action.id;
        break;
      }

      case ActionType.FallAsleep: {
        fellAsleepDate = action.date;
        break;
      }

      case ActionType.WakeUp: {
        if (currentGuardId === undefined || fellAsleepDate === undefined) {
          continue;
        }

        if (!sleepStats.has(currentGuardId)) {
          sleepStats.set(currentGuardId, new Uint32Array(60));
        }

        const guardStats = sleepStats.get(currentGuardId!)!;
        const wokeUpDate = action.date;

        for (
          let date = fellAsleepDate;
          date.getTime() < wokeUpDate.getTime();
          date = new Date(date.getTime() + 60_000)
        ) {
          guardStats[date.getMinutes()] += 1;
        }

        fellAsleepDate = undefined;
        break;
      }
    }
  }

  return sleepStats;
}

export function part1(input: string) {
  const sleepStats = computeSleepStats(input);

  const result = maxBy(sleepStats, ([_, guardStats]) => sum(guardStats));

  if (result === undefined) {
    throw new Error('No solution found!');
  }

  const [bestGuardId, bestGuardStats] = result;

  const bestMinute = maxBy(bestGuardStats.keys(), i => bestGuardStats[i]);

  if (bestMinute === undefined) {
    throw new Error('No solution found!');
  }

  return bestGuardId * bestMinute;
}

export function part2(input: string) {
  const sleepStats = computeSleepStats(input);

  let bestGuard: number | undefined = undefined;
  let bestMinute: number | undefined = undefined;
  let mostSleep = -Infinity;

  for (let i = 0; i < 60; ++i) {
    const [guardId, guardStats] = maxBy(sleepStats, ([_, guardStats]) => guardStats[i])!;
    if (mostSleep < guardStats[i]) {
      mostSleep = guardStats[i];
      bestMinute = i;
      bestGuard = guardId;
    }
  }

  if (bestGuard === undefined || bestMinute === undefined) {
    throw new Error('No solution found!');
  }

  return bestGuard * bestMinute;
}
