import { PlayerRooms, Rooms } from "./index.js";
import {
    nextPlayer
  } from "./Battle7.js";
export { dealCards31, mapPlayerInfo31 };


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
        if(roomData.players.get(roomData.turn.current).hand.length == 3) break;
        randomNum = Math.floor(Math.random() * cardDeck.length);
        roomData.players.get(roomData.turn.current).hand.push(cardDeck[randomNum])
        cardDeck.splice(randomNum, 1);
        roomData.turn.current = roomData.turn.next;
        roomData.turn.next = nextPlayer(roomData);
    }
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