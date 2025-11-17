import { socket } from "./../../../socket";

export default function BottomOfScreen({
    mustNewRound,
    valueRange,
    cardValues,
}) {
    if (mustNewRound) {
        return <NewRound />;
    } else {
        return <ScoreRound valueRange={valueRange} cardValues={cardValues} />;
    }
}

function ScoreRound({ valueRange, cardValues }) {
    function sendValue(value) {
        socket.emit("PlanningPokerMove", { moveType: "choice", value: value });
    }
    const values = Array.from({ length: valueRange }, (_, i) => i);
    const max = Math.max(...values);
    const min = Math.min(...values);
    return (
        <div className="container">
            <div className="row fixed-bottom planning-bottom-fixed">
                <div className="col text-center py-3 d-flex justify-content-center gap-3 flex-wrap">
                    {values.map((value, index) => {
                        const percent = (value - min) / (max - min); // 0 to 1
                        const red = Math.round(128 + percent * 127); // from 128 (gray) to 255 (red)
                        const color = `rgb(${red}, ${128 - percent * 64}, ${128 - percent * 64})`; // from gray to red

                        return (
                            <button
                                key={index}
                                className="btn fs-5"
                                style={{
                                    backgroundColor: color,
                                    color: "white",
                                    border: "none",
                                }}
                                onClick={() => {
                                    sendValue(value);
                                }}
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

function NewRound({}) {
    function sendValue(value) {
        socket.emit("PlanningPokerMove", { moveType: "newRound" });
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
