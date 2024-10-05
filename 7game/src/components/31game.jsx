import { useState, useEffect } from "react";
import { socket } from "./../socket";
import InGamePlayerList31 from "./subComponents/game31/InGamePlayerList31";
import MakeBoard31 from "./subComponents/game31/MakeBoard31"
import MakeHand31 from "./subComponents/game31/MakeHand31";


export default function GamePage31({ lobbyState, hand, setHand }) {
    function knockBTNFunc(){


    }

    const [pickedCard, setPickedCard] = useState(-1);

    /*useEffect(() => {
        console.log(pickedCard)
    }, [pickedCard]);*/
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
        </>
    );
}
