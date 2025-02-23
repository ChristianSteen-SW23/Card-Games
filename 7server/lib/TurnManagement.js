export { nextPlayer, swapOneTurn }


function swapOneTurn(roomData) {
    roomData.turn.current = roomData.turn.next;
    roomData.turn.next = nextPlayer(roomData);
}

function nextPlayer(roomData) {
    let playersLeft = mapPlayerInfo(roomData.players);
    let currentIndex = playersLeft.findIndex(player => roomData.turn.current == player.id);
    if (currentIndex === -1) {
        console.error("Error: Current player not found in player list.");
        return null;
    }
    return playersLeft[(currentIndex + 1) % playersLeft.length].id;
}

function mapPlayerInfo(map) {
    let array = [];
    for (const [key, value] of map.entries()) {
        array.push({
            name: value.name,
            id: key
        });
    }
    return array;
}