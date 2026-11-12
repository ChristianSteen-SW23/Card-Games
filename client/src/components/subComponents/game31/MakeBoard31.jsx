import { socket } from "../../../socket";
import MakeCard from "../game7/MakeCard";
import { getRank, getSuit } from "./../../../js/cardCal"

export default function MakeBoard({ stack, pickedCard }) {
    function drawCard() {
        let data = {}
        data.moveType = "pickTopCard"
        socket.emit("31Move", data);
    }
    function swapCard() {
        // if(mustPickCard) return alert("You must play a card, not swap");
        // if (pickedCard == -1) return alert("pick a card before playing")
        // if (!(pickedCard == 0 || pickedCard == 1 || pickedCard == 2)) return alert("pick a card to swap")
        let data = {}
        data.card = pickedCard
        data.moveType = "swap"

        socket.emit("31Move", data);
    }

    return (
        <>
            <div className="row">
                <div className="playingCards col-6" onClick={drawCard}>
                    <div className="card back">*</div>
                </div>
                <div className="playingCards col-6" onClick={swapCard}>
                    <MakeCard rank={getRank(stack)} suit={getSuit(stack)} />
                </div>
            </div>
        </>
    );
}
