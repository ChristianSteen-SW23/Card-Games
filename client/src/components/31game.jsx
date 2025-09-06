import { useState, useEffect } from "react";
import { socket } from "./../socket";
import InGamePlayerList31 from "./subComponents/game31/InGamePlayerList31";
import MakeBoard31 from "./subComponents/game31/MakeBoard31"
import MakeHand31 from "./subComponents/game31/MakeHand31";
import Popup from "./subComponents/helperComponents/Popup";
import { showPopup } from "../js/popupController";

export default function GamePage31({ lobbyStateStart }) {
    const [lobbyState, setLobbyState] = useState(lobbyStateStart);
    const [winPop, setWinPop] = useState(false);
    const [winData, setWinData] = useState([]);
    const [pickedCard, setPickedCard] = useState(-1);
    const [mustPickCard, setMustPickCard] = useState(false);
    const [hand, setHand] = useState([]);

    useEffect(() => {
        function elseKnockedFunc() {
            showPopup(`Error Not Possible: You can not knock, someone have knocked`, "error");
        }

        function GameOver31Func(data) {
            let newData = data
            setWinData(newData)
            setWinPop(true)
        }

        function MustDrawFromHandFunc(data) {
            let newHand = data;
            setHand(newHand);
            setPickedCard(-1);
            setMustPickCard(true);
        }

        function hand31Func(data) {
            let newHand = data;
            setHand(newHand);
            if (pickedCard == 3) setPickedCard(-1);
            setMustPickCard(false)
        }

        function New31GameFunc() {
            setWinPop(false);
            socket.emit("31Move", { moveType: "ReadyForHand" });
        }

        function gameInfoFunc(data) {
            setLobbyState(data);
            if (socket.id == data.turn.current)
                showPopup("It is your turn🥳", "success");
        }

        socket.on('elseKnocked', elseKnockedFunc);
        socket.on('GameOver31', GameOver31Func);
        socket.on('MustDrawFromHand', MustDrawFromHandFunc);
        socket.on('hand31', hand31Func);
        socket.on('New31Game', New31GameFunc);
        socket.on('gameInfo', gameInfoFunc);

        return () => {
            socket.off("elseKnocked");
            socket.off("GameOver31");
            socket.off('MustDrawFromHand');
            socket.off("hand31");
            socket.off("New31Game");
            socket.off("gameInfo");
        };
    }, []);

    useEffect(() => {
        socket.emit("31Move", { moveType: "ReadyForHand" });
    }, [])

    function knockBTNFunc() {
        socket.emit("31Move", { moveType: "Knock" });
    }

    function playCardBTNFunc() {
        if (pickedCard == -1) return;
        let data = {}
        data.card = pickedCard
        data.moveType = "cardPicked"

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
                        <MakeBoard31 stack={lobbyState.stack} pickedCard={pickedCard} mustPickCard={mustPickCard} />
                    </div>
                </div>
                <div className="row">
                    <MakeHand31 hand={hand} setPickedCard={setPickedCard} pickedCard={pickedCard} />
                </div>
                <div className="row" >
                    <button type="button" className="btn btn-success p-3 m-3 btn-lg" onClick={playCardBTNFunc} hidden={!mustPickCard}>
                        Play select card
                    </button>
                </div>
                <div className="row">
                    <button type="button" className="btn btn-primary p-3 m-3 btn-lg" onClick={knockBTNFunc} disabled={mustPickCard}>
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
                                    <span className="badge text-bg-primary rounded-pill">Score</span>
                                </li>
                                {winData.map((player, index) => (
                                    <li key={index} className="list-group-item d-flex justify-content-between align-items-center">
                                        <p>{player.name}</p>
                                        <span className="badge text-bg-primary rounded-pill">{player.point}</span>
                                    </li>
                                ))}
                            </ul>
                        </div>
                        <div className='modal-footer'>
                            <button type="button" className="btn btn-success" data-bs-dismiss="modal"
                                onClick={() => { socket.emit("31Move", { moveType: "NewGame", data: true }) }}>Play again?</button>
                            <button type="button" className="btn btn-danger" data-bs-dismiss="modal"
                                onClick={() => { socket.emit("31Move", { moveType: "NewGame", data: false }) }}>Leave lobby</button>
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
