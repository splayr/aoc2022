use std::ops::RangeInclusive;

const INPUT: &str = include_str!("./input.txt");

pub fn dec4_part_one() {
    let clearances = get_pair_of_elves_with_clearance(INPUT);
    let number_subranges =
        get_number_of_assignment_pairs_where_one_contains_the_other(clearances.as_slice());
    println!(
        "number of assignment where subranges : {}",
        number_subranges
    )
}

pub fn dec4_part_two() {
    let clearances = get_pair_of_elves_with_clearance(INPUT);
    let number_overlaps = get_number_of_overlapping_assignement_pairs(clearances.as_slice());
    println!("number of assignment where overlaps : {}", number_overlaps)
}

fn get_pair_of_elves_with_clearance(input: &str) -> Vec<(RangeInclusive<u8>, RangeInclusive<u8>)> {
    input
        .lines()
        .map(|line| {
            let (elf1, elf2) = line.split_once(',').unwrap();
            let elf1_clearance = elf1.split_once('-').unwrap();
            let elf2_clearance = elf2.split_once('-').unwrap();

            let elf1_clearance = (
                elf1_clearance.0.parse::<u8>().unwrap(),
                elf1_clearance.1.parse::<u8>().unwrap(),
            );
            let elf2_clearance = (
                elf2_clearance.0.parse::<u8>().unwrap(),
                elf2_clearance.1.parse::<u8>().unwrap(),
            );

            (
                elf1_clearance.0..=elf1_clearance.1,
                elf2_clearance.0..=elf2_clearance.1,
            )
        })
        .collect()
}

fn check_clearance_subrange(clearance_pair: &(RangeInclusive<u8>, RangeInclusive<u8>)) -> bool {
    clearance_pair
        .0
        .clone()
        .all(|i| clearance_pair.1.contains(&i))
        || clearance_pair
            .1
            .clone()
            .all(|i| clearance_pair.0.contains(&i))
}

fn check_clearance_overlap(clearance_pair: &(RangeInclusive<u8>, RangeInclusive<u8>)) -> bool {
    clearance_pair
        .0
        .clone()
        .any(|i| clearance_pair.1.contains(&i))
        || clearance_pair
            .1
            .clone()
            .any(|i| clearance_pair.0.contains(&i))
}

fn get_number_of_assignment_pairs_where_one_contains_the_other(
    clearances: &[(RangeInclusive<u8>, RangeInclusive<u8>)],
) -> usize {
    clearances
        .iter()
        .filter(|pair| check_clearance_subrange(pair))
        .count()
}

fn get_number_of_overlapping_assignement_pairs(
    clearances: &[(RangeInclusive<u8>, RangeInclusive<u8>)],
) -> usize {
    clearances
        .iter()
        .filter(|pair| check_clearance_overlap(pair))
        .count()
}

#[cfg(test)]
mod tests {
    use super::{
        get_number_of_assignment_pairs_where_one_contains_the_other,
        get_number_of_overlapping_assignement_pairs, get_pair_of_elves_with_clearance,
    };

    const TEST_INPUT: &str = include_str!("./test_input.txt");

    /*
    For the first few pairs, this list means:

    Within the first pair of Elves, the first Elf was assigned sections 2-4 (sections 2, 3, and 4), while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
    The Elves in the second pair were each assigned two sections.
    The Elves in the third pair were each assigned three sections: one got sections 5, 6, and 7, while the other also got 7, plus 8 and 9.
    This example list uses single-digit section IDs to make it easier to draw; your actual list might contain larger numbers. Visually, these pairs of section assignments look like this:
    */

    #[test]
    fn clearance_range_is_correct() {
        let expected_clearances = [
            (2..=4, 6..=8),
            (2..=3, 4..=5),
            (5..=7, 7..=9),
            (2..=8, 3..=7),
            (6..=6, 4..=6),
            (2..=6, 4..=8),
        ];

        let clearances = get_pair_of_elves_with_clearance(TEST_INPUT);

        for i in 0..expected_clearances.len() {
            assert_eq!(expected_clearances[i], clearances[i]);
        }
    }

    #[test]
    fn number_of_subranges_of_clearances_are_correct() {
        /*
        Some of the pairs have noticed that one of their assignments fully contains the other.
        For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6.
        In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration.
        In this example, there are 2 such pairs.
        */

        let clearances = get_pair_of_elves_with_clearance(TEST_INPUT);
        let number_subranges =
            get_number_of_assignment_pairs_where_one_contains_the_other(clearances.as_slice());

        assert_eq!(2, number_subranges)
    }

    #[test]
    fn number_of_overlapping_clearance_pairs_are_correct() {
        /*
        In the above example, the first two pairs (2-4,6-8 and 2-3,4-5) don't overlap, while the remaining four pairs (5-7,7-9, 2-8,3-7, 6-6,4-6, and 2-6,4-8) do overlap:
        5-7,7-9 overlaps in a single section, 7.
        2-8,3-7 overlaps all of the sections 3 through 7.
        6-6,4-6 overlaps in a single section, 6.
        2-6,4-8 overlaps in sections 4, 5, and 6.
        */

        let clearances = get_pair_of_elves_with_clearance(TEST_INPUT);
        let number_overlaps = get_number_of_overlapping_assignement_pairs(clearances.as_slice());

        assert_eq!(4, number_overlaps)
    }
}
