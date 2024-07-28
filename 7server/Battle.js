import { PlayerRooms, Rooms } from "./index.js";
export {  mapPlayerInfo, nextPlayer, switchRoles, dealCards, playCard, cardPlayable };

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

function dealCards(roomData){
    let cardDeck = []
    for (let i = 0; i < 52; i++) {
        cardDeck.push(i)        
    }

    let randomNum;
    while(cardDeck.length != 0){
        randomNum = Math.floor(Math.random() * cardDeck.length);
        roomData.players.get(roomData.turn.current).hand.push(cardDeck[randomNum])
        if(cardDeck[randomNum] == 19)
            roomData.startingPlayerID = roomData.turn.current
        cardDeck.splice(randomNum, 1);
        roomData.turn.current = roomData.turn.next;
        roomData.turn.next = nextPlayer(roomData);
    }
}

function cardPlayable(card, roomData){
    let board = roomData.board
    const suit = (card-(card%13))/13
    const rank = card%13+1
    if(board[1][1] == null && card != 19) return false

    if(rank == 7) return true

    if(board[1][suit] == null) return false

    if(rank == 6 || rank == 8) return true

    if(rank > 7 && board[0][suit]+1 == rank) return true

    if(rank < 7 && board[2][suit]-1 == rank) return true

    return false
}

function playCard(card, roomData){
    const suit = (card-(card%13))/13
    const rank = card%13+1

    if(rank == 7) roomData.board[1][suit] = 7;
    if(rank < 7) roomData.board[2][suit] = rank;
    if(rank > 7) roomData.board[0][suit] = rank;
}

function getRandomInt(min, max) {
    min = Math.ceil(min);
    max = Math.floor(max);
    return Math.floor(Math.random() * (max - min + 1)) + min;
}