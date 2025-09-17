import { useState, useEffect } from "react";
import { socket } from "./../socket";
import { showPopup } from "../js/popupController";
import BottomOfScreen from "./subComponents/planningPoker/BottomOfScreen";
import PlayersDisplay from "./subComponents/planningPoker/PlayersDisplay";

export default function PlanningPoker({ lobbyStateStart }) {

    // Correctly declare useState hooks
    const [players, setPlayers] = useState(lobbyStateStart.playersInfo);
    const [mustNewRound, setMustNewRound] = useState(lobbyStateStart.mustNewRound);


    const cardValues = { 0: "½", 1: "1", 2: "2", 3: "3", 4: "5", 5: "8", 6: "13", 7: "20", 8: "40", 9: "100", 10: "∞", 11: "?" };
    const valueRange = Object.keys(cardValues).length;

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
            <PlayersDisplay mustNewRound={mustNewRound} players={players} valueRange={valueRange} cardValues={cardValues} />
            <BottomOfScreen mustNewRound={mustNewRound} valueRange={valueRange} cardValues={cardValues} />
        </>
    );
}



