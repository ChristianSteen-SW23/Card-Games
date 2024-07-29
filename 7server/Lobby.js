import { Rooms, PlayerRooms } from "./index.js";
export { createLobbyID, createLobby, joinLobby, leaveLobby, deleteLobby, shouldStartGame, mapToArrayObj, isUsernameValid };

//* =================================================== host lobby =============================================================== *\\
function createLobby(socket, displayName) {
    const id = createLobbyID();
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

function createLobbyID() {
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
        "board": [[null,null,null,null],[null,null,null,null],[null,null,null,null]],
        "startingPlayerID": null
    };
    lobbyStateObj.players.set(socket.id, createPlayer(username, true, socket.id));
    return lobbyStateObj;
}

function deleteLobby(roomID, io) {
    //Deletes the key-value pairs from the PlayerRooms map
    const players = Rooms.get(roomID).players;
    for (const [id,] of players.entries()) {
        PlayerRooms.delete(id);
    }

    io.socketsLeave(roomID);
    Rooms.delete(roomID);
}

function shouldStartGame(roomID) {
    const players = Rooms.get(roomID).players;
    if (players.size < 2) {
        return false;
    }
    return true;
}

//* ============================================= joined lobby ============================================================== *\\

function joinLobby(playerObj, roomID, socket) {
    socket.join(roomID);

    const Players = Rooms.get(roomID).players;
    const player = createPlayer(playerObj.name, false, socket.id);

    PlayerRooms.set(socket.id, roomID);
    Players.set(socket.id, player);

    return mapToArrayObj(Players);
}

function leaveLobby(socket, roomID) {
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
            name: value.name,
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

export { Rooms };
