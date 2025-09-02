export { start500Game, call500Move };
import { drawDeck, dealFromDeckToHand, randomShuffle, isAdjacentCardsOverflow, isCardAdjacentToStack, areCardsAdjacentSet } from "../lib/CardDeckFunctions.js";
import { nextPlayer, swapOneTurn } from "../lib/TurnManagement.js";
import { countPointScore } from "../lib/PointCounter.js";
import { sendErrorMessage } from "../lib/InfoMessage.js";


function start500Game(roomData, socketID, io, roomID) {
    roomData.turn.current = socketID;
    roomData.turn.next = nextPlayer(roomData);
    roomData.gameStarted = true;
    roomData.gameData = {};

    dealCards500(roomData.gameData, roomData.players);

    let playersInfo = mapPlayerInfo500(roomData.players, roomData.gameData.players);
    let startedGameData = {
        playersInfo,
        turn: roomData.turn,
        stack: roomData.gameData.stack[0],
        stackSize: roomData.gameData.stack.length,
        deckSize: roomData.gameData.deck.length
    };

    for (let [playerID, player] of roomData.players) {
        let personnalData = { hand: roomData.gameData.players[playerID].hand, ...startedGameData };
        io.to(playerID).emit("startedGame500", personnalData); // Send data with the event
    }
}

function playNewRound(roomData, io) {
    dealCards500(roomData.gameData, roomData.players, false);
    let playersInfo = mapPlayerInfo500(roomData.players, roomData.gameData.players);
    let startedGameData = {
        playersInfo,
        turn: roomData.turn,
        stack: roomData.gameData.stack[0],
        stackSize: roomData.gameData.stack.length,
        deckSize: roomData.gameData.deck.length
    };

    for (let [playerID, player] of roomData.players) {
        console.log(playerID);
        let personnalData = { hand: roomData.gameData.players[playerID].hand, ...startedGameData };
        io.to(playerID).emit("newRound", personnalData); // Send data with the event
    }
}

function call500Move(roomData, socketData, playerID, io, roomID) {
    let moveType = socketData.moveType;
    switch (moveType) {
        case "draw":
            draw500Move(roomData, playerID, io, roomID, socketData.drawKind);
            break;
        case "endTurn":
            endTurnMove(roomData, socketData, playerID, io, roomID);
            break;
        case "playTrick":
            playTrickMove(roomData, socketData, playerID, io, roomID)
            break;
        case "playAgain":
            playNewRound(roomData, io)
            break;
    }
}

function draw500Move(roomData, playerID, io, roomID, drawKind) {
    switch (drawKind) {
        case "stacktop":
            drawStacktop(roomData.gameData, playerID, io);
            break;
        case "stack":
            drawStack(roomData.gameData, playerID, io);
            break;
        case "decktop":
            drawDecktop(roomData.gameData, playerID, io);
            break;
    }
    sendGameInformation(roomID, roomData, io, "trick");
    sendHandInformation(playerID, roomData, io);
}

function drawStacktop(gameData, playerID, io) {
    if (gameData.stack.length === 0) return sendErrorMessage(playerID, io, "The stack is empty", "Draw");

    let stacktopCard = gameData.stack.pop();
    gameData.players[playerID].hand.push(stacktopCard);
}

function drawStack(gameData, playerID, io) {
    if (gameData.stack.length === 0) return sendErrorMessage(playerID, io, "The stack is empty", "Draw");

    gameData.players[playerID].needsToTrick = true;
    gameData.players[playerID].hand = [...gameData.players[playerID].hand, ...gameData.stack];
    gameData.stack = [];
}

function drawDecktop(gameData, playerID, io) {
    if (gameData.deck.length === 0) return sendErrorMessage(playerID, io, "The deck is empty", "Draw");

    let decktopCard = gameData.deck.pop();
    gameData.players[playerID].hand.push(decktopCard);
}

function endTurnMove(roomData, socketData, playerID, io, roomID) {
    let playerHand = roomData.gameData.players[playerID].hand;
    // No cards left in hand
    if (playerHand.length === 0) return nextPlayerTurn(roomData, socketData, playerID, io, roomID);//TODO send request

    let playedCard = socketData.cardToPlay;
    if (playerHand.includes(playedCard)) {
        let index = playerHand.indexOf(playedCard); // Find index of the value
        playerHand.splice(index, 1); // Remove the value
    } else {
        return; //TODO you can not play that card
    }

    roomData.gameData.stack.push(playedCard);
    nextPlayerTurn(roomData, socketData, playerID, io, roomID);

    if (playerHand.length === 0) {
        return sendGameOver(roomID, updateScore(roomData), io);
    }
}

function updateScore(roomData) {
    let scoreData = [];

    for (const [key, value] of Object.entries(roomData.gameData.players)) {
        let score = countPointScore(value.tricks, 10, 15, 5) + value.gamePoints;
        roomData.gameData.players[key].totalScore += score;
        let curScore = {
            name: roomData.players.get(key).name,
            totalScore: roomData.gameData.players[key].totalScore,
            roundScore: score
        };
        scoreData.push(curScore);
    }

    return scoreData;
}

