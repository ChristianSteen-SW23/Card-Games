export {
    countPointScore
};


function countPointScore(cards = [], picture, ace, number) {
    return cards.reduce((accumulator, card) =>
        accumulator + (
            (card % 13 == 0) ? ace :
                (card % 13 <= 12 && card % 13 >= 10)
                    ? picture : number), 0);
}
