import { useState, useEffect } from "react";
import { socket } from "./../socket";
import DeckAndStack from "./subComponents/game500/DeckAndStack"
import SelectableHand from "./subComponents/cardMakeing/SelectableHand";


export default function GamePage500({ startHand, startPlayerInfo, startStackTop, startStackSize }) {
    // ✅ Correctly declare useState hooks
    const [hand, setHand] = useState(startHand);
    const [playerInfo, setPlayerInfo] = useState(startPlayerInfo);
    const [stackTop, setStackTop] = useState(startStackTop);
    const [stackSize, setStackSize] = useState(startStackSize);
    const [selectedCards, setSelectedCards] = useState([]); 

    useEffect(() => {
        function elseKnockedFunc() {
            alert("You can not knock, someone else has knocked");
        }

        socket.on("New31Game", elseKnockedFunc);

        return () => {
            socket.off("New31Game", elseKnockedFunc); // ✅ Correct cleanup
        };
    }, []);

    return (
        <>
            <div className="container text-center">
                <p><strong>Hand:</strong> {JSON.stringify(hand)}</p>
                <p><strong>Player Info:</strong> {JSON.stringify(playerInfo)}</p>
                <p><strong>Stack Top:</strong> {stackTop}</p>
                <p><strong>Stack Size:</strong> {stackSize}</p>
            </div>
            <div className="container text-center d-flex flex-column min-vh-100">
                {/* Row 1: 2 | 8 | 2 */}
                <div className="row">
                    <div className="col-6 bg-light border">
                        <SelectableHand initialHand={hand} selectedCards={selectedCards}
                            setSelectedCards={setSelectedCards} />

                    </div>
                    <div className="col-6 bg-primary text-white border">
                        <DeckAndStack stackTop={stackTop} stackSize={stackSize} deckSize={5} />
                    </div>
                </div>

                {/* Row 2: 6 | 6 */}
                <div className="row">
                    <div className="col-6 bg-secondary text-white border">Left Half
                    </div>
                    <div className="col-6 bg-dark text-white border">Right Half</div>
                </div>

                {/* Row 3: 6 | 6 */}
                <div className="row">
                    <div className="col-6 bg-warning border">Left Half</div>
                    <div className="col-6 bg-success text-white border">Right Half</div>
                </div>

                {/* Row 4: 12 (Sticky to Bottom) */}
                <div className="row mt-auto">
                    <div className="col-12 bg-danger text-white border">Sticky Bottom</div>
                </div>
            </div>

        </>
    );
}
