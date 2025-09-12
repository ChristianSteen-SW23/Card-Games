import { useState, useEffect } from "react";
import { socket } from "./../socket";
import { showPopup } from "../js/popupController";
import BottomOfScreen from "./subComponents/planningPoker/BottomOfScreen";
import PlayersDisplay from "./subComponents/planningPoker/PlayersDisplay";

export default function PlanningPoker({ lobbyStateStart }) {

    // Correctly declare useState hooks
    const [players, setPlayers] = useState(lobbyStateStart.playersInfo);
    const [mustNewRound, setMustNewRound] = useState(lobbyStateStart.mustNewRound);

    const valueRange = 10;

    useEffect(() => {
        function gameInfo(data) {
            console.log(data);
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
            {/* <div className="container text-center">
                <p><strong>Lobby:</strong> {JSON.stringify(players)}</p>
                <p><strong>Hand:</strong> {mustNewRound}</p>
            </div> */}
            <PlayersDisplay mustNewRound={mustNewRound} players={players} valueRange={valueRange} />
            <BottomOfScreen mustNewRound={mustNewRound} valueRange={valueRange} />
        </>
    );
}



