import { PlayerRooms, Rooms } from "./index.js";
export {  mapPlayerInfo, nextPlayer, switchRoles };

function mapPlayerInfo(map) {
    let array = [];
    for (const [key, value] of map.entries()) {
        array.push({
            name: value.name,
            id: key,
            cardsLeft: value.cardsLeft
        });
    }
    return array;
}

function nextPlayer(roomData) {
    let playersLeft = mapPlayerInfo(roomData.players).filter(player => player.lives !== 0);
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

