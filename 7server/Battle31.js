import { PlayerRooms, Rooms } from "./index.js";
import { nextPlayer } from "./Battle7.js";
import { Server } from "socket.io";
export { dealCards31, mapPlayerInfo31, cal31Move, start31Game };
import { deleteLobby } from "./Lobby.js";


function dealCards31(roomData) {
    let cardDeck = []
    for (let i = 0; i < 52; i++) {
        cardDeck.push(i)
    }

    let randomNum = Math.floor(Math.random() * cardDeck.length);
    roomData.stack = []
    roomData.stack[0] = cardDeck[randomNum];
    cardDeck.splice(randomNum, 1);

    while (true) {
        if (roomData.players.get(roomData.turn.current).hand.length == 3) break;
        randomNum = Math.floor(Math.random() * cardDeck.length);
        roomData.players.get(roomData.turn.current).hand.push(cardDeck[randomNum])
        cardDeck.splice(randomNum, 1);
        roomData.turn.current = roomData.turn.next;
        roomData.turn.next = nextPlayer(roomData);
    }
    roomData.cardDeck = cardDeck;
}

function mapPlayerInfo31(map) {
    let array = [];
    for (const [key, value] of map.entries()) {
        array.push({
            name: value.name,
            id: key
        });
    }
    return array;
}


function outOfTurn(roomData, socketID, io) {
    if (roomData.turn.current != socketID) {
        io.to(socketID).emit("outOfTurn")
        return true;
    }
}

function cal31Move(roomData, socketData, socketID, io, roomID) {
    let playerData = roomData.players.get(socketID);
    let playersInfo;
    let startedGameData;
    let randomNum;
    switch (socketData.moveType) {
        case "NewGame":
            if (socketData.data) {
                // Sets the player hands down to 0 cards
                for (let [playerid, player] of roomData.players.entries()) {
                  player.hand = [];
                }
                start31Game(roomData, socketID, io, roomID)
                io.to(roomID).emit("New31Game");
            } else {
                deleteLobby(roomID, io);
            }
            return;
        case "ReadyForHand":
            io.to(socketID).emit("hand31", playerData.hand);
            return;
        case "showTopCard":
            if (outOfTurn(roomData, socketID, io)) return
            // Draw new card
            randomNum = Math.floor(Math.random() * roomData.cardDeck.length);
            playerData.hand.push(roomData.cardDeck[randomNum]);
            roomData.cardDeck.splice(randomNum, 1);
            io.to(socketID).emit("MustDrawFromHand", roomData.players.get(roomData.turn.current).hand);
            break;
        case "cardPicked":
            roomData.stack.push(playerData.hand[socketData.card]);
            playerData.hand.splice(socketData.card, 1);
            io.to(socketID).emit("hand31", roomData.players.get(roomData.turn.current).hand);

            roomData.turn.current = roomData.turn.next;
            roomData.turn.next = nextPlayer(roomData);

            playersInfo = mapPlayerInfo31(roomData.players);
            startedGameData = {
                playersInfo,
                turn: roomData.turn,
                stack: roomData.stack[roomData.stack.length - 1],
                endPlayer: roomData.endPlayer
            };
            io.to(roomID).emit("startedGame31", startedGameData);
            break;
        case "swap":
            if (outOfTurn(roomData, socketID, io)) return
            // Take card from stack and set the new one in
            let topStackCard = roomData.stack.pop()
            let swapCardHand = playerData.hand[socketData.card]
            roomData.stack.push(swapCardHand)
            playerData.hand[socketData.card] = topStackCard
            io.to(socketID).emit("hand31", playerData.hand)

            roomData.turn.current = roomData.turn.next
            roomData.turn.next = nextPlayer(roomData);

            playersInfo = mapPlayerInfo31(roomData.players);
            startedGameData = {
                playersInfo,
                turn: roomData.turn,
                stack: roomData.stack[roomData.stack.length - 1],
                endPlayer: roomData.endPlayer
            };
            io.to(roomID).emit("startedGame31", startedGameData);
            break;
        case "Knock":
            if (outOfTurn(roomData, socketID, io)) return
            // Sets the endPlayer to be the player that have knocked
            if (roomData.endPlayer != null) {
                io.to(socketID).emit("elseKnocked")
                return
            }
            roomData.endPlayer = socketID;

            roomData.turn.current = roomData.turn.next
            roomData.turn.next = nextPlayer(roomData);

            playersInfo = mapPlayerInfo31(roomData.players);
            startedGameData = {
                playersInfo,
                turn: roomData.turn,
                stack: roomData.stack[roomData.stack.length - 1],
                endPlayer: roomData.endPlayer
            };
            io.to(roomID).emit("startedGame31", startedGameData);
            break;
    }
    if (calPointForOne(playerData.hand) == 31) {
        return sendWinner(roomData, roomID, io)
    }
    if (roomData.endPlayer == roomData.turn.current) {
        return sendWinner(roomData, roomID, io)
    }
}

function sendWinner(roomData, roomID, io) {
    let winnerData = calculatePoints(roomData, roomID, io)
    winnerData = winnerData.sort((a, b) => b.point - a.point);
    io.to(roomID).emit("GameOver31", winnerData);
    console.log(winnerData)
    //deleteLobby(roomID, io);
}

function calculatePoints(roomData, roomID, io) {
    let pointsArray = calPointForAll(roomData);
    return pointsArray;
}
function calPointForAll(roomData) {
    let pointsArray = [];
    roomData.players.forEach((key, value) => {
        pointsArray.push({ id: value, name: key.name, point: calPointForOne(key.hand) })
    })
    return pointsArray
}
function calPointForOne(hand) {
    let suitPoints = [0, 0, 0, 0];
    hand.forEach((value, index) => {
        let rank = value % 13 + 1;
        let suit = (value - (value % 13)) / 13
        if (rank == 1) {
            suitPoints[suit] += 11
        } else if ([10, 11, 12, 13].includes(rank)) {
            suitPoints[suit] += 10
        } else {
            suitPoints[suit] += rank
        }
    })
    return Math.max(...suitPoints);
}


function start31Game(roomData, socketID, io, roomID) {
    roomData.turn.current = socketID;
    roomData.turn.next = nextPlayer(roomData);
    roomData.gameStarted = true;
    dealCards31(roomData);

    roomData.endPlayer = null;

    let playersInfo = mapPlayerInfo31(roomData.players);
    let startedGameData = {
        playersInfo,
        turn: roomData.turn,
        stack: roomData.stack[0],
        endPlayer: roomData.endPlayer
    };


    io.to(roomID).emit("startedGame31", startedGameData);
    console.log("Started game 31 made by", socketID);
}