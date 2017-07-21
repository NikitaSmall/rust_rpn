use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Stackable {
    Operation(char),
    Value(i32),
}

pub struct Converter {
    raw: String,
    pub result: String,
    prededence: HashMap<Stackable, i8>,
}

impl Converter {
    pub fn convert(raw: String) -> Converter {
        let mut prededence = HashMap::new();
        prededence.insert(Stackable::Operation('/'), 5);
        prededence.insert(Stackable::Operation('*'), 5);
        prededence.insert(Stackable::Operation('+'), 4);
        prededence.insert(Stackable::Operation('-'), 4);

        let mut converter = Converter {
            raw: raw,
            result: String::from(""),
            prededence: prededence,
        };
        converter.process();

        converter
    }

    fn process(&mut self) {
        let elements = Converter::parse(&self.raw);
        let mut q: Vec<Stackable> = Vec::new();
        let mut s: Vec<Stackable> = Vec::new();

        for value in elements {
            match value {
                Stackable::Value(_) => q.push(value),
                Stackable::Operation(_) => {
                    while (s.len() > 0) &&
                        self.prededence.get(&value) <= self.prededence.get(&s[s.len() - 1])
                    {
                        q.push(s.pop().unwrap());
                    }
                    s.push(value);
                    continue;
                }
            }
        }

        while s.len() > 0 {
            q.push(s.pop().unwrap());
        }

        let res: Vec<String> = q.iter()
            .map(|el| match el {
                &Stackable::Value(num) => num.to_string(),
                &Stackable::Operation(op) => op.to_string(),
            })
            .collect();

        self.result = res.join(" ");
    }

    pub fn parse(raw_value: &str) -> Vec<Stackable> {
        let values: Vec<&str> = raw_value.split_whitespace().collect();

        values
            .into_iter()
            .map(|value| match value {
                "+" => Stackable::Operation('+'),
                "-" => Stackable::Operation('-'),
                "/" => Stackable::Operation('/'),
                "*" => Stackable::Operation('*'),
                _ => Stackable::Value(Converter::parse_value(value)),
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
    fn parse_simple() {
        let example_string = String::from("33 + 23");

        let mut converter = Converter::convert(example_string);
        assert_eq!(converter.result, String::from("33 23 +"))
    }

    #[test]
    fn parse_complex() {
        let example_string = String::from("4 / 2 + 3 * 2");

        let mut converter = Converter::convert(example_string);
        assert_eq!(converter.result, String::from("2 3 4 2 / + *"))
    }
}
