pub struct CoordinatesIterator {
    modulos: Vec<usize>,
    state: Vec<usize>,
    finished: bool,
}

impl CoordinatesIterator {
    pub fn new(modulos: &Vec<usize>) -> Self {
        let mut state = modulos.clone();
        for i in &mut state {
            *i -= 1;
        }
        Self {
            modulos: modulos.clone(),
            state: state,
            finished: false,
        }
    }

    fn tick(&mut self) {
        let mut carry = true;
        for i in (0usize..self.state.len()).rev() {
            if !carry {
                break;
            }
            if self.state[i] == 0 {
                self.state[i] = self.modulos[i] - 1;
                carry = true;
                continue;
            }
            self.state[i] = self.state[i] - 1;
            carry = false;
        }
    }
}

impl std::iter::Iterator for CoordinatesIterator {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let ret = Some(self.state.clone());

        self.finished = self.state == vec![0; self.state.len()];

        self.tick();

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let modulos = vec![5, 4, 3, 2];
        let counter = CoordinatesIterator::new(&modulos);
        for thing in counter {
            println!("{:?}", thing);
        }
    }
}
