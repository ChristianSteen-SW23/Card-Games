import { useState } from "react";
import { socket } from "../../../socket";

export default function MakeCard({ suit, rank }) {
    return (
        <>
            <div class="playingCards">
                <div className={"rank-" + rank + " " + suit + " card"}>
                    <span className="rank">{rank}</span>
                    <span className="suit">&nbsp;</span>
                </div>
            </div>
        </>
    );
}
