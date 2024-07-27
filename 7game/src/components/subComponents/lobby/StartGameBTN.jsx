import { useState } from "react";
import { socket } from "../../../socket";

export default function StartGameBTN( { players } ) {
    function startGame(){
        socket.emit("startGame")
    }
    return (
        <>
            <button type="button" className="btn btn-primary p-3 m-3 btn-lg" onClick={startGame} disabled={!(players.length > 1)}>
                Start game {(players.length > 1) ? "" : <span className="badge text-bg-secondary">Need more players to start</span>}
            </button>
        </>
    );
}