export { nextPlayer }

function nextPlayer(roomData) {
    let playersLeft = mapPlayerInfo(roomData.players).filter(player => player.lives !== 0);
    let currentIndex = playersLeft.findIndex(player => roomData.turn.current === player.id);
    return playersLeft[(currentIndex + 1) % playersLeft.length].id;
}

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