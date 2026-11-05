import { useState, useEffect } from "react";
import { socket } from "./../socket";
import LobbyID from "./subComponents/lobby/LobbyID";
import ListPlayersLobby from "./subComponents/lobby/ListPlayersLobby";
import StartGameBTN from "./subComponents/lobby/StartGameBTN";
import GameModeDropDown from "./subComponents/lobby/GameModeDropDown";
import { getSetting } from "../js/settings";
import { showPopup } from "../js/popupController";

export default function Lobby({ lobbyState }) {
    const [players, setPlayers] = useState(lobbyState.players);
    const [gameModeName, setGameModeName] = useState(getGameModeName(lobbyState.lobbySettings.gamemode));
    const [settings, setSettings] = useState(lobbyState.lobbySettings);

    const player = players.find(player => player.playerid === socket.id);
    const isHost = player.host;
    const roomID = lobbyState.id;

    useEffect(() => {
        socket.on("playerHandler", (data) => {
            console.log(data)
            setPlayers(data.players);
            setSettings(data.lobbySettings);
            setGameModeName(getGameModeName(lobbyState.lobbySettings.gamemode))
        });

        function errorMessage(data) {
            showPopup(`Error ${data.type}: ${data.message}`, "error");
        }
        socket.on("errorMessage", errorMessage);
        return () => {
            socket.off("playerHandler");
            socket.off("errorMessage");

        };

    }, []);


    return (
        <div className="container mt-4">
            <LobbyID roomID={roomID} />

            <div className="row g-3">
                {/* Left: players */}
                <div className="col-8">
                    <div className="border rounded p-3 h-100">
                        <ListPlayersLobby players={players} />
                    </div>
                </div>

                {/* Right: gamemode on top, settings below */}
                <div className="col-4">
                    <div className="border rounded p-3 mb-3">
                        <h5>Gamemode</h5>
                        {isHost
                            ? <GameModeDropDown gameMode={gameModeName} setGameMode={setGameModeName} lobbySettings={settings} socket={socket} />
                            : <p className="mb-0">{gameModeName}</p>}
                    </div>

                    <div className="border rounded p-3">
                        <h5>Settings</h5>
                        {isHost
                            ? <SettingsHost settings={settings} socket={socket} />
                            : <SettingsPlayer settings={settings} />}
                    </div>
                </div>
            </div>

            {/* Bottom: start game */}
            {isHost && (
                <div className="d-grid mt-3">
                    <StartGameBTN players={players} gameMode={gameModeName} />
                </div>
            )}
        </div>
    );
}

const getGameModeName = (gamemode) =>
    typeof gamemode === "string" ? gamemode : Object.keys(gamemode)[0];
// Gets the game specific settings, e.g. { bot_count: 4 }
const getGameModeSettings = (gamemode) =>
    typeof gamemode === "string" ? {} : Object.values(gamemode)[0] ?? {};

// Turns "bot_count" or "maxPlayers" into "Bot count" or "Max players"
const toLabel = (key) => {
    const text = key.replace(/_/g, " ").replace(/([A-Z])/g, " $1").toLowerCase();
    return text.charAt(0).toUpperCase() + text.slice(1);
};

function SettingsPlayer({ settings }) {
    const general = { maxPlayers: settings.maxPlayers };
    const modeSettings = getGameModeSettings(settings.gamemode);

    return (
        <ul className="list-group">
            {Object.entries({ ...general, ...modeSettings }).map(([key, value]) => (
                <li key={key} className="list-group-item d-flex justify-content-between">
                    <span>{toLabel(key)}</span>
                    <span className="fw-bold">{value}</span>
                </li>
            ))}
        </ul>
    );
}

function SettingsHost({ settings, socket }) {
    const modeName = getGameModeName(settings.gamemode);
    const modeSettings = getGameModeSettings(settings.gamemode);

    const send = (newSettings) => {
        socket.emit("lobbyControl", { eventType: "changeSettings", settings: newSettings });
    };

    const changeGeneral = (key, value) => {
        send({ ...settings, [key]: Number(value) });
    };

    const changeGameMode = (key, value) => {
        send({
            ...settings,
            gamemode: { [modeName]: { ...modeSettings, [key]: Number(value) } }
        });
    };

    const field = (key, value, onChange) => (
        <div className="mb-3 text-start" key={key}>
            <label className="form-label">{toLabel(key)}</label>
            <input
                type="number"
                className="form-control mb-1"
                value={value}
                onChange={(e) => onChange(key, e.target.value)}
            />
            <div className="btn-group w-100">
                <button
                    type="button"
                    className="btn btn-outline-primary"
                    disabled={value <= 0}
                    onClick={() => onChange(key, value - 1)}
                >
                    ➖
                </button>
                <button
                    type="button"
                    className="btn btn-outline-primary"
                    onClick={() => onChange(key, value + 1)}
                >
                    ➕
                </button>
            </div>
        </div>
    );

    return (
        <>
            {field("maxPlayers", settings.maxPlayers, changeGeneral)}
            {Object.entries(modeSettings).map(([key, value]) =>
                field(key, value, changeGameMode)
            )}
        </>
    );
}