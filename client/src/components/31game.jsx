import { useState, useEffect } from "react";
import { socket } from "./../socket";
import InGamePlayerList31 from "./subComponents/game31/InGamePlayerList31";
import MakeBoard31 from "./subComponents/game31/MakeBoard31"
import MakeHand31 from "./subComponents/game31/MakeHand31";
import { showPopup } from "../js/popupController";


export default function GamePage31({ lobbyStateStart }) {
    const [lobbyState, setLobbyState] = useState(lobbyStateStart);
    const [winPop, setWinPop] = useState(false);
    const [winData, setWinData] = useState([]);
    const [pickedCard, setPickedCard] = useState(10);
    const [mustPickCard, setMustPickCard] = useState(false);
    const [hand, setHand] = useState(lobbyStateStart.handInfo);

    useEffect(() => {
        function errorMessage(data) {
            showPopup(`Error ${data.type}: ${data.message}`, "error");
        }

        function GameOver31Func(data) {
            console.log(data)
            let temp = data.winData; 
            setWinData(temp)
            setWinPop(true)
        }
        function hand31Func(data) {
            let newHand = data.handInfo;
            setHand(newHand);
            if (data.handInfo.length == 4) 
                setMustPickCard(true);
            else 
                setMustPickCard(false);

        }

        function gameInfoFunc(data) {
            console.log(data)
            setLobbyState(data);
            if (socket.id == data.turn.current)
                showPopup("It is your turn🥳", "success");
            if (!data.gameOver)
                setWinPop(false);
        }


        socket.on("errorMessage", errorMessage);
        socket.on('gameOver31', GameOver31Func);
        socket.on('handInfo', hand31Func);
        socket.on('gameInfo', gameInfoFunc);

        return () => {
            socket.off("errorMessage");
            socket.off("gameOver31");
            socket.off("handInfo");
            socket.off("gameInfo");
        };
    }, []);

    function knockBTNFunc() {
        socket.emit("31Move", { moveType: "knock" });
    }

    function playCardBTNFunc() {
        let data = {}
        data.card = pickedCard
        data.moveType = "playTopCard"

        socket.emit("31Move", data);
    }


    return (
        <>
            <div className="container text-center">
                <div className="row p-3"></div>
                <div className="row">
                    <div className="col-4">
                        <InGamePlayerList31 lobbyState={lobbyState} />
                    </div>
                    <div className="col-8">
                        <MakeBoard31 stack={lobbyState.topStackCard} pickedCard={pickedCard} />
                    </div>
                </div>
                <div className="row">
                    <MakeHand31 hand={hand} setPickedCard={setPickedCard} pickedCard={pickedCard} />
                </div>
                <div className="row" >
                    <button type="button" className="btn btn-primary p-3 m-3 btn-lg" onClick={playCardBTNFunc} disabled={!mustPickCard}>
                        Play select card
                    </button>
                </div>
                <div className="row">
                    <button type="button" className="btn btn-danger p-3 m-3 btn-lg" onClick={knockBTNFunc} disabled={mustPickCard}>
                        Knock table
                    </button>
                </div>
            </div>

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
                                    <span className="badge text-bg-primary rounded-pill">Total Wins</span>
                                </li>
                                {winData.map((player, index) => (
                                    <li key={index} className="list-group-item d-flex justify-content-between align-items-center">
                                        <p>{player.name}</p>
                                        <span className="badge text-bg-primary rounded-pill">{player.roundScore}</span>
                                        <span className="badge text-bg-primary rounded-pill">{player.totalWins}</span>
                                    </li>
                                ))}
                            </ul>
                        </div>
                        <div className='modal-footer'>
                            <button type="button" className="btn btn-success" data-bs-dismiss="modal"
                                onClick={() => { socket.emit("31Move", { moveType: "newGame" }) }}>Play again?</button>
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
