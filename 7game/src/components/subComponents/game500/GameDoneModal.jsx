import { socket } from "./../../../socket";

export default function GameDoneModal({ winPop, winData }) {

    return (
        <>
            <div className={`modal fade ${(winPop == true) ? "show" : ""}`} id={"nameModal"} aria-hidden="true" data-bs-backdrop="static" aria-labelledby="staticBackdropLabel" style={{ display: (winPop == true) ? "block" : "none" }}>
                <div className='modal-dialog modal-dialog-scrollable modal-dialog-centered'>
                    <div className='modal-content'>
                        <div className='modal-header'>
                            <h1>Scoreboard:</h1>
                        </div>
                        <div className='modal-body'>
                            <ul className="list-group">
                                <li className="list-group-item d-flex justify-content-between align-items-center">
                                    <strong>Name:</strong>
                                    <span className="badge text-bg-primary rounded-pill">Round Score</span>
                                    <span className="badge text-bg-danger rounded-pill">Total Score</span>
                                </li>
                                {winData.map((player, index) => (
                                    <li key={index} className="list-group-item d-flex justify-content-between align-items-center">
                                        <p>{player.name}</p>
                                        <span className="badge text-bg-primary rounded-pill">{player.roundScore}</span>
                                        <span className="badge text-bg-danger rounded-pill">{player.totalScore}</span>
                                    </li>
                                ))}
                            </ul>
                        </div>
                        <div className='modal-footer'>
                            <button type="button" className="btn btn-success" data-bs-dismiss="modal"
                                onClick={() => { socket.emit("500Move", { "moveType": "playAgain" }) }}>Keep Going</button>
                            {/* <button type="button" className="btn btn-danger" data-bs-dismiss="modal"
                                onClick={() => { socket.emit("disconnect") }}>Leave Lobby</button> */}
                        </div>
                    </div>
                </div>
            </div>
            {(winPop == true) && (
                <div className="modal-backdrop fade show" />
            )}
        </>
    );
}