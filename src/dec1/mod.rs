pub fn dec1() {
    let input: &str = include_str!("./input.txt");

    let elves_groceries = get_groceries_for_each_elf(input);
    let elves_cal = get_calories_for_each_elf(elves_groceries);

    println!("total of elf with most calories: {}", elves_cal.iter().max().unwrap_or(&0));
    println!("total of top 3 elves with most calories: {}", elves_cal[..3].iter().sum::<u32>());
}

fn get_groceries_for_each_elf(input: &str) -> Vec<&str> {
    input.split("\r\n\r\n").collect()
}

fn get_calories_for_each_elf(groceries: Vec<&str>) -> Vec<u32> {
    let mut cals: Vec<u32> = groceries
        .iter()
        .map(|elf| {
            elf.lines()
                .filter_map(|cal| cal.parse::<u32>().ok())
                .sum::<u32>()
        })
        .collect();

        cals.sort_by(|a, b| b.cmp(a));

        cals
}