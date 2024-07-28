import { useState } from "react";
import { socket } from "../../../socket";
import MakeCard from "./MakeCard";

export default function MakeBoard({ board }) {
    
    return (
        <>
        <div className="row">
            <MakeCardRow cardRow={[...board[0]]}/>
        </div>
        <div className="row">
            <MakeCardRow cardRow={[...board[1]]}/>
        </div>
        <div className="row">
            <MakeCardRow cardRow={[...board[2]]}/>
        </div>
        </>
    );
}

function MakeCardRow({ cardRow }){
    return(
        <>
        <div className="col-3">
            <MakeCard rank={cardRow[0]} suit={"diams"}/>
        </div>
        <div className="col-3">
            <MakeCard rank={cardRow[1]} suit={"spades"}/>
        </div>
        <div className="col-3">
            <MakeCard rank={cardRow[2]} suit={"hearts"}/>
        </div>
        <div className="col-3">
            <MakeCard rank={cardRow[3]} suit={"clubs"}/>
        </div>
        </>
    )

}