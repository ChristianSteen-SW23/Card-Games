import { useState, useEffect } from "react";
import { socket } from "./../socket";


export default function GamePage500({ startHand, startPlayerInfo, startStackTop, startStackSize }) {
    // ✅ Correctly declare useState hooks
    const [hand, setHand] = useState(startHand);
    const [playerInfo, setPlayerInfo] = useState(startPlayerInfo);
    const [stackTop, setStackTop] = useState(startStackTop);
    const [stackSize, setStackSize] = useState(startStackSize);

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
        <div className="container text-center">
            <p><strong>Hand:</strong> {JSON.stringify(hand)}</p>
            <p><strong>Player Info:</strong> {JSON.stringify(playerInfo)}</p>
            <p><strong>Stack Top:</strong> {stackTop}</p>
            <p><strong>Stack Size:</strong> {stackSize}</p>
        </div>
    );
}
