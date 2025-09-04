import { useState, useEffect } from "react";
import { socket } from "../socket";
import SettingsModal from "./subComponents/helperComponents/SettingsModal";
import { GAME_MODES } from "../js/gameModes";
import { showPopup } from "../js/popupController";

export default function StartPage() {
    const [showSettings, setShowSettings] = useState(false);

    useEffect(() => {
        function errorMessage(data) {
            showPopup(`Error ${data.type}: ${data.message}`, "error");
        }
        socket.on("errorMessage", errorMessage);
        return () => {
            socket.off("errorMessage");
        };
    }, []);

    return (
        <>
            <button
                type="button"
                className="btn btn-outline-secondary position-absolute top-0 end-0 m-3 fs-4 py-2 px-3"
                onClick={() => setShowSettings(true)}
                title="Open settings"
            >
                <i className="bi bi-person-fill-gear"></i>
            </button>

            <div className="container text-center">
                <h1 className="p-4 text-center">Card Games</h1>
                <h4 className="p-1 text-center">Game modes are:</h4>
                <h4 className="p-1 text-center">{GAME_MODES.join(" | ")}</h4>
                <div className="p-5"></div>
                <div className="row justify-content-md-center">
                    <div className="d-grid gap-2 col-6">
                        <JoinGameModalButton />
                    </div>
                    <div className="d-grid gap-2 col-6">
                        <HostGameModalButton />
                    </div>
                </div>
            </div>

            <SettingsModal show={showSettings} onClose={() => setShowSettings(false)} />
        </>
    );
}

function HostGameModalButton() {
    const [displayName, setDisplayName] = useState("");

    function hostGame() {
        socket.emit("lobbyControl", { eventType: "createLobby", username: displayName });
    }

    return (
        <>
            <button
                type="button"
                className="btn btn-primary btn-lg"
                data-bs-toggle="modal"
                data-bs-target="#hostGameModal"
            >
                Host Game
            </button>
            <div
                className="modal fade"
                id="hostGameModal"
                tabIndex="-1"
                aria-labelledby="hostGameModalLabel"
                aria-hidden="true"
            >
                <div className="modal-dialog">
                    <div className="modal-content">
                        <div className="modal-header">
                            <h1 className="modal-title fs-5" id="hostGameModalLabel">
                                Host Game
                            </h1>
                            <button
                                type="button"
                                className="btn-close"
                                data-bs-dismiss="modal"
                                aria-label="Close"
                            ></button>
                        </div>
                        <div className="modal-body">
                            <form onSubmit={(e) => e.preventDefault()}>
                                <div className="mb-3">
                                    <label
                                        htmlFor="display-name"
                                        className="col-form-label text-start"
                                    > Enter a name:
                                    </label>
                                    <input
                                        type="text"
                                        className="form-control"
                                        id="display-name"
                                        value={displayName}
                                        onChange={(e) => setDisplayName(e.target.value)}
                                    />
                                </div>
                            </form>
                        </div>
                        <div className="modal-footer">
                            <button
                                type="button"
                                className="btn btn-secondary"
                                data-bs-dismiss="modal"
                            > Close
                            </button>
                            <button
                                type="button"
                                className="btn btn-primary"
                                data-bs-dismiss="modal"
                                onClick={hostGame}
                            > Create Lobby
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
}

function JoinGameModalButton() {
    const [displayName, setDisplayName] = useState("");
    const [gameCode, setGameCode] = useState("");

    function joinGame() {
        socket.emit("lobbyControl", { eventType: "joinLobby", name: displayName, id: gameCode });
    }

    return (
        <>
            <button
                type="button"
                className="btn btn-primary btn-lg"
                data-bs-toggle="modal"
                data-bs-target="#joinGameModal"
            > Join Game
            </button>
            <div
                className="modal fade"
                id="joinGameModal"
                tabIndex="-1"
                aria-labelledby="joinGameModalLabel"
                aria-hidden="true"
            >
                <div className="modal-dialog">
                    <div className="modal-content">
                        <div className="modal-header">
                            <h1 className="modal-title fs-5" id="joinGameModalLabel"> Join Game
                            </h1>
                            <button
                                type="button"
                                className="btn-close"
                                data-bs-dismiss="modal"
                                aria-label="Close"
                            ></button>
                        </div>
                        <div className="modal-body">
                            <form onSubmit={(e) => e.preventDefault()}>
                                <div className="mb-3">
                                    <label
                                        htmlFor="game-code"
                                        className="col-form-label text-start"
                                    > Enter Game Code:
                                    </label>
                                    <input
                                        type="text"
                                        className="form-control"
                                        id="game-code"
                                        value={gameCode}
                                        onChange={(e) => setGameCode(e.target.value)}
                                    />
                                    <label
                                        htmlFor="display-name"
                                        className="col-form-label text-start"
                                    > Enter a name:
                                    </label>
                                    <input
                                        type="text"
                                        className="form-control"
                                        id="display-name"
                                        value={displayName}
                                        onChange={(e) => setDisplayName(e.target.value)}
                                    />
                                </div>
                            </form>
                        </div>
                        <div className="modal-footer">
                            <button
                                type="button"
                                className="btn btn-secondary"
                                data-bs-dismiss="modal"
                            > Close
                            </button>
                            <button
                                type="button"
                                className="btn btn-primary"
                                data-bs-dismiss="modal"
                                onClick={joinGame}
                            > Join Lobby
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
}
