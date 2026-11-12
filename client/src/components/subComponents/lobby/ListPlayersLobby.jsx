import { socket } from "../../../socket";

export default function ListPlayersLobby({ players }) {
    return (
        <>
            <h2 className="text-start">Players</h2>
            <ul className="list-group">
                {players.map((player, index) =>
                    <li key={index} className="list-group-item d-flex justify-content-between align-items-center">
                        {player.name}
                        <span>
                            {player.playerid === socket.id && <span className="badge text-bg-primary rounded-pill me-1">You</span>}
                            {player.host && <span className="badge text-bg-success rounded-pill">Host</span>}
                            {player.isBot && <span className="badge text-bg-warning rounded-pill">Bot 🤖</span>}
                        </span>
                    </li>
                )}
            </ul>
        </>
    );
}