use chrono::{Duration, NaiveDateTime, Timelike};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::iter::Iterator;

lazy_static! {
    static ref ONE_MINUTE: Duration = Duration::minutes(1);
}

type ID = u32;

#[derive(Debug)]
enum Action {
    BeginShift { date: NaiveDateTime, id: ID },
    FallAsleep { date: NaiveDateTime },
    WakeUp { date: NaiveDateTime },
}

#[derive(Debug)]
enum State {
    Vacant,
    Working { date: NaiveDateTime, id: ID },
    Sleeping { date: NaiveDateTime, id: ID },
}

fn actions_iter<'a>(input: &'a str) -> impl Iterator<Item = Action> + 'a {
    input.trim().lines().map(|line| {
        let date = NaiveDateTime::parse_from_str(&line[1..17], "%Y-%m-%d %H:%M").unwrap();

        match &line[19..24] {
            "Guard" => {
                let slice = &line[26..];
                let end_index = slice.find(char::is_whitespace).unwrap();
                let id = slice[..end_index].parse().unwrap();
                Action::BeginShift { date, id }
            }
            "wakes" => Action::WakeUp { date },
            "falls" => Action::FallAsleep { date },
            value => {
                panic!("Unexpected value: {}", value);
            }
        }
    })
}

fn compute_stats(input: &str) -> HashMap<ID, Vec<u32>> {
    let mut state = State::Vacant;
    let mut stats = HashMap::new();

    let actions = actions_iter(input).sorted_by_key(|action| match action {
        Action::BeginShift { date, .. } => date.clone(),
        Action::FallAsleep { date } => date.clone(),
        Action::WakeUp { date } => date.clone(),
    });

    for action in actions {
        state = match (state, action) {
            (_, Action::BeginShift { id, date }) => {
                // Start working
                State::Working { id, date }
            }

            (State::Working { id, .. }, Action::FallAsleep { date }) => {
                // Go to sleep
                State::Sleeping { id, date }
            }

            (State::Sleeping { id, date: sleep_date }, Action::WakeUp { date: wake_date }) => {
                let hourly_stats = stats.entry(id).or_insert_with(|| vec![0; 60]);
                let mut current_date = sleep_date;

                while current_date < wake_date {
                    hourly_stats[current_date.time().minute() as usize] += 1;
                    current_date += *ONE_MINUTE;
                }

                // Go back to work
                State::Working { id, date: wake_date }
            }

            (state, action) => panic!("Illegal state transition: {:?}, {:?}", state, action),
        }
    }

    stats
}

pub fn part1(input: &str) -> u32 {
    let stats = compute_stats(input);

    let (best_guard_id, best_guard_stats) = stats
        .into_iter()
        .max_by_key(|(_, guard_stats)| guard_stats.iter().sum::<u32>())
        .expect("No best guard found!");

    let best_minute = best_guard_stats
        .into_iter()
        .enumerate()
        .max_by_key(|(_, x)| *x)
        .map(|(i, _)| i as u32)
        .expect("No best minute found");

    best_guard_id * best_minute
}

pub fn part2(input: &str) -> u32 {
    let stats = compute_stats(input);

    let (best_minute, best_guard_id) = (0..60)
        .cartesian_product(stats.iter())
        .max_by_key(|(i, (_, guard_stats))| guard_stats[*i])
        .map(|(i, (id, _))| (i as u32, id))
        .expect("No best minute or guard found");

    best_minute * best_guard_id
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = include_str!("test_input");
    const INPUT: &'static str = include_str!("input");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 240);
        assert_eq!(part1(INPUT), 21956);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 4455);
        assert_eq!(part2(INPUT), 134511);
    }
}
