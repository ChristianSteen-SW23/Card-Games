import { useState } from "react";
import { socket } from "../../../socket";

export default function InGamePlayerList31({ lobbyState }) {
    return (
        <ul className="list-group">
            <li className="list-group-item fs-2 justify-content-between align-items-center active">
                Player info:
            </li>
            {lobbyState.playersInfo.map((player, index) =>
                <li className="list-group-item d-flex justify-content-between align-items-center">
                    {player.name}
                    {player.id == lobbyState.turn.current ? <span className="badge text-bg-success rounded-pill">Current</span> : ""}
                    {player.id == lobbyState.turn.next ? <span className="badge text-bg-success rounded-pill">Next</span> : ""}
                    {player.id == lobbyState.endPlayer ? <span className="badge text-bg-danger rounded-pill text-white">{"KNOCKED"}</span>:""}
                </li>
            )}
        </ul>
    );
}
