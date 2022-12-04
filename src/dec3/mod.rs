use std::collections::HashMap;

pub fn dec3_part_one() {
    let priority_hashmap = get_priority_hashmap();
    let input = include_str!("./input.txt");

    let rucksacks = get_rucksacks(input);
    let compartiments = get_compartiments(rucksacks.as_slice());
    let shared_items = check_all_compartiments(compartiments.as_slice());
    let priorities = get_all_priorities(&priority_hashmap, shared_items.as_slice());

    println!(
        "sum of shared items priorities = {}",
        priorities.iter().sum::<u32>()
    )
}

pub fn dec3_part_two() {
    let priority_hashmap = get_priority_hashmap();
    let input = include_str!("./input.txt");

    let rucksack_groups = get_rucksacks_by_group_of_elves(input);
    let badges = get_all_badges(rucksack_groups.as_slice());
    let priorities = get_all_priorities(&priority_hashmap, badges.as_slice());

    println!(
        "sum of badges priorities = {}",
        priorities.iter().sum::<u32>()
    )
}

fn get_rucksacks(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn get_rucksacks_by_group_of_elves(input: &str) -> Vec<[&str; 3]> {
    let rucksacks = get_rucksacks(input);
    let mut rucksack_groups: Vec<[&str; 3]> = vec![];

    rucksacks
        .chunks_exact(3)
        .for_each(|group| rucksack_groups.push([group[0], group[1], group[2]]));

    rucksack_groups
}

fn get_badge_from_group(rucksack_group: &[&str; 3]) -> char {
    let mut hashmap: HashMap<char, u8> = HashMap::new();

    rucksack_group[0].chars().for_each(|item| {
        hashmap.insert(item, 1);
    });

    rucksack_group[1].chars().for_each(|item| {
        if let Some((_, count)) = hashmap.get_key_value(&item) {
            if count == &1 {
                hashmap.insert(item, 2);
            };
        }
    });

    rucksack_group[2].chars().for_each(|item| {
        if let Some((_, count)) = hashmap.get_key_value(&item) {
            if count == &2 {
                hashmap.insert(item, 3);
            };
        }
    });

    *hashmap.iter().find(|hash| *hash.1 == 3).unwrap().0
}

fn get_all_badges(rucksack_groups: &[[&str; 3]]) -> Vec<char> {
    rucksack_groups.iter().map(get_badge_from_group).collect()
}

fn get_compartiments<'a>(rucksacks: &'a [&str]) -> Vec<(&'a str, &'a str)> {
    rucksacks
        .iter()
        .map(|line| line.split_at(line.len() >> 1))
        .collect()
}

fn check_compartiment<'a>(compartiment: (&'a str, &'a str)) -> char {
    let mut hashmap = HashMap::new();
    let mut shared_item = ' ';

    compartiment.0.chars().for_each(|item| {
        hashmap.insert(item, ());
    });

    compartiment.1.chars().for_each(|item| {
        if hashmap.get(&item).is_some() {
            shared_item = item;
        };
    });

    shared_item
}

fn check_all_compartiments<'a>(compartiments: &[(&'a str, &'a str)]) -> Vec<char> {
    compartiments
        .iter()
        .map(|compartiment| check_compartiment(*compartiment))
        .collect()
}

fn get_priority_hashmap() -> HashMap<char, u8> {
    /*
    To help prioritize item rearrangement, every item type can be converted to a priority:

    Lowercase item types a through z have priorities 1 through 26.
    Uppercase item types A through Z have priorities 27 through 52.
    */

    let mut hashmap = HashMap::new();

    let lowercase = b'a'..=b'z';
    let uppercase = b'A'..=b'Z';

    for i in lowercase {
        hashmap.insert(char::from_u32(i as u32).unwrap(), i - b'a' + 1);
    }

    for i in uppercase {
        hashmap.insert(char::from_u32(i as u32).unwrap(), i - b'A' + 27);
    }

    hashmap
}

fn get_priority(priority_hashmap: &HashMap<char, u8>, shared_item: &char) -> u32 {
    *priority_hashmap.get(shared_item).unwrap_or(&0) as u32
}

fn get_all_priorities(priority_hashmap: &HashMap<char, u8>, shared_items: &[char]) -> Vec<u32> {
    shared_items
        .iter()
        .map(|item| get_priority(priority_hashmap, item))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        check_all_compartiments, get_all_badges, get_all_priorities, get_compartiments,
        get_priority_hashmap, get_rucksacks, get_rucksacks_by_group_of_elves,
    };

    const TEST_INPUT: &str = include_str!("./test_input.txt");

    /*
    One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.
    Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments. The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.
    The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors. Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).
    The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.
    */

    #[test]
    fn shared_item_is_correct() {
        /*
        The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
        The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both compartments is uppercase L.
        The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
        The fourth rucksack's compartments only share item type v.
        The fifth rucksack's compartments only share item type t.
        The sixth rucksack's compartments only share item type s.
        */
        let rucksacks = get_rucksacks(TEST_INPUT);
        let compartiments = get_compartiments(rucksacks.as_slice());

        let expected_shared_items = ['p', 'L', 'P', 'v', 't', 's'];
        let shared_items = check_all_compartiments(compartiments.as_slice());

        for i in 0..6usize {
            assert_eq!(expected_shared_items[i], shared_items[i]);
        }
    }

    #[test]
    fn priorities_are_correct() {
        // In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.
        //16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

        let property_hashmap = get_priority_hashmap();
        let rucksacks = get_rucksacks(TEST_INPUT);
        let compartiments = get_compartiments(rucksacks.as_slice());
        let shared_items = check_all_compartiments(compartiments.as_slice());

        let expected_priorities = [16, 38, 42, 22, 20];
        let priorities = get_all_priorities(&property_hashmap, shared_items.as_slice());

        for i in 0..5usize {
            assert_eq!(expected_priorities[i], priorities[i])
        }

        assert_eq!(157, priorities.iter().sum::<u32>())
    }

    #[test]
    fn groups_are_correct() {
        let group1 = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ];

        let group2 = [
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];

        let rucksack_groups = get_rucksacks_by_group_of_elves(TEST_INPUT);

        assert_eq!(group1, rucksack_groups[0]);
        assert_eq!(group2, rucksack_groups[1]);
    }

    #[test]
    fn badges_are_correct() {
        /*
        In the first group, the only item type that appears in all three rucksacks is lowercase r; this must be their badges.
        In the second group, their badge item type must be Z.
        */

        let rucksack_groups = get_rucksacks_by_group_of_elves(TEST_INPUT);
        let badges = get_all_badges(rucksack_groups.as_slice());

        assert_eq!('r', badges[0]);
        assert_eq!('Z', badges[1]);
    }

    #[test]
    fn priorities_for_badges_are_correct() {
        /*
        Priorities for these items must still be found to organize the sticker attachment efforts:
        here, they are 18 (r) for the first group and 52 (Z) for the second group.
        The sum of these is 70.
        */

        let priority_hashmap = get_priority_hashmap();
        let rucksack_groups = get_rucksacks_by_group_of_elves(TEST_INPUT);
        let badges = get_all_badges(rucksack_groups.as_slice());
        let priorities = get_all_priorities(&priority_hashmap, badges.as_slice());

        assert_eq!(18, priorities[0]);
        assert_eq!(52, priorities[1]);
        assert_eq!(70, priorities.iter().sum::<u32>());
    }
}
