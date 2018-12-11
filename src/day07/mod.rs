use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::iter::Iterator;

fn pairs_iter<'a>(input: &'a str) -> impl Iterator<Item = (char, char)> + 'a {
    input.trim().lines().filter_map(|line| {
        Some((
            // parent
            line.chars().nth(5)?,
            // id
            line.chars().nth(36)?,
        ))
    })
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Task(char);

impl Task {
    pub fn required_time(&self) -> u32 {
        let mut value = [0u8; 4];
        self.0.encode_utf8(&mut value);
        u32::from(value[0]) - 0x40
    }

    pub fn value(&self) -> char {
        self.0
    }
}

#[derive(Debug)]
pub struct Tasks {
    parents: HashMap<Task, HashSet<Task>>,
    locked: HashSet<Task>,
}

impl Tasks {
    pub fn from_input(input: &str) -> Tasks {
        let mut parents = HashMap::new();

        for (parent_id, child_id) in pairs_iter(input) {
            let child_task = Task(child_id);
            let parent_task = Task(parent_id);
            parents.entry(child_task).or_insert_with(HashSet::new).insert(parent_task.clone());
            parents.entry(parent_task).or_insert_with(HashSet::new);
        }

        Tasks { parents, locked: HashSet::new() }
    }

    pub fn is_finished(&self) -> bool {
        self.parents.is_empty()
    }

    fn available_nodes(&self) -> impl Iterator<Item = &Task> {
        self.parents
            .iter()
            .filter_map(|(node, parents)| if parents.is_empty() { Some(node) } else { None })
    }

    pub fn get_task(&mut self) -> Option<Task> {
        let node = self.available_nodes().filter(|node| !self.locked.contains(node)).min()?.clone();
        self.locked.insert(node.clone());
        Some(node)
    }

    pub fn complete_task(&mut self, task: &Task) -> Option<Task> {
        if let Some(task) = self.locked.take(task) {
            self.parents.remove(&task);
            for (_, parents) in self.parents.iter_mut() {
                parents.remove(&task);
            }
            Some(task)
        } else {
            None
        }
    }
}

impl Iterator for Tasks {
    type Item = Task;

    fn next(&mut self) -> Option<Task> {
        let task = self.get_task()?;
        self.complete_task(&task);
        Some(task)
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Worker(usize);

impl Worker {
    pub fn generate(count: usize) -> Vec<Worker> {
        (0..count).map(Worker).collect()
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Job {
    pub task: Task,
    pub worker: Worker,
    pub time_finished: u32,
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Job) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Job {
    fn cmp(&self, other: &Job) -> Ordering {
        other.time_finished.cmp(&self.time_finished)
    }
}

pub fn part1(input: &str) -> String {
    let tasks = Tasks::from_input(input);

    tasks.map(|task| task.value()).collect()
}

pub fn part2(input: &str, worker_count: usize, base_task_time: u32) -> u32 {
    let mut current_time = 0;
    let mut tasks = Tasks::from_input(input);
    let mut available_workers = Worker::generate(worker_count);
    let mut running_jobs = BinaryHeap::<Job>::new();

    loop {
        println!("time: {}", current_time);

        while let Some(job) = running_jobs.pop() {
            if job.time_finished == current_time {
                let Job { task, worker, .. } = job;
                println!("- {:?} finished {:?}", &worker, &task);
                tasks.complete_task(&task);
                available_workers.push(worker);
            } else {
                running_jobs.push(job);
                break;
            }
        }

        while let Some(worker) = available_workers.pop() {
            if let Some(task) = tasks.get_task() {
                let task_time = base_task_time + task.required_time();
                let time_finished = current_time + task_time;
                println!("- {:?} scheduled {:?} at {}", &worker, &task, time_finished);
                running_jobs.push(Job { worker, task, time_finished });
            } else {
                available_workers.push(worker);
                break;
            }
        }

        if tasks.is_finished() {
            break;
        }

        current_time += 1;
    }

    current_time
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input");
    const TEST_INPUT: &str = include_str!("test_input");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), "CABDFE");
        assert_eq!(part1(INPUT), "FDSEGJLPKNRYOAMQIUHTCVWZXB");
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT, 2, 0), 15);
        assert_eq!(part2(INPUT, 5, 60), 1000);
    }
}
