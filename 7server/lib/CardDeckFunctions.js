export { drawDeck, dealFromDeckToHand, randomShuffle };

function drawDeck(deckVar, jokerAmount = 0) {
    let deck = []
    for (let i = 0; i < 52 + jokerAmount; i++) {
        deck.push(i)
    }
    deckVar.deck = deck;
}

function dealFromDeckToHand(placement, deck, amount) {
    let i = 0;
    while (i < amount && deck.length > 0) {
        let randomNum = Math.floor(Math.random() * deck.length);
        placement.push(deck[randomNum]);
        deck.splice(randomNum, 1);
        i++;
    }
}

// Makes use of Fisher-Tates Shuffle
function randomShuffle(deck) {
    for (let i = deck.length - 1; i > 0; i--) {
        let randomIndex = Math.floor(Math.random() * (i + 1));
        [deck[i], deck[randomIndex]] = [deck[randomIndex], deck[i]]; // Swap elements
    }
    return deck;
}
