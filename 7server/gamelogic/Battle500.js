export { start500Game, call500Move };
import { drawDeck, dealFromDeckToHand, randomShuffle } from "../lib/CardDeckFunctions.js";
import { nextPlayer, swapOneTurn } from "../lib/TurnManagement.js";


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
    };

    for (let [playerID, player] of roomData.players) {
        let personnalData = { hand: roomData.gameData.players[playerID].hand, ...startedGameData }; // ✅ Correct way to access player hand
        io.to(playerID).emit("startedGame500", personnalData); // Send data with the event
    }

    // io.to(roomID).emit("startedGame500", startedGameData);
    // console.log("Started game 500 made by", socketID);
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
            break;
    }
}

function draw500Move(roomData, playerID, io, roomID, drawKind) {
    switch (drawKind) {
        case "stacktop":
            drawStacktop(roomData.gameData, playerID);
            break;
        case "stack":
            drawStack(roomData.gameData, playerID);
            break;
        case "decktop":
            drawDecktop(roomData.gameData, playerID);
            break;
    }
    sendGameInformation(roomID, roomData, io);
    sendHandInformation(playerID, roomData, io);
}

function drawStacktop(gameData, playerID) {
    if (gameData.stack.length === 0) return;//TODO SEND ERROR

    let stacktopCard = gameData.stack.pop();
    gameData.players[playerID].hand.push(stacktopCard);
}

function drawStack(gameData, playerID) {
    if (gameData.stack.length === 0) return;//TODO SEND ERROR

    gameData.players[playerID].needsToTrick = true;
    gameData.players[playerID].hand = [...gameData.players[playerID].hand, ...gameData.stack];
    gameData.stack = [];
}

function drawDecktop(gameData, playerID) {
    if (gameData.deck.length === 0) return;//TODO SEND ERROR

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

    if (playerHand.length === 0) return sendGameOver(roomID, roomData, io);
}

function nextPlayerTurn(roomData, socketData, playerID, io, roomID) {
    if (roomData.gameData.players[playerID].needsToTrick) {
        roomData.gameData.players[playerID].gamePoints += -50;
        roomData.gameData.players[playerID].needsToTrick = false;
    }
    swapOneTurn(roomData);
    sendGameInformation(roomID, roomData, io);
    sendHandInformation(playerID, roomData, io);
}


function playTrickMove(roomData, socketData, socketID, io, roomID) {


}

function sendGameOver(roomID, roomData, io) {
    io.to(roomID).emit("gameEnded", { contineAllowed: false });
}


function sendGameInformation(roomID, roomData, io) {
    const playersInfo = mapPlayerInfo500(roomData.players, roomData.gameData?.players);
    const stackSize = roomData.gameData.stack.length - 1;
    const curGameData = {
        playersInfo,
        turn: roomData.turn,
        stack: (!(stackSize < 0)) ? roomData.gameData.stack[stackSize] : -1,
        stackSize: roomData.gameData.stack.length,
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
        });
    }
    return array;
}

function dealCards500(gameData, players) {
    drawDeck(gameData);
    gameData.players = {};

    for (let [playerID, player] of players) {
        gameData.players[playerID] = { hand: [], tricks: [], needsToTrick: false, gamePoints: 0, totalPoints: 0 };
        dealFromDeckToHand(gameData.players[playerID].hand, gameData.deck, 7);
    }
    gameData.stack = [];
    dealFromDeckToHand(gameData.stack, gameData.deck, 1);
    randomShuffle(gameData.deck);
}
