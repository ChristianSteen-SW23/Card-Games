import { useState } from "react";
import { socket } from "./../../socket";

export default function ListPlayersLobby({ players }) {
    return (
        <>
            <div className="row m-3"></div>
            <div className="row justify-content-md-center p-4">
                <div className="col-4">
                    <ul className="list-group">
                        <li className="list-group-item fs-2 justify-content-between align-items-center active">
                            Players in lobby
                        </li>
                        {players.map((player, index) =>
                            <li className="list-group-item d-flex justify-content-between align-items-center">
                                {player.name}
                                {player.playerid === socket.id ? <span className="badge text-bg-primary rounded-pill">You</span> : ""}
                                {player.host ? <span className="badge text-bg-success rounded-pill">Host</span> : ""}
                            </li>
                        )}
                    </ul>
                </div>
            </div>
        </>
    );
}