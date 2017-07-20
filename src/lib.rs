#[derive(Debug)]
enum Stackable {
    Operation(char),
    Value(i32),
}

#[derive(Debug)]
pub struct Processor {
    stack: Vec<i32>,
    values: Vec<Stackable>,
}

impl Processor {
    pub fn new(raw_value: String) -> Processor {
        Processor {
            stack: Vec::new(),
            values: Processor::parse(raw_value),
        }
    }

    pub fn calculate(&mut self) -> i32 {
        for value in self.values.iter() {
            match value {
                &Stackable::Value(num) => self.stack.push(num),
                &Stackable::Operation(operation) => {
                    let value_one = self.stack.pop().unwrap();
                    let value_two = self.stack.pop().unwrap();

                    match operation {
                        '+' => self.stack.push(value_two + value_one),
                        '-' => self.stack.push(value_two - value_one),
                        '*' => self.stack.push(value_two * value_one),
                        '/' => self.stack.push(value_two / value_one),
                        _ => panic!("Unexpected operation!"),
                    }
                }
            }

        }
        self.stack.pop().unwrap()
    }

    fn parse(raw_value: String) -> Vec<Stackable> {
        let values: Vec<&str> = raw_value.split_whitespace().collect();

        values
            .into_iter()
            .map(|value| match value {
                "+" => Stackable::Operation('+'),
                "-" => Stackable::Operation('-'),
                "/" => Stackable::Operation('/'),
                "*" => Stackable::Operation('*'),
                _ => Stackable::Value(Processor::parse_value(value)),
            })
            .collect()
    }

    fn parse_value(number: &str) -> i32 {
        number.trim().parse().expect("Please type a number!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_plus() {
        let example_string = String::from("4 2 +");

        let mut processor = Processor::new(example_string);
        assert_eq!(processor.calculate(), 6)
    }

    #[test]
    fn calculate_minus() {
        let example_string = String::from("4 2 -");

        let mut processor = Processor::new(example_string);
        assert_eq!(processor.calculate(), 2)
    }

    #[test]
    fn calculate_multi() {
        let example_string = String::from("4 2 *");

        let mut processor = Processor::new(example_string);
        assert_eq!(processor.calculate(), 8)
    }

    #[test]
    fn calculate_div() {
        let example_string = String::from("8 2 /");

        let mut processor = Processor::new(example_string);
        assert_eq!(processor.calculate(), 4)
    }

    #[test]
    fn calculate_complex() {
        let example_string = String::from("5 1 2 + 4 * + 3 -");

        let mut processor = Processor::new(example_string);
        assert_eq!(processor.calculate(), 14)
    }
}
