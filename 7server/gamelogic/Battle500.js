export { start500Game };
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
