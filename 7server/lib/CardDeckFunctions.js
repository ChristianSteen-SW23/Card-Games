export { drawDeck, dealFromDeckToHand };

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
