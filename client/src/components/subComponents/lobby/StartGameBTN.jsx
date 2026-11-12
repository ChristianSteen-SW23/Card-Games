import { socket } from "../../../socket";

export default function StartGameBTN({ players, gameMode }) {
    function startGame() {
        socket.emit("startGame", {"gameMode": gameMode})
    }

    return (
        <>
            <button type="button" className="btn btn-primary p-3 m-3 btn-lg" onClick={startGame} disabled={!(players.length > 0)}>
                Start game {(players.length > 0) ? "" : <span className="badge text-bg-secondary">Need more players to start</span>}
            </button>
        </>
    );
}