use crate::AddNumber;

#[derive(Debug, Default)]
pub struct State {
    numbers: Vec<u128>,
}

impl AddNumber for State {
    fn add_number(&mut self, number: u128) {
        if self.does_crumble(number) {
            println!("{} will crumble", number);
        }

        self.numbers.push(number);
        if self.numbers.len() > 100 {
            self.numbers.remove(0);
        };
    }
}

impl State {
    const MINE_LENGTH: usize = 100;

    fn does_crumble(&self, candidate: u128) -> bool {
        if self.numbers.len() < Self::MINE_LENGTH {
            return false;
        }

        for (i, outer) in self.numbers.iter().enumerate() {
            for (j, inner) in self.numbers.iter().enumerate() {
                if outer + inner == candidate && i != j {
                    return false;
                }
            }
        }
        return true;
    }
}
