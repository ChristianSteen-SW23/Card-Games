import { useState } from "react";
import { GAME_MODES } from "../../../js/gameModes";

export default function GameModeDropDown({ gameMode, setGameMode }) {
    const handleSelect = (option) => {
        setGameMode(option);
    };

    return (
        <div className="dropdown">
            <button
                className="btn btn-primary dropdown-toggle"
                type="button"
                data-bs-toggle="dropdown"
                aria-expanded="false"
            >
                {gameMode}
            </button>
            <ul className="dropdown-menu">
                {GAME_MODES.map((option, index) => (
                    <li key={index}>
                        <button className="dropdown-item" onClick={() => handleSelect(option)}>
                            {option}
                        </button>
                    </li>
                ))}
            </ul>
        </div>
    );
}
