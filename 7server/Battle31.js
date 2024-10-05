import { PlayerRooms, Rooms } from "./index.js";
import {
    nextPlayer
} from "./Battle7.js";
import { Server } from "socket.io";
export { dealCards31, mapPlayerInfo31, cal31Move };


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
        //console.log(roomData.players.get(roomData.turn.current))
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

function cal31Move(roomData, socketData, socketID, io, roomID) {
    if (roomData.turn.current != socketID) {
        io.to(socketID).emit("outOfTurn")
        return
    }
    let playerData = roomData.players.get(socketID);

   // playerData.hand = [0,13,26] // TODO DELETE
    console.table(playerData.hand)
    let playersInfo;
    let startedGameData;
    switch (socketData.moveType) {
        case "draw":
            // Add card to stack
            roomData.stack.push(playerData.hand.splice(socketData.card, 1))
            // Draw new card
            let randomNum = Math.floor(Math.random() * roomData.cardDeck.length);
            playerData.hand.push(roomData.cardDeck[randomNum])
            roomData.cardDeck.splice(randomNum, 1);
            io.to(socketID).emit("playable", roomData.players.get(roomData.turn.current).hand)

            roomData.turn.current = roomData.turn.next
            roomData.turn.next = nextPlayer(roomData);

            playersInfo = mapPlayerInfo31(roomData.players);
            startedGameData = {
                playersInfo,
                turn: roomData.turn,
                stack: roomData.stack[roomData.stack.length - 1]
            };
            io.to(roomID).emit("startedGame31", startedGameData);
            break;
        case "swap":
            // Take card from stack and set the new one in
            console.table(roomData.stack)
            console.table(playerData.hand)

            /*let card = roomData.stack[roomData.stack.length-1]
            console.log(card)
            roomData.stack.pop()
            playerData.hand.push((card instanceof Array)?card[0]:card)

            console.table(roomData.stack)
            console.table(playerData.hand)*/
            /*let card = roomData.stack.pop()
            console.log((card instanceof Array)?card[0]:card)
            playerData.hand.push((card instanceof Array)?card[0]:card)
            console.log(card)
            console.table(roomData.stack)
            console.table(playerData.hand)*/
            
            let topStackCard = roomData.stack.pop()
            let swapCardHand = playerData.hand[socketData.card]
            roomData.stack.push(swapCardHand)
            playerData.hand[socketData.card] = topStackCard
            
            //playerData.hand.push(roomData.stack.pop())
            //roomData.stack.push(playerData.hand.splice(socketData.card, 1)[0])


            io.to(socketID).emit("playable", playerData.hand/*roomData.players.get(roomData.turn.current).hand*/)

            roomData.turn.current = roomData.turn.next
            roomData.turn.next = nextPlayer(roomData);

            playersInfo = mapPlayerInfo31(roomData.players);
            startedGameData = {
                playersInfo,
                turn: roomData.turn,
                stack: roomData.stack[roomData.stack.length - 1]
            };
            io.to(roomID).emit("startedGame31", startedGameData);
            console.table(playerData.hand)
            break;
    }

}