import SelectableHand from "./../cardMakeing/SelectableHand";
import DeckAndStack from "./DeckAndStack";
import { socket } from "./../../../socket";


export default function BottomOfScreen({ hand, selectedCards, setSelectedCards, stackSize, stackTop, deckSize, yourTurn, turnStep, popupRef }) {
    function makeTrick() {
        socket.emit("500Move", { "moveType": "playTrick", "cardsToPlay": selectedCards });
    }

    function drawFunction(drawType) {
        socket.emit("500Move", { "moveType": "draw", "drawKind": drawType });
    }

    function endTurn() {
        if (hand.length == 0) return socket.emit("500Move", { "moveType": "endTurn", "cardToPlay": -1 });
        if (selectedCards.length != 1) return popupRef.current.show("Pick 1 card to end your turn !!!", "error");;
        socket.emit("500Move", { "moveType": "endTurn", "cardToPlay": selectedCards[0] });
    }

    return (
        <>
            <div className="container">
                <div className="row fixed-bottom bg-light border">
                    <div className="col text-center py-3 bg-dark fixed-bottom d-flex justify-content-center gap-3 flex-wrap">
                        <button type="button" className="btn btn-primary" disabled={!yourTurn || turnStep != "trick"} onClick={makeTrick}>
                            Make Trick
                        </button>
                        <button type="button" className="btn btn-success" disabled={!yourTurn || turnStep != "draw"} onClick={() => drawFunction("decktop")}>
                            Draw Deck Top
                        </button>
                        <button type="button" className="btn btn-info text-dark" disabled={!yourTurn || turnStep != "draw"} onClick={() => drawFunction("stacktop")}>
                            Draw Stack Top
                        </button>
                        <button type="button" className="btn btn-warning text-dark" disabled={!yourTurn || turnStep != "draw"} onClick={() => drawFunction("stack")}>
                            Draw Hole Stack
                        </button>
                        {/* <button type="button" className="btn btn-secondary" disabled={!yourTurn}>
                            Romi
                        </button> */}
                        <button type="button" className="btn btn-danger" disabled={!yourTurn || turnStep != "trick"} onClick={endTurn}>
                            End Turn
                        </button>
                    </div>
                    <div className="row">
                        <div className="col-8 d-flex justify-content-center align-items-center py-3">
                            <SelectableHand initialHand={hand} selectedCards={selectedCards} setSelectedCards={setSelectedCards} />
                        </div>
                        <div className="col-4 d-flex align-items-center py-3">
                            <DeckAndStack stackTop={stackTop} stackSize={stackSize} deckSize={deckSize} />
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
}
