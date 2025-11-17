import { useState, useEffect } from "react";
import { socket } from "./../socket";
import { showPopup } from "../js/popupController";
import BottomOfScreen from "./subComponents/planningPoker/BottomOfScreen";
import PlayersDisplay from "./subComponents/planningPoker/PlayersDisplay";

export default function PlanningPoker({ lobbyStateStart }) {

    // Correctly declare useState hooks
    const [players, setPlayers] = useState(lobbyStateStart.playersInfo);
    const [mustNewRound, setMustNewRound] = useState(lobbyStateStart.mustNewRound);


    // const cardValues = { 0: "0", 1: "½", 2: "1", 3: "2", 4: "3", 5: "5", 6: "8", 7: "13", 8: "20", 9: "40", 10: "100", 11: "∞", 12: "?" };
    const cardValues = {
        0: "0",
        1: "1/4",
        2: "1/2",
        3: "1",
        4: "1.5",
        5: "2",
        6: "3",
        7: "4",
        8: "5",
        9: "7",
        10: "10",
        11: "14",
        12: "20",
        11: "∞",
        12: "?",
        13: "☕"
    };
    
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



