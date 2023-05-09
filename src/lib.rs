use std::iter::once;

mod tests;

pub fn get_diamond(c: char) -> Vec<String> {
    let chars = ('A'..=(c.to_uppercase().next().unwrap())).collect::<String>();
    let diamond = chars
        .chars()
        .enumerate()
        .map(|(index, char)| {
            let left_side = " "
                .repeat(chars.len() - (index + 1))
                .chars()
                .chain(once(char))
                .chain(" ".repeat(index).chars())
                .collect::<String>();
            left_side
                .clone()
                .chars()
                .chain(left_side.chars().rev().skip(1))
                .collect::<String>()
        })
        .collect::<Vec<String>>();
    diamond
        .clone()
        .into_iter()
        .chain(diamond.into_iter().rev().skip(1))
        .collect()
}
