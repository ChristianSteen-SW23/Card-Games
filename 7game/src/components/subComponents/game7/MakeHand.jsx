import { useState } from "react";
import { socket } from "../../../socket";
import MakeCardWithSuit from "./MakeCardWithSuit";
import MakeCard from "./MakeCard";

export default function MakeHand({ hand, setHand }) {
    return (
        <>
            <div className="row p-5"></div>
            <h1 className="text-primary"><b>Your hand:</b></h1>
            <div className="playingCards">
                <ul className="table">
                    {hand.map((card, index) =>
                        <CalNeedCard card={card} />
                    )}
                    <li>
                        <div className="playingCards fourColours" onClick={() => socket.emit("skipTurn")}>
                            <div className="card little joker">
                                <span className="rank">&nbsp;&nbsp;&nbsp;Skip </span>
                                <span class="suit">Joker</span>
                            </div>
                        </div>
                    </li>
                </ul>
            </div>
        </>
    );
}

function CalNeedCard({ card }) {
    let rank = 0;
    switch (card % 13 + 1) {
        case 1:
            rank = "a"
            break;
        case 2:
            rank = "2"
            break;
        case 3:
            rank = "3"
            break;
        case 4:
            rank = "4"
            break;
        case 5:
            rank = "5"
            break;
        case 6:
            rank = "6"
            break;
        case 7:
            rank = "7"
            break;
        case 8:
            rank = "8"
            break;
        case 9:
            rank = "9"
            break;
        case 10:
            rank = "10"
            break;
        case 11:
            rank = "j"
            break;
        case 12:
            rank = "q"
            break;
        case 13:
            rank = "k"
            break;
    }
    let suit
    switch ((card - (card % 13)) / 13 + 1) {
        case 1:
            suit = "diams"
            break;
        case 2:
            suit = "spades"
            break;
        case 3:
            suit = "hearts"
            break;
        case 4:
            suit = "clubs"
            break;
    }

    function playCard() {
        socket.emit("playCard", card)
    }
    return (
        <li>
            <div onClick={playCard}><MakeCard suit={suit} rank={rank} /></div>

        </li>
    )
}