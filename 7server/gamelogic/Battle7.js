export { mapPlayerInfo, nextPlayer, switchRoles, dealCards, playCard, cardPlayable, possibleSkip, start7Game, call7Move };

function start7Game(roomData, socketID, io, roomID) {
    let startedGameData;
    let playersInfo;
    roomData.turn.current = socketID;
    roomData.turn.next = nextPlayer(roomData);
    roomData.gameStarted = true;

    dealCards(roomData)
    roomData.turn.current = roomData.startingPlayerID
    roomData.turn.next = nextPlayer(roomData);

    // Gives players their hand
    for (let [playerid, player] of roomData.players.entries()) {
        player.cardsLeft = player.hand.length;
        io.to(playerid).emit("handInfo", player.hand);
    }

    playersInfo = mapPlayerInfo(roomData.players);
    startedGameData = {
        playersInfo,
        turn: roomData.turn,
        board: roomData.board
    };
    io.to(roomID).emit("startedGame7", startedGameData);
    console.log("Started game 7 made by", socketID);
}

function call7Move(roomData, socketData, playerID, io, roomID) {
    let moveType = socketData.moveType;
    //const roomID = PlayerRooms.get(socket.id)
    //const roomData = Rooms.get(roomID)
    switch (moveType) {
        case "skipTurn":
            if (playerID == roomData.turn.current) {
                if (possibleSkip(roomData, playerID)) {

                    // Set next turn
                    roomData.turn.current = roomData.turn.next;
                    roomData.turn.next = nextPlayer(roomData);

                    // Send out new game info
                    const playersInfo = mapPlayerInfo(roomData.players);
                    const gameInfo = {
                        playersInfo,
                        turn: roomData.turn,
                        board: roomData.board
                    };
                    io.to(roomID).emit("gameInfo", gameInfo);
                } else {
                    io.to(playerID).emit("noSkip")
                }

            } else {
                io.to(playerID).emit("outOfTurn")
            }
            break;
        case "playCard":
            if (playerID == roomData.turn.current) {
                const card = socketData.card;
                if (cardPlayable(card, roomData)) {
                    roomData.players.get(roomData.turn.current).cardsLeft--;
                    playCard(card, roomData)
                    const playersInfo = mapPlayerInfo(roomData.players);

                    // Remove card from hand and sent it out
                    let indexToRemove = roomData.players.get(roomData.turn.current).hand.indexOf(card);
                    roomData.players.get(roomData.turn.current).hand.splice(indexToRemove, 1);
                    io.to(playerID).emit("playable", roomData.players.get(roomData.turn.current).hand)

                    // Set next turn
                    roomData.turn.current = roomData.turn.next;
                    roomData.turn.next = nextPlayer(roomData);

                    // Send out new game info
                    const gameInfo = {
                        playersInfo,
                        turn: roomData.turn,
                        board: roomData.board
                    };
                    io.to(roomID).emit("gameInfo", gameInfo);
                } else {
                    io.to(playerID).emit("notPlayable")
                }
            } else {
                io.to(playerID).emit("outOfTurn")
            }
            break;
    }
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

function dealCards(roomData) {
    let cardDeck = []
    for (let i = 0; i < 52; i++) {
        cardDeck.push(i)
    }

    let randomNum;
    while (cardDeck.length != 0) {
        randomNum = Math.floor(Math.random() * cardDeck.length);
        roomData.players.get(roomData.turn.current).hand.push(cardDeck[randomNum])
        if (cardDeck[randomNum] == 19)
            roomData.startingPlayerID = roomData.turn.current
        cardDeck.splice(randomNum, 1);
        roomData.turn.current = roomData.turn.next;
        roomData.turn.next = nextPlayer(roomData);
    }
}

function cardPlayable(card, roomData) {
    let board = roomData.board
    const suit = (card - (card % 13)) / 13
    const rank = card % 13 + 1
    if (board[1][1] == null && card != 19) return false

    if (rank == 7) return true

    if (board[1][suit] == null) return false

    if (rank == 6 || rank == 8) return true

    if (rank > 7 && board[0][suit] + 1 == rank) return true

    if (rank < 7 && board[2][suit] - 1 == rank) return true

    return false
}

function playCard(card, roomData) {
    const suit = (card - (card % 13)) / 13
    const rank = card % 13 + 1

    if (rank == 7) roomData.board[1][suit] = 7;
    if (rank < 7) roomData.board[2][suit] = rank;
    if (rank > 7) roomData.board[0][suit] = rank;
}

function getRandomInt(min, max) {
    min = Math.ceil(min);
    max = Math.floor(max);
    return Math.floor(Math.random() * (max - min + 1)) + min;
}

function possibleSkip(roomData, socketID) {
    let playerHand = roomData.players.get(socketID).hand;
    let canSkip = true;
    playerHand.forEach((card, index) => {
        if (cardPlayable(card, roomData)) {
            canSkip = false;
        }
    });
    return canSkip;
}