import { useState, useEffect } from "react";
import { socket } from "./../socket";
import InGamePlayerList from "./subComponents/game/InGamePlayerList";
import MakeBoard from "./subComponents/game/MakeBoard";
import MakeHand from "./subComponents/game/MakeHand";


export default function GamePage7({ lobbyState, hand, setHand }) {
    /*useEffect(() => {
        socket.on("gameInfo", (players) => {
            setPlayers(players);
        });
        return () => {
            socket.off("gameInfo");
        };
    }, []);*/

    return (
        <>
            <div className="container text-center">
                <div className="row p-3"></div>
                <div className="row">
                    <div className="col-4">
                        <InGamePlayerList lobbyState={lobbyState} />
                    </div>
                    <div className="col-8">
                        <MakeBoard board={lobbyState.board} />
                    </div>
                </div>
                <div className="row">
                    <MakeHand hand={hand} setHand={setHand}/>
                </div>
            </div>
        </>
    );
}