function nextPlayerTurn(roomData, socketData, playerID, io, roomID) {
    if (roomData.gameData.players[playerID].needsToTrick) {
        roomData.gameData.players[playerID].gamePoints += -50;
        roomData.gameData.players[playerID].needsToTrick = false;
    }
    swapOneTurn(roomData);
    sendGameInformation(roomID, roomData, io, "draw");
    sendHandInformation(playerID, roomData, io);
}


function playTrickMove(roomData, socketData, playerID, io, roomID) {
    if (socketData.cardsToPlay.length == 0) return sendErrorMessage(playerID, io, "No Card Picked", "Trick");

    if (socketData.cardsToPlay.length == 1) return singleTrick(roomData, socketData.cardsToPlay[0], playerID, io, roomID);

    multiTrick(roomData, socketData.cardsToPlay, playerID, io, roomID);
}

function multiTrick(roomData, trickCards, playerID, io, roomID) {
    if (!areCardsAdjacentSet(trickCards) || trickCards.length < 3) return sendErrorMessage(playerID, io, "Multi Trick Not Possible", "Trick");

    let hand = roomData.gameData.players[playerID].hand;
    let missingCards = trickCards.filter(card => !hand.includes(card));
    if (missingCards.length > 0) {
        return sendErrorMessage(playerID, io, `Missing Cards: ${missingCards.join(", ")}`, "Trick");
    }

    trickCards.forEach(card => {
        hand.splice(hand.indexOf(card), 1);
    });

    roomData.gameData.players[playerID].tricks.push(...trickCards)

    roomData.gameData.players[playerID].needsToTrick = false;

    sendGameAndHandInformation(roomID, playerID, roomData, io);
}

function singleTrick(roomData, trickCard, playerID, io, roomID) {
    let gameData = roomData.gameData;
    let possibleTrick = false;
    for (const player of Object.values(gameData.players)) {
        if (isCardAdjacentToStack(trickCard, player.tricks)) {
            possibleTrick = true;
            break;
        }
    }

    if (!possibleTrick) return sendErrorMessage(playerID, io, "Single Trick Not Possible", "Trick");

    let hand = gameData.players[playerID].hand;
    let tricks = gameData.players[playerID].tricks;
    let index = hand.indexOf(trickCard);
    if (index === -1) return sendErrorMessage(playerID, io, "Card Not Found In Hand (Error Bug)", "Trick");
    hand.splice(index, 1);
    tricks.push(trickCard);

    sendGameAndHandInformation(roomID, playerID, roomData, io);
}

function sendGameOver(roomID, data, io) {
    io.to(roomID).emit("gameEnded", { contineAllowed: false, winData: data });
}



function sendGameAndHandInformation(roomID, playerID, roomData, io) {
    sendGameInformation(roomID, roomData, io, "trick");
    sendHandInformation(playerID, roomData, io);
}

function sendGameInformation(roomID, roomData, io, gameStep = "draw") {
    const playersInfo = mapPlayerInfo500(roomData.players, roomData.gameData?.players);
    const stackSize = roomData.gameData.stack.length - 1;
    const curGameData = {
        playersInfo,
        turn: roomData.turn,
        stack: (!(stackSize < 0)) ? roomData.gameData.stack[stackSize] : -1,
        stackSize: roomData.gameData.stack.length,
        deckSize: roomData.gameData.deck.length,
        gameStep: gameStep
    };

    io.to(roomID).emit("gameInformation", curGameData);
}

function sendHandInformation(playerID, roomData, io) {
    const data = { hand: roomData.gameData.players[playerID].hand }
    io.to(playerID).emit("handInformation", data);
}

function mapPlayerInfo500(map, gameData) {
    let array = [];
    for (const [key, value] of map.entries()) {
        array.push({
            name: value.name,
            id: key,
            handSize: gameData[key].hand.length,
            tricks: gameData[key].tricks,
            totalScore: gameData[key].totalScore,
        });
    }
    return array;
}

function dealCards500(gameData, players, firstGame = true) {
    drawDeck(gameData);
    if (firstGame) gameData.players = {};

    for (let [playerID, player] of players) {
        if (firstGame)
            gameData.players[playerID] = { hand: [0, 1, 2, 3, 10, 11, 12], tricks: [], needsToTrick: false, gamePoints: 0, totalScore: 0 };
        else
            gameData.players[playerID] = { hand: [], tricks: [], needsToTrick: false, gamePoints: 0, totalScore: gameData.players[playerID].totalScore };
        //dealFromDeckToHand(gameData.players[playerID].hand, gameData.deck, 7);
    }
    gameData.stack = [];
    dealFromDeckToHand(gameData.stack, gameData.deck, 1);
    randomShuffle(gameData.deck);
}
