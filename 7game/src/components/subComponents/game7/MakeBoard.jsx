import { useState } from "react";
import { socket } from "../../../socket";
import MakeCard from "./MakeCard";

export default function MakeBoard({ board }) {

    return (
        <>
            <div className="row">
                <MakeCardRow cardRow={[...board[0]]} />
            </div>
            <div className="row">
                <MakeCardRow cardRow={[...board[1]]} />
            </div>
            <div className="row">
                <MakeCardRow cardRow={[...board[2]]} />
            </div>
        </>
    );
}

const cardRanks = {
    1: "a",
    2: "2",
    3: "3",
    4: "4",
    5: "5",
    6: "6",
    7: "7",
    8: "8",
    9: "9",
    10: "10",
    11: "j",
    12: "q",
    13: "k"
};
function getCardRank(card) {
    let rank = card % 14;
    return cardRanks[rank];
}
function getBackSide(card) {
    let rank = card % 14;

    return cardRanks[rank] == "a" || cardRanks[rank] == "k" ? true : false;
}
function MakeCardRow({ cardRow }) {
    return (
        <>
            <div className="col-3">
                <MakeCard rank={getCardRank(cardRow[0])} suit={"diams"} backSide={getBackSide(cardRow[0])} />
            </div>
            <div className="col-3">
                <MakeCard rank={getCardRank(cardRow[1])} suit={"spades"} backSide={getBackSide(cardRow[1])} />
            </div>
            <div className="col-3">
                <MakeCard rank={getCardRank(cardRow[2])} suit={"hearts"} backSide={getBackSide(cardRow[2])} />
            </div>
            <div className="col-3">
                <MakeCard rank={getCardRank(cardRow[3])} suit={"clubs"} backSide={getBackSide(cardRow[3])} />
            </div>
        </>
    )

}