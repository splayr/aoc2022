mod game_result;
mod janken;

use std::str::FromStr;

use janken::Janken;

use self::game_result::GameResult;

/*
In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
*/

pub fn dec2() {
    let input = include_str!("./input.txt");
    let games = Janken::from_input(input);

    // In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).
    let (_, player_totals) = get_totals(games.as_slice());
    println!("player total part 1 : {player_totals}");

    let games = Janken::from_input_part_2(input);
    let (_, player_totals) = get_totals(games.as_slice());
    println!("player total part 2 : {player_totals}")
}

fn get_winner(p1: &Janken, p2: &Janken) -> GameResult {
    if p2 == p1 {
        GameResult::Draw
    } else if p2 > p1 {
        if p1 == &Janken::Rock && p2 == &Janken::Scissors {
            GameResult::Lose
        } else {
            GameResult::Win
        }
    } else if p1 == &Janken::Scissors && p2 == &Janken::Rock {
        GameResult::Win
    } else {
        GameResult::Lose
    }
}

fn get_winner_janken(opponent: &Janken, result: &GameResult) -> Janken {
    match (opponent, result) {
        (&Janken::Rock, &GameResult::Win) => Janken::Paper,
        (&Janken::Paper, &GameResult::Win) => Janken::Scissors,
        (&Janken::Scissors, &GameResult::Win) => Janken::Rock,
        (&Janken::Rock, &GameResult::Lose) => Janken::Scissors,
        (&Janken::Paper, &GameResult::Lose) => Janken::Rock,
        (&Janken::Scissors, &GameResult::Lose) => Janken::Paper,
        (opponent, &GameResult::Draw) => *opponent,
    }
}

fn get_points(opponent: &Janken, player: &Janken) -> (u32, u32) {
    let mut opponent_points = 0u32;
    let mut player_points = 0u32;

    opponent_points += *opponent as u32;
    player_points += *player as u32;

    opponent_points += get_winner(player, opponent) as u32;
    player_points += get_winner(opponent, player) as u32;

    (opponent_points, player_points)
}

fn get_totals(games: &[(Janken, Janken)]) -> (u32, u32) {
    let mut opponent_total = 0u32;
    let mut player_total = 0u32;

    games.iter().for_each(|game| {
        let (opponent_points, player_points) = get_points(&game.0, &game.1);
        opponent_total += opponent_points;
        player_total += player_points;
    });

    (opponent_total, player_total)
}

fn part_2(janken_pair: &str) -> (Janken, Janken) {
    let (opponent, result_str) = janken_pair.split_once(' ').unwrap();

    let opponent_janken = Janken::from_str(opponent).unwrap();
    let game_result = GameResult::from_str(result_str).unwrap();

    (
        opponent_janken,
        get_winner_janken(&opponent_janken, &game_result),
    )
}

#[cfg(test)]
mod tests {
    use crate::dec2::get_totals;

    use super::Janken;

    const INPUT: &str = r#"A Y
B X
C Z"#;

    #[test]
    fn points_calculation_is_correct() {
        let games = Janken::from_input(INPUT);

        // In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).
        let (_, player_totals) = get_totals(games.as_slice());
        assert_eq!(player_totals, 15);
    }
}
