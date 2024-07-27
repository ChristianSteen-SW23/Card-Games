import { PlayerRooms, Rooms } from "./index.js";
export { drawHand, removeCardFromHand, checkWinner, mapToPlayerLives, nextPlayer, switchRoles, drawCard, computeOppPerformance };

// create the starting hand for a player
function drawHand(deck, handSize) {
    const avgDeckRating = deck.cards.reduce((ratingSum, card) => ratingSum + card.rating, 0) / deck.cards.length;
    //randomize the rating so that it can vary between 1 and -1 from original value
    let randomRating = (avgDeckRating + (Math.random() * 2) - 1);
    //make sure rating does not become bigger than 5 or smaller than 1
    randomRating = Math.min(Math.max(randomRating, 1), 5);

    //Makes an array of cards adding the index from the deck
    const cards = deck.cards.map((card, index) => ({ ...card, index: index }));

    //Sorts the cards after the randomized rating that puts the cards, which rating closes matches the random rating, at the end
    const sortFunc = (a, b) => Math.abs(randomRating - b.rating) - Math.abs(randomRating - a.rating);
    const cardsSorted = cards.sort(sortFunc);
    //The indexes of the cards that closes match the random rating are added to the hand array. 
    let hand = [];
    for (let i = 0; i < handSize; i++) {
        hand.push(cardsSorted.pop().index);
    }
    return hand;
}

function removeCardFromHand(playerID, usedIndex, roomID) {
    let roomPlayers = Rooms.get(roomID).players;
    let updatedHand = [...roomPlayers.get(playerID).hand];
    roomPlayers.get(playerID).usedCards.push(updatedHand[usedIndex]);
    updatedHand.splice(usedIndex, 1);
    roomPlayers.get(playerID).hand = [...updatedHand];
}

function drawCard(oppPerformance, deck, usedCards, handCards, maxLives) {
    let newCardRating = new Array(2);
    //check if the new card on the hand should be harder or easier
    if (oppPerformance >= 0) {
        const middlePerformance = maxLives / 2;
        if (oppPerformance >= middlePerformance) {
            newCardRating[0] = 5;
            newCardRating[1] = 4;
        }
        else {
            newCardRating[0] = 4;
            newCardRating[1] = 3;
        }
    }
    else {
        const middlePerformance = -1 * (maxLives / 2);
        if (oppPerformance > middlePerformance) {
            newCardRating[0] = 3;
            newCardRating[1] = 2;
        } else {
            newCardRating[0] = 2;
            newCardRating[1] = 1;
        }
    }

    //make an array where each card also has their index as a key/value pair
    const cards = deck.cards.map((card, index) => ({ ...card, index: index }));
    //find all the cards that have not been played
    const unusedCards = cards.filter(card => !(handCards.includes(card.index) || usedCards.includes(card.index)));
    //find cards that match the rating we want
    const candidates = unusedCards.filter(card => card.rating === newCardRating[0] || card.rating === newCardRating[1]);

    if (candidates.length === 0) {
        return unusedCards[Math.floor(Math.random() * unusedCards.length)].index;
    }
    return candidates[Math.floor(Math.random() * candidates.length)].index;
}

function mapToPlayerLives(map) {
    let array = [];
    for (const [key, value] of map.entries()) {
        array.push({
            name: value.name,
            id: key,
            lives: value.lives
        });
    }
    return array;
}

function checkWinner(roomID, roomData, socket, io) {
    const players = [...roomData.players.values()];

    // check if theres only one player left, declare that player as winner.
    if (players.filter(player => player.lives !== 0).length === 1) {
        socket.emit("foundWinner", "won");
        socket.to(roomID).emit("foundWinner", "lose");
        return true;
    }

    // Draw card for player if any is left and their played cards does not exceed the maxDeckSize for the room.
    let player = Rooms.get(roomID).players.get(socket.id);
    if (roomData.maxDeckSize > player.usedCards.length + player.hand.length) {
        const oppPerformance = computeOppPerformance(roomData, socket.id);
        let pickedCard = drawCard(oppPerformance, player.deck, player.usedCards, player.hand, roomData.settings.life);
        player.hand.push(pickedCard);
    }

    // If no cards left, give win to player with most lives, or end with draw for remaining players.
    if (!players.some((player) => player.lives > 0 && player.hand.length > 0)) {
        // Sort players descending by lives.
        const playersSorted = players.sort((a, b) => b.lives - a.lives);
        const winLivesAmount = playersSorted[0].lives;
        const multipleDraws = playersSorted[1].lives === winLivesAmount;
        playersSorted.forEach(player => {
            // Player Won / Draw
            if (player.lives === winLivesAmount) {
                if (multipleDraws) {
                    io.to(player.id).emit("foundWinner", "draw");
                } else {
                    io.to(player.id).emit("foundWinner", "won");
                }
                // Player lost
            } else {
                io.to(player.id).emit("foundWinner", "lose");
            }
        });
        return true;
    }

    return false;
}

function nextPlayer(roomData) {
    let playersLeft = mapToPlayerLives(roomData.players).filter(player => player.lives !== 0);
    let currentIndex = playersLeft.findIndex(player => roomData.turn.current === player.id);
    return playersLeft[(currentIndex + 1) % playersLeft.length].id;
}

// Assign new player to select card and new player to answer.
function switchRoles(roomID, roomData, socket) {
    // Check if current next player is alive, else they should be skipped.
    if (roomData.players.get(roomData.turn.next).lives === 0) {
        // Set next player to a player alive.
        roomData.turn.next = nextPlayer(roomData);
    }
    // Shift players alive
    roomData.turn.current = roomData.turn.next;
    roomData.turn.next = nextPlayer(roomData);

    socket.to(roomID).emit("switchRoles", { turn: roomData.turn });
    socket.emit("switchRoles", { turn: roomData.turn, hand: roomData.players.get(socket.id).hand });
}

function computeOppPerformance(roomData, playerID) {
    const opponentID = roomData.turn.next;
    const players = Rooms.get(PlayerRooms.get(opponentID)).players;
    const opponent = players.get(opponentID);
    const player = players.get(playerID);

    const playerArr = mapToPlayerLives(players);
    const avgPerformance = playerArr.reduce((sum, player) => sum + player.lives, 0) / playerArr.length;

    // The opponents performance is based on difference from you and the difference from the average 
    return (opponent.lives - player.lives) + (opponent.lives - avgPerformance);
}
