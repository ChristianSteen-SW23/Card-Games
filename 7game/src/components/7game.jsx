import { useState, useEffect, useRef } from "react";
import { socket } from "./../socket";
import InGamePlayerList from "./subComponents/game7/InGamePlayerList";
import MakeBoard from "./subComponents/game7/MakeBoard";
import MakeHand from "./subComponents/game7/MakeHand";
import Popup from "./subComponents/helperComponents/Popup";


export default function GamePage7({ lobbyState, hand, setHand }) {
    const popupRef = useRef();

    useEffect(() => {
        function errorMessage(data) {
            popupRef.current.show(`Error ${data.type}: ${data.message}`, "error");
        }
        socket.on("errorMessage", errorMessage);
        return () => {
            socket.off("errorMessage");
        };
    }, []);

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
                    <MakeHand hand={hand} setHand={setHand} />
                </div>
            </div>
            <Popup ref={popupRef} duration={5000} />
        </>
    );
}
