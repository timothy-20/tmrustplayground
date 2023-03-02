struct Cache<T>
    where T: Fn(u32) -> u32 {
    calculate: T,
    value: Option<u32>,
}

impl<T> Cache<T>
    where T: Fn(u32) -> u32 {
    fn new(calculate: T) -> Self<T> {
        Cache {
            calculate,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(value) => value,
            None => {
                let value = (self.calculate)(arg);
                self.value = Some(value);

                value
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {

    }
}