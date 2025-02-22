export { start500Game, cal500Move };
import { drawDeck, dealFromDeckToHand } from "../lib/CardDeckFunctions.js";
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
        io.to(playerID).emit("startedGame500", personnalData); // ✅ Send data with the event
    }

    // io.to(roomID).emit("startedGame500", startedGameData);
    console.log("Started game 500 made by", socketID);
}


function cal500Move(roomData, socketData, socketID, io, roomID) {
    let moveType = socketData.moveType;

    switch (moveType) {
        case "draw":
            draw500Move(roomData, socketData, socketID, io, roomID, moveType.drawKind);
            break;
        case "endTurn":
            break;
        case "playTrick":
            break;
    }
}

function draw500Move(roomData, socketData, socketID, io, roomID, drawKind) {
    switch (drawKind) {
        case "stacktop":
            drawStacktop();
            break;
        case "stack":
            drawStack();
            break;
        case "decktop":
            drawDecktop();
            break;
    }
}

function drawStacktop() {

}

function drawStack() {

}

function drawDecktop() {

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
        gameData.players[playerID] = { hand: [], tricks: [] };
        dealFromDeckToHand(gameData.players[playerID].hand, gameData.deck, 7);
    }
    gameData.stack = [];
    dealFromDeckToHand(gameData.stack, gameData.deck, 1);
    console.log(gameData)
}
