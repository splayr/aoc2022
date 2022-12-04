use std::str::FromStr;

#[repr(u32)]
pub enum GameResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for GameResult {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(String::from("invalid char"))
        }
    }
}