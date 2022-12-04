use std::str::FromStr;

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Janken {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Janken {
    pub fn from_input(input: &str) -> Vec<(Janken, Janken)> {
        let mut games = vec![];
        input.lines().for_each(|line| {
            let (opponent_move, player_move) = line.split_once(' ').unwrap_or(("A", "X"));

            games.push((
                Janken::from_str(opponent_move).unwrap_or(Janken::Rock),
                Janken::from_str(player_move).unwrap_or(Janken::Rock),
            ));
        });

        games
    }

    pub fn from_input_part_2(input: &str) -> Vec<(Janken, Janken)> {
        let mut games = vec![];
        input.lines().for_each(|line| {
            let game = part_2(line);
            games.push(game);
        });

        games
    }
}

impl FromStr for Janken {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "A" | "X" => Ok(Janken::Rock),
            "B" | "Y" => Ok(Janken::Paper),
            "C" | "Z" => Ok(Janken::Scissors),
            _ => Err("invalid input".to_string()),
        }
    }
}