import { useState, useEffect } from "react";
import { socket } from "./../socket";
import LobbyID from "./subComponents/LobbyID";
import ListPlayersLobby from "./subComponents/ListPlayersLobby";


function Lobby({ lobbyState }) {
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
            <h1>he</h1>
        </div>
    );
}

export default Lobby;
