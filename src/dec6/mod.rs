const INPUT: &str = include_str!("./input.txt");

pub fn dec6_part_one() {
    println!("start-of-packet marker : {}", find_start_of_packet_marker(INPUT));
}

pub fn dec6_part_two() {
    println!("start-of-message marker : {}", find_start_of_message_marker(INPUT));
}

fn find_start_of_packet_marker(input: &'static str) -> usize {
    find_marker(input, 4)
}

fn find_start_of_message_marker(input: &'static str) -> usize {
    find_marker(input, 14)
}

fn find_marker(input: &'static str, n: usize) -> usize {
    let mut string = String::with_capacity(4);

    input.chars().for_each(|char| {
        if string.len() < n {
            if let Some(index) = string.find(char) {
                string = string[(index + 1)..].to_string();
            }
            string.push(char)
        }
    });

    input.find(string.as_str()).unwrap() + n
}

#[cfg(test)]
mod tests {
    use super::{find_start_of_packet_marker, find_start_of_message_marker};

    const TEST_SLICES: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];


    #[test]
    fn start_of_packet_marker_is_correct() {
        let expected_starts: [usize; 5] = [7, 5, 6, 10, 11];
        for i in 0..5 {
            assert_eq!(expected_starts[i], find_start_of_packet_marker(TEST_SLICES[i]))
        }
    }

    #[test]
    fn start_of_message_marker_is_correct() {
        let expected_starts: [usize; 5] = [19, 23, 23, 29, 26];
        for i in 0..5 {
            assert_eq!(expected_starts[i], find_start_of_message_marker(TEST_SLICES[i]))
        }
    }
}
