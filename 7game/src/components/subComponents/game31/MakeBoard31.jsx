import { useState } from "react";
import { socket } from "../../../socket";
import MakeCard from "./../game/MakeCard";
import { getCardDetails, getRank, getSuit } from "./../../../js/cardCal"

export default function MakeBoard({ stack }) {

    return (
        <>
            <div className="row">
                <div class="playingCards col-6">
                    <div class="card back">*</div>
                </div>
                <div class="playingCards col-6">
                    <MakeCard rank={getRank(stack)} suit={getSuit(stack)}/>
                </div>
            </div>
        </>
    );
}
