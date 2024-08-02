import { useState, useEffect } from "react";
import { socket } from "./../socket";
import InGamePlayerList31 from "./subComponents/game31/InGamePlayerList31";
import MakeBoard31 from "./subComponents/game31/MakeBoard31"
import MakeHand31 from "./subComponents/game31/MakeHand31";


export default function GamePage31({ lobbyState, hand, setHand }) {
    return (
        <>
            <div className="container text-center">
                <div className="row p-3"></div>
                <div className="row">
                    <div className="col-4">
                        <InGamePlayerList31 lobbyState={lobbyState} />
                    </div>
                    <div className="col-8">
                        <MakeBoard31 stack={lobbyState.stack} />
                    </div>
                </div>
                <div className="row">
                    <MakeHand31 hand={hand} setHand={setHand} />
                </div>
            </div>
        </>
    );
}
