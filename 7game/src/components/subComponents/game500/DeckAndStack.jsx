import { useState, useEffect } from "react";
import MakeCard from "../cardMakeing/MakeCard";


export default function DeckAndStack({ stackTop, stackSize, deckSize }) {

    function convertCard(card) {
        let rank, suit;
        switch (card % 13 + 1) {
            case 1: rank = "a"; break;
            case 11: rank = "j"; break;
            case 12: rank = "q"; break;
            case 13: rank = "k"; break;
            default: rank = (card % 13 + 1).toString();
        }

        switch (Math.floor(card / 13)) {
            case 0: suit = "diams"; break;
            case 1: suit = "spades"; break;
            case 2: suit = "hearts"; break;
            case 3: suit = "clubs"; break;
        }
        return { rank, suit };
    }

    const stackCard = convertCard(stackTop);

    return (
        <>
            <div className="row mt-4 align-items-start">
                {/* Stack Side: Stack size on top + card below */}
                <div className="col-6 d-flex flex-column align-items-center">
                    <p className="fw-bold mb-3">Size: {stackSize}</p>
                    <MakeCard suit={stackCard.suit} rank={stackCard.rank} />
                </div>

                {/* Deck Side: Size on top, then deck cards side by side */}
                <div className="col-6 d-flex flex-column align-items-center">
                    <p className="fw-bold mb-3">Size: {deckSize}</p>
                    <div className="playingCards">
                        <ul className="deck d-flex gap-2 justify-content-center flex-wrap">
                            {Array.from({ length: deckSize }).map((_, index) => (
                                <li key={index}>
                                    <div className="card back">*</div>
                                </li>
                            ))}
                        </ul>
                    </div>
                </div>
            </div>
        </>
    );
}
