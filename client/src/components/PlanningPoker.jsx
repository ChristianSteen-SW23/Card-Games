import { useState, useEffect } from "react";
import { socket } from "./../socket";
import { showPopup } from "../js/popupController";
import BottomOfScreen from "./subComponents/planningPoker/BottomOfScreen";

export default function PlanningPoker({ lobbyStateStart }) {

    // Correctly declare useState hooks
    const [players, setPlayers] = useState(lobbyStateStart.playersInfo);
    const [mustNewRound, setMustNewRound] = useState(lobbyStateStart.mustNewRound);

    useEffect(() => {
        function gameInfo(data) {
            setPlayers(data.playersInfo);
            setMustNewRound(data.mustNewRound);
        }

        function errorMessage(data) {
            showPopup(`Error ${data.type}: ${data.message}`, "error");
        }

        socket.on("gameInfo", gameInfo);
        socket.on("errorMessage", errorMessage);

        return () => {
            socket.off("gameInfo");
            socket.off("errorMessage");
        };
    }, []);

    return (
        <>
            <div className="container text-center">
                <p><strong>Lobby:</strong> {JSON.stringify(players)}</p>
                <p><strong>Hand:</strong> {JSON.stringify(mustNewRound)}</p>
            </div>

            <BottomOfScreen />
        </>
    );
}



