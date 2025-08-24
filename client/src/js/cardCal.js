const rankDictionary = {
    1: "a",
    2: "2",
    3: "3",
    4: "4",
    5: "5",
    6: "6",
    7: "7",
    8: "8",
    9: "9",
    10: "10",
    11: "j",
    12: "q",
    13: "k"
};

function getRank(card) {
    return rankDictionary[card % 13 + 1];
}
const suitDictionary = {
    1: "diams",
    2: "spades",
    3: "hearts",
    4: "clubs"
};

function getSuit(card) {
    return suitDictionary[Math.floor(card / 13) + 1];
}
function getCardDetails(card) {
    const rank = getRank(card);
    const suit = getSuit(card);
    return { rank, suit };
}

export {getCardDetails, getSuit, getRank};