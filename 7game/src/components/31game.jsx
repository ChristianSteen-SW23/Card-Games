import { useState, useEffect } from "react";
import { socket } from "./../socket";
import InGamePlayerList31 from "./subComponents/game31/InGamePlayerList31";
import MakeBoard31 from "./subComponents/game31/MakeBoard31"
import MakeHand31 from "./subComponents/game31/MakeHand31";


export default function GamePage31({ lobbyState, hand, setHand }) {
    const [winPop, setWinPop] = useState(false);
    const [winData, setWinData] = useState([]);
    const [pickedCard, setPickedCard] = useState(-1);
    const [mustPickCard, setMustPickCard] = useState(false);

    useEffect(() => {
        function elseKnockedFunc() {
            alert("You can not knock, someone else have knocked")
        }

        function GameOver31Func(data) {
            let newData = data
            setWinData(newData)
            setWinPop(true)
        }

        function MustDrawFromHandFunc(data) {
            console.log(data)
            let newData = data
            setWinData(newData)
            setWinPop(true)
        }

        function hand31Func(data) {
            console.log(data)
            let newHand = data;
            setHand(newHand);
            if(pickedCard == 3) setPickedCard(-1);
            setMustPickCard(false)
        }

        socket.on('elseKnocked', elseKnockedFunc);
        socket.on('GameOver31', GameOver31Func);
        socket.on('MustDrawFromHand', MustDrawFromHandFunc);
        socket.on('hand31', hand31Func);
        
        return () => {
            socket.off("elseKnocked");
            socket.off("GameOver31");
            socket.off('MustDrawFromHand');
            socket.off("hand31");
        };
    }, []);

    useEffect(()=>{
        socket.emit("31Move", { moveType: "ReadyForHand" });
    },[])

    function knockBTNFunc() {
        socket.emit("31Move", { moveType: "Knock" });
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
                        <MakeBoard31 stack={lobbyState.stack} pickedCard={pickedCard} />
                    </div>
                </div>
                <div className="row">
                    <MakeHand31 hand={hand} setPickedCard={setPickedCard} pickedCard={pickedCard} />
                </div>
                <div className="row">
                    <button type="button" className="btn btn-primary p-3 m-3 btn-lg" onClick={knockBTNFunc}>
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
                            <button type="button" className="btn btn-danger" data-bs-dismiss="modal"
                                onClick={console.log("hej")}>Go to front page</button>
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
