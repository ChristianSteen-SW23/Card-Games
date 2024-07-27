import { useState, useEffect } from "react";
import { socket } from "./../socket";
import InGamePlayerList from "./subComponents/game/InGamePlayerList";


export default function GamePage({ lobbyState }) {
    /*useEffect(() => {
        socket.on("playerHandler", (players) => {
            setPlayers(players);
        });
        return () => {
            socket.off("playerHandler");
        };
    }, []);*/

    return (
        <div className="container text-center">
            <div className="row">
                <div className="col-2">
                    <InGamePlayerList lobbyState={lobbyState}/>
                </div>
                <div className="col-10"></div>
            </div>
        </div>
    );
}
