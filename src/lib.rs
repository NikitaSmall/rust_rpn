mod converter;

use converter::Stackable;
use converter::Converter;

#[derive(Debug)]
pub struct Processor {
    stack: Vec<i32>,
    values: Vec<Stackable>,
}

impl Processor {
    pub fn new(raw_value: String) -> Processor {
        Processor {
            stack: Vec::new(),
            values: Converter::parse(&raw_value),
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
