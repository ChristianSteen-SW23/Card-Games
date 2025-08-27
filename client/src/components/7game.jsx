import { useState, useEffect, useRef } from "react";
import { socket } from "./../socket";
import InGamePlayerList from "./subComponents/game7/InGamePlayerList";
import MakeBoard from "./subComponents/game7/MakeBoard";
import MakeHand from "./subComponents/game7/MakeHand";
import Popup from "./subComponents/helperComponents/Popup";


export default function GamePage7({ lobbyStateStart }) {
    const popupRef = useRef();
    const [lobbyState, setLobbyState] = useState(lobbyStateStart);

    const [hand, setHand] = useState(lobbyStateStart.handInfo.sort((a, b) => a - b));

    useEffect(() => {
        function errorMessage(data) {
            popupRef.current.show(`Error ${data.type}: ${data.message}`, "error");
        }
        function playableFunc(data) {
            console.log(data)
            let newHand = data;
            newHand.sort((a, b) => a - b)
            setHand(newHand);
        }

        function gameInfoFunc(data) {
            setLobbyState(data)
        }
        socket.on("errorMessage", errorMessage);
        socket.on('playable', playableFunc);
        socket.on('gameInfo', gameInfoFunc);

        return () => {
            socket.off("errorMessage");
            socket.off("playable");
            socket.off("gameInfo");
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
