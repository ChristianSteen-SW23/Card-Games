import { useState } from "react";
import { socket } from "../socket";

export default function StartPage() {
    return (
        <div className="container text-center">
            <h1 className="p-5 text-center">Card Games</h1>
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
    );
}

function HostGameModalButton() {
    const [displayName, setDisplayName] = useState("");

    function hostGame() {
        socket.emit("createLobby", displayName);
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
        socket.emit("joinLobby", { name: displayName, id: gameCode });
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
