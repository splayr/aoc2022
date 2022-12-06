use crate::utils::get_newline_style;

const INPUT: &str = include_str!("./input.txt");

pub fn dec5_part_one() {
    let (crates_str, instructions_str) = get_crates_str_and_instructions_str(INPUT);
    let mut crate_stacks = get_crate_stacks(crates_str);

    run_cratemover_9000(crate_stacks.as_mut_slice(), instructions_str);

    println!("message from crates: {}", get_message(crate_stacks.as_slice()))
}

pub fn dec5_part_two() {
    let (crates_str, instructions_str) = get_crates_str_and_instructions_str(INPUT);
    let mut crate_stacks = get_crate_stacks(crates_str);

    run_cratemover_9001(crate_stacks.as_mut_slice(), instructions_str);

    println!("message from crates under cratemover 9001: {}", get_message(crate_stacks.as_slice()))
}



fn get_crates_str_and_instructions_str(input: &'static str) -> (&str, &str) {
    let double_newline = get_newline_style(input).repeat(2);

    input
        .split_once(double_newline.as_str())
        .expect("invalid input : can't separate crates and instructions")
}

fn get_crate_stacks(crates_str: &str) -> Vec<Vec<char>> {
    let crate_line_length = crates_str.lines().next().unwrap().len();
    let mut crate_stacks: Vec<Vec<char>> = vec![vec!(); (crate_line_length + 1) >> 2]; // or crate_line_length / 4, but double right bitshift is faster

    let mut i = 0;
    while i < crate_line_length - 1 {
        crates_str
            .lines()
            .filter(|line| !line.contains('1'))
            .for_each(|crate_value| {
                let maybe_crate_inner = crate_value
                    .get(i..(i + 3))
                    .unwrap_or(" ")
                    .chars()
                    .find(|c| c.is_ascii_uppercase());

                if let Some(crate_inner) = maybe_crate_inner {
                    crate_stacks[i >> 2].insert(0, crate_inner);
                };
            });
        i += 4
    }

    crate_stacks
}

fn get_instructions(instruction_line: &str) -> Vec<usize> {
instruction_line
            .split(' ')
            .filter_map(|part| part.parse().ok())
            .collect()
}

fn run_cratemover_9000(crate_stacks: &mut [Vec<char>], instructions_str: &str) {
    instructions_str.lines().for_each(|instruction_line| {
        let instruction = get_instructions(instruction_line);
        let (take_index, give_index) = (instruction[1] - 1, instruction[2] - 1);

        for _ in 0..instruction[0] {
            let taken_crate = crate_stacks[take_index]
                .pop()
                .expect("instruction invalid : no crate left in stack");
            crate_stacks[give_index].push(taken_crate);
        }
    })
}

fn run_cratemover_9001(crate_stacks: &mut [Vec<char>], instructions_str: &str) {
    instructions_str.lines().for_each(|instruction_line| {
        let instruction = get_instructions(instruction_line);
        let (take_index, give_index) = (instruction[1] - 1, instruction[2] - 1);
        let mut taken_crates = vec![];

        for _ in 0..instruction[0] {
            taken_crates.insert(0, crate_stacks[take_index]
                .pop()
                .expect("instruction invalid : no crate left in stack"));
        }

        crate_stacks[give_index].append(&mut taken_crates);
    })
}

fn get_message(crate_stacks: &[Vec<char>]) -> String {
    let mut message = String::new();

    for stack in crate_stacks.iter() {
        if let Some(top_crate) = stack.last() {
            message.push(*top_crate);
        }
    }

    message
}

#[cfg(test)]
mod tests {
    use super::{get_crate_stacks, get_crates_str_and_instructions_str, run_cratemover_9000, get_message, run_cratemover_9001};

    const TEST_INPUT: &str = include_str!("./test_input.txt");

    #[test]
    fn crate_stacks_are_parsed_correctly() {
        let expected = [
            Vec::from(['Z', 'N']),
            Vec::from(['M', 'C', 'D']),
            Vec::from(['P']),
        ];

        let crate_stacks = get_crate_stacks(get_crates_str_and_instructions_str(TEST_INPUT).0);

        for i in 0..expected.len() {
            assert_eq!(expected[i], crate_stacks[i])
        }
    }

    #[test]
    fn instructions_run_correctly() {
        let expected = [
            Vec::from(['C']),
            Vec::from(['M']),
            Vec::from(['P', 'D', 'N', 'Z']),
        ];

        let (crates_str, instructions_str) = get_crates_str_and_instructions_str(TEST_INPUT);

        let mut crate_stacks = get_crate_stacks(crates_str);

        run_cratemover_9000(crate_stacks.as_mut_slice(), instructions_str);

        for i in 0..crate_stacks.len() {
            assert_eq!(expected[i], crate_stacks[i])
        }

        assert_eq!("CMZ", get_message(crate_stacks.as_slice()))
    }

    #[test]
    fn instructions_run_correctly_under_cratemover_9001() {
        let expected = [
            Vec::from(['M']),
            Vec::from(['C']),
            Vec::from(['P','Z','N','D'])
        ];

        let (crates_str, instructions_str) = get_crates_str_and_instructions_str(TEST_INPUT);

        let mut crate_stacks = get_crate_stacks(crates_str);

        run_cratemover_9001(crate_stacks.as_mut_slice(), instructions_str);

        for i in 0..crate_stacks.len() {
            assert_eq!(expected[i], crate_stacks[i])
        }

        assert_eq!("MCD", get_message(crate_stacks.as_slice()))
    }
}
