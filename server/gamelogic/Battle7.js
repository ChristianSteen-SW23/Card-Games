export { start7Game, call7Move };
import { sendErrorMessage } from "../lib/InfoMessage.js";
import { nextPlayer } from "../lib/TurnManagement.js";
import { countPointScore } from "../lib/PointCounter.js";

function start7Game(roomData, socketID, io, roomID) {
    roomData.gameStarted = true;
    roomData.turn.current = socketID;

    const startedGameData = startGameHelper(roomData, true);

    for (let [playerid, player] of roomData.players.entries()) {
        io.to(playerid).emit("startedGame7", { ...startedGameData, handInfo: player.hand });
    }
    console.log("Started game 7 made by", socketID);
}

function startGameAgain(roomData, io, socketID) {
    const startedGameData = startGameHelper(roomData, false);
    for (let [playerid, player] of roomData.players.entries()) {
        io.to(playerid).emit("gameInfo", startedGameData);
        io.to(playerid).emit("handInfo", player.hand);
    }
}

function startGameHelper(roomData, firstGame) {
    roomData.box = null;
    roomData.board = [[null, null, null, null], [null, null, null, null], [null, null, null, null]];
    roomData.turn.next = nextPlayer(roomData);
    dealCards(roomData)
    roomData.turn.current = roomData.startingPlayerID
    roomData.turn.next = nextPlayer(roomData);

    for (let [playerid, player] of roomData.players.entries()) {
        player.cardsLeft = player.hand.length;
        if (firstGame) player.totalScore = 0;
    }

    return {
        playersInfo: mapPlayerInfo7(roomData.players),
        turn: roomData.turn,
        board: roomData.board
    };
}

function call7Move(roomData, socketData, playerID, io, roomID) {
    let moveType = socketData.moveType;
    if (playerID != roomData.turn.current && moveType != "playAgain") return sendErrorMessage(playerID, io, "It is not your turn", "Out of turn");
    switch (moveType) {
        case "playAgain":
            startGameAgain(roomData, io, playerID);
            break;
        case "skipTurn":
            if (!possibleSkip(roomData, playerID)) return sendErrorMessage(playerID, io, "You can play a card. Please stop the cheating", "Card playable");

            roomData.box = roomData.turn.current;
            roomData.turn.current = roomData.turn.next;
            roomData.turn.next = nextPlayer(roomData);

            sendGameInfo(roomData, roomID, io);
            break;
        case "playCard":
            const card = socketData.card;
            if (!cardPlayable(card, roomData)) return sendErrorMessage(playerID, io, "You can not play this card", "Card not playable");
            roomData.players.get(roomData.turn.current).cardsLeft--;
            playCard(card, roomData)

            // Remove card from hand and sent it out
            let indexToRemove = roomData.players.get(roomData.turn.current).hand.indexOf(card);
            roomData.players.get(roomData.turn.current).hand.splice(indexToRemove, 1);
            io.to(playerID).emit("handInfo", roomData.players.get(roomData.turn.current).hand)

            roomData.turn.current = roomData.turn.next;
            roomData.turn.next = nextPlayer(roomData);

            sendGameInfo(roomData, roomID, io);

            if (roomData.players.get(playerID).hand.length === 0) return sendGameOver(roomID, updateScore(roomData), io);;
            break;
    }
}

function sendGameInfo(roomData, roomID, io) {
    const gameInfo = {
        playersInfo: mapPlayerInfo7(roomData.players),
        turn: roomData.turn,
        board: roomData.board
    };
    io.to(roomID).emit("gameInfo", gameInfo);
}

function updateScore(roomData) {
    let scoreData = [];

    for (const [key, value] of roomData.players.entries()) {
        let score = countPointScore(value.hand, 10, 15, 5) + ((roomData.box === key) ? 25 : 0);
        value.totalScore += score;

        scoreData.push({
            name: value.name,
            totalScore: value.totalScore,
            roundScore: score
        });
    }
    return scoreData;
}

function sendGameOver(roomID, data, io) {
    io.to(roomID).emit("gameEnded", { contineAllowed: false, winData: data });
}

function mapPlayerInfo7(map) {
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

function dealCards(roomData) {
    for (let [_, player] of roomData.players.entries())
        player.hand = [];

    let cardDeck = Array.from({ length: 52 }, (_, i) => i);

    roomData.players.get(roomData.turn.current).hand.push(19)
    roomData.players.get(roomData.turn.current).hand.push(18)
    roomData.players.get(roomData.turn.next).hand.push(0)
    roomData.startingPlayerID = roomData.turn.current
    return;

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