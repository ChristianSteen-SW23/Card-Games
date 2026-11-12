import { GAME_MODES } from "../../../js/gameModes";

export default function GameModeDropDown({ gameMode, socket, lobbySettings }) {
    const handleSelect = (option) => {
        let local_settings = lobbySettings
        local_settings.gamemode = option
        switch (option) {
            case "7":
                lobbySettings.gamemode = { [option]: { bot_count: 0 } }
                break;
            case "31":
                lobbySettings.gamemode = { [option]: { bot_count: 0 } }
                break;
            case "500":
                lobbySettings.gamemode = { [option]: { bot_count: 0 } }
                break;
            case "Planning Poker":
                lobbySettings.gamemode = { [option]: { bot_count: 0 } }
                break;
            case "unselected":
                lobbySettings.gamemode = { [option]: { } }
                break;
        }
        // { ...lobbySettings, gamemode: option }
        socket.emit("lobbyControl", { eventType: "changeSettings", "settings": lobbySettings });
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