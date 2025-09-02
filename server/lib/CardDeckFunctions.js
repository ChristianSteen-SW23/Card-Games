export {
  drawDeck,
  dealFromDeckToHand,
  randomShuffle,
  isAdjacentCardsNoOverflow,
  isAdjacentCardsOverflow,
  isCardAdjacentToStack,
  isSameSuit,
  areCardsAdjacentSet,
};

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

function areCardsAdjacentSet(cards) {
  if (cards.length < 2) return 1;

  // Sortes by checking if an overflow occurs. If it does,
  //then the cards after the overflow are placed before the other cards.  
  cards.sort((a, b) => a - b);
  let stack = [...cards]; // Copy to avoid modifying original array
  for (let i = 0; i < stack.length - 1; i++)
    if (!(stack[i] + 1 === stack[i + 1])) {
      stack = [...stack.slice(i + 1), ...stack.slice(0, i + 1)];
      break;
    }

  for (let i = 0; i < stack.length - 1; i++)
    if (!isAdjacentCardsOverflow(stack[i], stack[i + 1]))
      return 0;
  return 1;
}

function isCardAdjacentToStack(card, stack) {
  for (const curCard of stack) {
    if (isAdjacentCardsOverflow(card, curCard)) return 1;
  }
  return 0;
}

function isAdjacentCardsOverflow(cardA, cardB) {
  if (!isSameSuit(cardA, cardB)) return 0;

  const rankA = cardA % 13;
  const rankB = cardB % 13;
  return (rankA + 1 === rankB || rankA - 1 === rankB || (rankA === 0 && rankB === 12) || (rankA === 12 && rankB === 0)) ? 1 : 0;
}

function isAdjacentCardsNoOverflow(cardA, cardB) {
  if (!isSameSuit(cardA, cardB)) return 0;

  return (cardA + 1 == cardB || cardA - 1 == cardB) ? 1 : 0;
}

function isSameSuit(cardA, cardB) {
  return (cardA - cardA % 13 == cardB - cardB % 13) ? 1 : 0;
}
