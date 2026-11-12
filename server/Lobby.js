export { createLobbyID, createLobby, joinLobby, leaveLobby, deleteLobby, shouldStartGame, mapToArrayObj, isUsernameValid, lobbyController };
import { sendErrorMessage } from "./lib/InfoMessage.js";
import { Rooms, PlayerRooms } from "./index.js";
//* =================================================== Event controller =============================================================== *\\

function lobbyController(socketData, io, socket) {
    const type = socketData.eventType;
    switch (type) {
        case "joinLobby":
            joinLobbyController(socketData, socket, io);
            break;
        case "createLobby":
            if (!isUsernameValid(socketData.username)) return sendErrorMessage(socket.id, io, "Invalided user name", "Lobby")
            const createLobbyObj = createLobby(socket, socketData.username, Rooms, PlayerRooms);
            socket.emit("conToLobby", createLobbyObj);
            break;
    }
}

function joinLobbyController(joined, socket, io) {
    const roomID = `/${joined.lobbyId}`;
    const roomData = Rooms.get(roomID);
    if (roomData === undefined) return sendErrorMessage(socket.id, io, "Lobby code does not exit", "Lobby");
    if (roomData && roomData.gameStarted) return sendErrorMessage(socket.id, io, "The lobby has start their game", "Lobby");
    if (!isUsernameValid(joined.username)) return sendErrorMessage(socket.id, io, "Invalided user name", "Lobby");

    const playersArr = joinLobby(joined, roomID, socket, Rooms, PlayerRooms);
    socket.to(roomID).emit("playerHandler", playersArr);

    //Adds the current settings to the Object for the joining player
    const joinedreturnData = {
        id: joined.lobbyId,
        players: playersArr,
    };
    socket.emit("conToLobby", joinedreturnData);
    console.log(joined.username, "/", socket.id, "has joined the lobby with id:", roomID);
}


//* =================================================== host lobby =============================================================== *\\
function createLobby(socket, displayName, Rooms, PlayerRooms) {
    const id = createLobbyID(Rooms);
    const roomID = `/${id}`;
    socket.join(roomID);

    //Sets the socket's id to be assigned to the room id when events are called
    PlayerRooms.set(socket.id, roomID);

    let roomObj = roomStateObj(socket, displayName);
    Rooms.set(roomID, roomObj);

    const playerArr = mapToArrayObj(roomObj.players);
    // Sends the default settings and id to the host
    return {
        id: id,
        players: playerArr
    };
}

function createLobbyID(Rooms) {
    let id;
    do {
        id = "";
        for (let i = 0; i < 1; i++) {
            id += Math.floor(Math.random() * 10);
        }
    } while (Rooms.get(`/${id}`));
    return id;
}

function roomStateObj(socket, username) {
    // The lobby state is added to the rooms map as a value to the given room id
    let lobbyStateObj = {
        "players": new Map(),
        "turn": { current: undefined, next: undefined },
        "gameStarted": false,
        //"board": [[null, null, null, null], [null, null, null, null], [null, null, null, null]],
        "startingPlayerID": null
    };
    lobbyStateObj.players.set(socket.id, createPlayer(username, true, socket.id));
    return lobbyStateObj;
}

function deleteLobby(roomID, io) {
    io.to(roomID).emit("leaveLobby");
    //Deletes the key-value pairs from the PlayerRooms map
    const players = Rooms.get(roomID).players;
    for (const [id,] of players.entries()) {
        PlayerRooms.delete(id);
    }
    io.socketsLeave(roomID);
    Rooms.delete(roomID);
}

function shouldStartGame(roomID, Rooms) {
    const players = Rooms.get(roomID).players;
    if (players.size < 2) {
        return false;
    }
    return true;
}

//* ============================================= joined lobby ============================================================== *\\

function joinLobby(playerObj, roomID, socket, Rooms, PlayerRooms) {
    socket.join(roomID);

    const Players = Rooms.get(roomID).players;
    const player = createPlayer(playerObj.name, false, socket.id);

    PlayerRooms.set(socket.id, roomID);
    Players.set(socket.id, player);

    return mapToArrayObj(Players);
}

function leaveLobby(socket, roomID, PlayerRooms, Rooms) {
    //Delete the player from the PlayerRoom map
    PlayerRooms.delete(socket.id);

    //Delete the player from the Rooms map
    const roomData = Rooms.get(roomID);
    roomData.players.delete(socket.id);
    socket.leave(roomID);

    const playersleftArr = mapToArrayObj(roomData.players);
    return playersleftArr;
}

//* ====================================================== All users ========================================================= *\\

function createPlayer(username, flag, socketid) {
    return {
        "id": socketid,
        "name": username,
        "host": flag,
        "hand": []
    };
}

function mapToArrayObj(map) {
    let array = [];
    for (const [key, value] of map.entries()) {
        array.push({
            name: value.username,
            host: value.host,
            playerid: key,
        });
    }
    return array;
}

function isUsernameValid(username) {
    if (username.length < 2) {
        return false;
    }
    return true;
}
