use std::collections::{btree_map::Entry, BTreeMap, VecDeque};

use crate::AddNumber;

#[derive(Debug)]
pub struct State {
    sorted_numbers: BTreeMap<u128, u8>,
    numbers_queue: VecDeque<u128>,
}

impl Default for State {
    fn default() -> Self {
        State {
            sorted_numbers: BTreeMap::new(),
            numbers_queue: VecDeque::with_capacity(State::MINE_LENGTH + 10),
        }
    }
}

impl AddNumber for State {
    fn add_number(&mut self, number: u128) {
        if self.does_crumble(number) {
            println!("{} will crumble", number);
        }

        self.numbers_queue.push_back(number);
        let to_remove = match self.numbers_queue.len() {
            len if len > Self::MINE_LENGTH => self.numbers_queue.pop_front(),
            _ => None,
        };

        self.manage_sorted_number(number, to_remove);
    }
}

impl State {
    const MINE_LENGTH: usize = 100;

    fn manage_sorted_number(&mut self, to_add: u128, to_remove: Option<u128>) {
        self.sorted_numbers
            .entry(to_add)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        if let Some(to_remove) = to_remove {
            let entry = self.sorted_numbers.entry(to_remove);
            if let Entry::Occupied(mut entry) = entry {
                let new_count = entry.get() - 1;
                match new_count {
                    0 => entry.remove(),
                    _ => entry.insert(new_count),
                };
            }
        }
    }

    fn min_sum(&self) -> u128 {
        let mut iter = self.sorted_numbers.iter();
        let (first_number, first_count) = iter.next().expect("Expected at least 1 number");
        match first_count {
            2.. => first_number * 2,
            _ => {
                let (second_number, _) = iter.next().expect("Expected at least 2 numbers");
                first_number + second_number
            }
        }
    }

    fn max_sum(&self) -> u128 {
        let mut iter = self.sorted_numbers.iter();
        let (first_number, first_count) = iter.next_back().expect("Expected at least 1 number");

        match first_count {
            2.. => first_number * 2,
            _ => {
                let (second_number, _) = iter.next_back().expect("Expected at least 2 numbers");
                first_number + second_number
            }
        }
    }

    fn does_crumble(&self, candidate: u128) -> bool {
        match self.numbers_queue.len() {
            len if len < Self::MINE_LENGTH => return false,
            _ if !(self.min_sum()..=self.max_sum()).contains(&candidate) => return true,
            _ => {}
        };

        for (&key, &count) in &self.sorted_numbers {
            let remainder = candidate - key;
            match key {
                key if key > (candidate / 2) => return true,
                key if remainder == key && count > 1 => return false,
                key if remainder != key && self.sorted_numbers.get(&remainder).is_some() => {
                    return false
                }
                _ => {}
            };
        }

        return true;
    }
}
