import { useState, useEffect } from "react";
import { socket } from "./../socket";
import { showPopup } from "../js/popupController";
import BottomOfScreen from "./subComponents/planningPoker/BottomOfScreen";
import PlayersDisplay from "./subComponents/planningPoker/PlayersDisplay";

export default function PlanningPoker({ lobbyStateStart }) {
    // Correctly declare useState hooks
    const [players, setPlayers] = useState(lobbyStateStart.playersInfo);
    const [gameOver, setGameOver] = useState(lobbyStateStart.gameOver);

    const cardValues = lobbyStateStart.cardSet
    
    const valueRange = Object.keys(cardValues).length;

    useEffect(() => {
        function gameInfo(data) {
            setPlayers(data.playersInfo);
            setGameOver(data.gameOver);
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
                <p><strong>Hand:</strong> {gameOver}</p>
            </div> */}
            <PlayersDisplay gameOver={gameOver} players={players} valueRange={valueRange} cardValues={cardValues} />
            <BottomOfScreen gameOver={gameOver} valueRange={valueRange} cardValues={cardValues} />
        </>
    );
}



