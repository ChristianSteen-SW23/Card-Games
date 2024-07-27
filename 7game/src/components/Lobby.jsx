import { useState, useEffect } from "react";
import { socket } from "./../socket";
import LobbyID from "./subComponents/lobby/LobbyID";
import ListPlayersLobby from "./subComponents/lobby/ListPlayersLobby";
import StartGameBTN from "./subComponents/lobby/StartGameBTN";


export default function Lobby({ lobbyState }) {
    // All the variables that changes throughout the lobby lifetime
    const [players, setPlayers] = useState(lobbyState.players);

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
            {player.host ? <StartGameBTN players={players}/> : <></> }
        </div>
    );
}
