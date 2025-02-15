import { useState, useEffect } from "react";
import { socket } from "./../socket";
import LobbyID from "./subComponents/lobby/LobbyID";
import ListPlayersLobby from "./subComponents/lobby/ListPlayersLobby";
import StartGameBTN from "./subComponents/lobby/StartGameBTN";
import GameModeDropDown from "./subComponents/lobby/GameModeDropDown";


export default function Lobby({ lobbyState }) {
    // All the variables that changes throughout the lobby lifetime
    const [players, setPlayers] = useState(lobbyState.players);
    const [gameMode, setGameMode] = useState("7");
    const availableGames = ["7", "31"];

    const player = players.find(player => player.playerid === socket.id);
    const isHost = player.host;
    const roomID = lobbyState.id;

    useEffect(() => {
        socket.on("playerHandler", (players) => {
            setPlayers(players);
        });
        return () => {
            socket.off("playerHandler");
        };
    }, []);

    return (
        <div className="container text-center">
            <LobbyID roomID={roomID}/>
            <ListPlayersLobby players={players}/>

            {player.host ? <GameModeDropDown gameMode={gameMode} setGameMode={setGameMode} availableGames={availableGames} /> : <></>}
            {player.host ? <StartGameBTN players={players} gameMode={gameMode} /> : <></>}
        </div>
    );
}
