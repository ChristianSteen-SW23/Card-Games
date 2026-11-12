import { useState, useEffect } from "react";
import { socket } from "./../socket";
import InGamePlayerList from "./subComponents/game7/InGamePlayerList";
import MakeBoard from "./subComponents/game7/MakeBoard";
import MakeHand from "./subComponents/game7/MakeHand";
import { showPopup } from "../js/popupController";
import WinModal from "./subComponents/helperComponents/WinModal";

export default function GamePage7({ lobbyStateStart }) {
    const [lobbyState, setLobbyState] = useState(lobbyStateStart);
    const [hand, setHand] = useState(lobbyStateStart.handInfo.sort((a, b) => a - b));
    const [winPop, setWinPop] = useState(false);
    const [winData, setWinData] = useState([]);

    useEffect(() => {
        function errorMessage(data) {
            showPopup(`Error ${data.type}: ${data.message}`, "error");
        }
        function handInfoFunc(data) {
            let newHand = data.handInfo;
            newHand.sort((a, b) => a - b)
            setHand(newHand);
        }
        
        function gameInfoFunc(data) {
            console.log(data)
            setWinPop(false);
            setLobbyState(data);
            if (socket.id == data.turn.current)
                showPopup("It is your turn🥳", "success");
        }

        function gameEnded(data) {
            setWinPop(true);
            setWinData(data.winData);
        }
        socket.on("errorMessage", errorMessage);
        socket.on('handInfo', handInfoFunc);
        socket.on('gameInfo', gameInfoFunc);
        socket.on("gameEnded", gameEnded);

        return () => {
            socket.off("errorMessage");
            socket.off("handInfo");
            socket.off("gameInfo");
            socket.off("gameEnded");
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

            <WinModal show={winPop} data={winData.map(player => ({
                name: player.name,
                "Round Score": player.roundScore,
                "Total Score": player.totalScore,
            }))}
                btns={<button type="button" className="btn btn-success" data-bs-dismiss="modal"
                    onClick={() => { socket.emit("7Move", { "moveType": "playAgain" }) }}>Keep Going</button>}
            />
        </>
    );
}
