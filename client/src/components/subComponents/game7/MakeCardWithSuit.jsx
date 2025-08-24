import { useState } from "react";
import { socket } from "../../../socket";

export default function MakeCardWithSuit({ suit, rank }) {
    return (
        <>
            <div className="playingCards">
                <div className={"rank-" + rank + " " + suit + " card"}>
                    <span className="rank">{rank}</span>
                    {suit == "diams"? <span className="suit">&diams;</span>:
                    suit == "spades"? <span className="suit">&spades;</span>:
                    suit == "hearts"? <span className="suit">&hearts;</span>:
                    <span className="suit">&clubs;</span>
                    }
                    
                </div>
            </div>
        </>
    );
}
