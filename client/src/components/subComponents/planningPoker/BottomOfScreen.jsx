import { socket } from "./../../../socket";

export default function BottomOfScreen({
    gameOver,
    valueRange,
    cardValues,
}) {
    if (gameOver) {
        return <NewRound />;
    } else {
        return <ScoreRound valueRange={valueRange} cardValues={cardValues} />;
    }
}

function ScoreRound({ valueRange, cardValues }) {
    function sendValue(value) {
        socket.emit("PlanningPokerMove", { moveType: "playCard", card: value });
    }
    const values = Array.from({ length: valueRange }, (_, i) => i);
    const max = Math.max(...values);
    const min = Math.min(...values);
    return (
        <div className="container">
            <div className="row fixed-bottom planning-bottom-fixed">
                <div className="col text-center py-3 d-flex justify-content-center gap-3 flex-wrap">
                    {values.map((value, index) => {
                        const color = valueColor(value, min, max);

                        return (
                            <button
                                key={index}
                                className="btn fs-5"
                                style={{ backgroundColor: color, color: "white", border: "none" }}
                                onClick={() => sendValue(value)}
                            >
                                {cardValues[value]}
                            </button>
                        );
                    })}
                </div>
            </div>
        </div>
    );
}

function NewRound({ }) {
    function sendValue(value) {
        socket.emit("PlanningPokerMove", { moveType: "playAnotherRound", card: 0 });
    }

    return (
        <div className="container">
            <div className="row fixed-bottom planning-bottom-fixed">
                <div className="col text-center py-3 d-flex justify-content-center">
                    <button
                        className="btn btn-light fs-5 shadow rounded-pill"
                        style={{
                            maxWidth: "300px", // prevent full screen width
                            width: "80%", // responsive scaling
                        }}
                        onClick={sendValue}
                    >
                        Next round
                    </button>
                </div>
            </div>
        </div>
    );
}

export const valueColor = (value, min, max) => {
    const percent = max === min
        ? 0
        : Math.min(Math.max((value - min) / (max - min), 0), 1); // 0 to 1
    const red = Math.round(128 + percent * 127);   // 128 (grey) to 255 (red)
    const other = Math.round(128 - percent * 64);  // 128 (grey) to 64 (red)
    return `rgb(${red}, ${other}, ${other})`;
};