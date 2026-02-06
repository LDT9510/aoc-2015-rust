use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(7);

type Signal = u16;

enum Source<'a> {
    Solved(Signal),
    Unsolved(&'a str),
}

impl<'a> Source<'a> {
    fn parse(source_text: &'a str) -> Self {
        if let Ok(signal) = source_text.parse() {
            Self::Solved(signal)
        } else {
            Self::Unsolved(source_text)
        }
    }
}

type Circuit<'a> = HashMap<&'a str, Source<'a>>;

fn parse_wire_or_signal(wire_or_signal: &str, circuit: &mut Circuit) -> Signal {
    if let Ok(signal) = wire_or_signal.parse::<Signal>() {
        signal
    } else {
        evaluate_wire(wire_or_signal, circuit)
    }
}

fn parse_circuit(input: &str) -> Circuit<'_> {
    let mut circuit = Circuit::new();

    for connection in input.lines() {
        let (source_text, output_wire) = connection
            .split(" -> ")
            .next_tuple()
            .expect("Bad connection format.");

        circuit.insert(output_wire, Source::parse(source_text));
    }

    circuit
}

fn parse_source(source_text: &str, circuit: &mut Circuit) -> Signal {
    let mut source_it = source_text.split(' ');
    let s1 = source_it.next().expect("Bad source format");
    let s2 = source_it.next();
    let s3 = source_it.next();

    match (s1, s2, s3) {
        // handle 'wire' case, since 'signal' case (numeric value) is the Solved case
        (wire, None, None) => evaluate_wire(wire, circuit),
        // handle 'NOT wire/signal' case
        ("NOT", Some(wire_or_signal), None) => !parse_wire_or_signal(wire_or_signal, circuit),
        //handle 'wire/signal OPERATION wire/signal' case
        (op1, Some(operation), Some(op2)) => {
            let a = parse_wire_or_signal(op1, circuit);
            let b = parse_wire_or_signal(op2, circuit);
            match operation {
                "AND" => a & b,
                "OR" => a | b,
                "LSHIFT" => a << b,
                "RSHIFT" => a >> b,
                _ => unreachable!("Unknown bitwise operation"),
            }
        }
        _ => unreachable!("Unknown source format"),
    }
}

fn evaluate_wire(wire: &str, circuit: &mut Circuit) -> Signal {
    match circuit.get(wire).expect("Unknow wire") {
        Source::Solved(signal) => *signal,
        Source::Unsolved(source_text) => {
            let signal = parse_source(source_text, circuit);
            if let Some(source) = circuit.get_mut(wire) {
                *source = Source::Solved(signal);
            }
            signal
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut circuit = parse_circuit(input);
    Some(evaluate_wire("a", &mut circuit) as usize)
}
pub fn part_two(input: &str) -> Option<usize> {
    let mut circuit = parse_circuit(input);
    let a_old = evaluate_wire("a", &mut circuit);

    let mut re_parsed_circuit = parse_circuit(input);

    if let Some(source) = re_parsed_circuit.get_mut("b") {
        *source = Source::Solved(a_old);
    }

    Some(evaluate_wire("a", &mut re_parsed_circuit) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65079));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65079)); // not actually testing anything
    }
}
