use std::env::{args, Args};

#[derive(Debug)]
struct Calculator {
    first_number: f32,
    operator: String,
    second_number: f32,
}

impl Calculator {
    fn run(
        first_number: Option<String>,
        operator: Option<String>,
        second_number: Option<String>,
    ) -> (Calculator, f32) {
        let calc: Calculator = Calculator {
            first_number: first_number.unwrap().parse::<f32>().unwrap(),
            operator: operator.unwrap(),
            second_number: second_number.unwrap().parse::<f32>().unwrap(),
        };

        let result = match calc.operator.as_str() {
            "+" => calc.first_number + calc.second_number,
            "-" => calc.first_number - calc.second_number,
            _ => panic!("Operator not implemented"),
        };
        return (calc, result);
    }
}

fn main() {
    let mut args: Args = args();

    let first = args.nth(1);
    let operator = args.nth(0);
    let second = args.nth(0);

    let (calc, result) = Calculator::run(first, operator, second);
    println!(
        "{} {} {} = {result}",
        calc.first_number, calc.operator, calc.second_number
    );
}
