export { startPlanningPoker, callPlanningPokerMove };
import { sendErrorMessage } from "../lib/InfoMessage.js";

function startPlanningPoker(roomData, socketID, io, roomID) {
    roomData.gameStarted = true;
    roomData.SM = socketID;
    roomData.mustNewRound = false;

    for (let [playerid, player] of roomData.players.entries()) {
        player.ready = false;
        player.value = -1;
    }


    emitGameInfo(io, roomID, roomData);
    console.log("Started game planning poker made by", socketID);
}

function emitGameInfo(io, roomID, roomData) {
    io.to(roomID).emit("startedGamePlanningPoker", sendGameInfo(roomData));
}

function callPlanningPokerMove(roomData, socketData, playerID, io, roomID) {
    let moveType = socketData.moveType;
    switch (moveType) {
        case "choice":
            choice(roomData, io, playerID, roomID, socketData.value);
            break;
        case "newRound":
            newRound(roomData, playerID, io, roomID);
            sendGameInfo(roomData, roomID, io);
            break;
    }
}

function newRound(roomData, playerID, io, roomID) {
    if (roomData.SM !== playerID) return sendErrorMessage(playerID, io, "Only the Scrum master can start a new game. This is the host.", "Not Scrum master");
    for (let [playerid, player] of roomData.players.entries())
        player.ready = false;
    emitGameInfo(io, roomID, roomData);
}

function choice(roomData, playerID, io, roomID, value) {
    if (roomData.mustNewRound == true) return sendErrorMessage(playerID, io, "You can not play a card. Wait for Scrum master to start new turn", "Out of turn");
    roomData.players.get(playerID).value = value;
    roomData.players.get(playerID).ready = true;
    lastChoice(roomData);
    emitGameInfo(io, roomID, roomData);
}

function lastChoice(roomData) {
    for (let [playerid, player] of roomData.players.entries())
        if (player.ready == false) return false;
    roomData.mustNewRound = true;
    return true;
}

function sendGameInfo(roomData, roomID, io) {
    const gameInfo = {
        playersInfo: mapPlayerInfoPlanningPoker(roomData.players),
        turn: roomData.turn,
        board: roomData.board
    };
    io.to(roomID).emit("gameInfo", gameInfo);
}

function mapPlayerInfoPlanningPoker(map) {
    let array = [];
    for (const [key, value] of map.entries()) {
        array.push({
            name: value.name,
            id: key,
            value: value.value,
            ready: player.ready,
        });
    }
    return array;
}
