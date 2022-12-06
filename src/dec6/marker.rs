/**
 * UNUSED
 * this is unused, i just uploaded it to show how to newtype pattern
 */

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Marker(String);

impl FromStr for Marker {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut string = String::with_capacity(4);

        s.chars().try_for_each(|char| {
            if let Some(index) = string.find(char) {
                string = string[(index+1)..].to_string()
            }
            if string.len() < 4 {
                string.push(char)
            }
        });

        if string.len() < 4 {
            Err(String::from("marker was not found."))
        } else {
            Ok(Marker(string))
        }
    }
}
