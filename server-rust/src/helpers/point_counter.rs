
pub fn count_point_score(cards: &Vec<u32>, picture: u32, ace: u32, number: u32) -> u32 {
    cards.iter().fold(0, |accumulator, &card| {
        accumulator
            + if card % 13 == 0 {
                ace
            } else if card % 13 <= 12 && card % 13 >= 10 {
                picture
            } else {
                number
            }
    })
}

