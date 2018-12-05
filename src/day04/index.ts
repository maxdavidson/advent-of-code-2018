import { lines, maxBy, sum, createComparator, cartesianProduct, range } from '../utils';

const enum ActionType {
  BeginShift,
  FallAsleep,
  WakeUp,
}

type Action =
  | { type: ActionType.BeginShift; date: Date; id: number }
  | { type: ActionType.FallAsleep; date: Date }
  | { type: ActionType.WakeUp; date: Date };

const enum StateType {
  Vacant,
  Working,
  Sleeping,
}

type State =
  | { type: StateType.Vacant }
  | { type: StateType.Working; date: Date; id: number }
  | { type: StateType.Sleeping; date: Date; id: number };

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
  const stats = new Map<number, Uint32Array>();

  let state: State = { type: StateType.Vacant };

  for (const action of actions) {
    if (action.type === ActionType.BeginShift) {
      const { id, date } = action;
      state = { type: StateType.Working, date, id };
    } else if (action.type === ActionType.FallAsleep && state.type === StateType.Working) {
      // @ts-ignore
      const { id } = state;
      const { date } = action;
      state = { type: StateType.Sleeping, date, id };
    } else if (action.type === ActionType.WakeUp && state.type === StateType.Sleeping) {
      // @ts-ignore
      const { id, date: sleepDate } = state;
      const { date: wakeDate } = action;

      if (!stats.has(id)) {
        stats.set(id, new Uint32Array(60));
      }

      const guardStats = stats.get(id)!;

      for (
        let currentDate = sleepDate;
        currentDate.getTime() < wakeDate.getTime();
        currentDate = new Date(currentDate.getTime() + 60_000)
      ) {
        guardStats[currentDate.getMinutes()] += 1;
      }

      state = { type: StateType.Working, date: wakeDate, id };
    }
  }

  return stats;
}

export function part1(input: string) {
  const sleepStats = computeSleepStats(input);

  const [bestGuardId, bestGuardStats] = maxBy(sleepStats, ([_, guardStats]) => sum(guardStats))!;

  const [bestMinute, _] = maxBy(bestGuardStats.entries(), ([_, value]) => value)!;

  return bestGuardId * bestMinute;
}

export function part2(input: string) {
  const sleepStats = computeSleepStats(input);

  const [bestMinute, [bestGuardId, _]] = maxBy(
    cartesianProduct(range(0, 60), sleepStats),
    ([i, [_, guardStats]]) => guardStats[i],
  )!;

  return bestGuardId * bestMinute;
}
