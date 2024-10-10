import { useState } from "react";
import { socket } from "../../../socket";
import MakeCard from "./../game/MakeCard";
import { getCardDetails, getRank, getSuit } from "./../../../js/cardCal"

export default function MakeBoard({ stack, pickedCard, mustPickCard }) {
    function drawCard() {
        if(mustPickCard) return alert("You must play a card, draw");
        /*if (pickedCard == -1) return alert("pick a card before playing")
        let data = {}
        data.card = pickedCard
        data.moveType = "draw"

        socket.emit("31Move", data);*/
        let data = {}
        data.moveType = "showTopCard"
        socket.emit("31Move", data);
    }
    function swapCard() {
        if(mustPickCard) return alert("You must play a card, not swap");
        if (pickedCard == -1) return alert("pick a card before playing")
        let data = {}
        data.card = pickedCard
        data.moveType = "swap"

        socket.emit("31Move", data);
    }

    return (
        <>
            <div className="row">
                <div className="playingCards col-6" onClick={drawCard}>
                    <div className="card back">*</div>
                </div>
                <div className="playingCards col-6" onClick={swapCard}>
                    <MakeCard rank={getRank(stack)} suit={getSuit(stack)} />
                </div>
            </div>
        </>
    );
}
