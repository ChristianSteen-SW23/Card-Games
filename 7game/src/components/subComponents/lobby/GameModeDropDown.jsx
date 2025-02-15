import { useState } from "react";

export default function GameModeDropDown({ gameMode, setGameMode, availableGames }) {
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
                {availableGames.map((option, index) => (
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
