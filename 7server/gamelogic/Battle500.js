export { start500Game, call500Move };
import { drawDeck, dealFromDeckToHand, randomShuffle } from "../lib/CardDeckFunctions.js";
import { nextPlayer } from "./Battle7.js";


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
    console.log("Started game 500 made by", socketID);
}


function call500Move(roomData, socketData, playerID, io, roomID) {
    let moveType = socketData.moveType;
    switch (moveType) {
        case "draw":
            draw500Move(roomData, socketData, playerID, io, roomID, socketData.drawKind);
            break;
        case "endTurn":
            break;
        case "playTrick":
            break;
    }
}

function draw500Move(roomData, socketData, playerID, io, roomID, drawKind) {
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

function endTurnMove(roomData, socketData, socketID, io, roomID) {


}

function playTrickMove(roomData, socketData, socketID, io, roomID) {


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
        gameData.players[playerID] = { hand: [], tricks: [], needsToTrick: false };
        dealFromDeckToHand(gameData.players[playerID].hand, gameData.deck, 7);
    }
    gameData.stack = [];
    dealFromDeckToHand(gameData.stack, gameData.deck, 1);
    randomShuffle(gameData.deck);
    console.log(gameData)
}
