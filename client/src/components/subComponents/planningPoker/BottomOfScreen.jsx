import { socket } from "./../../../socket";


export default function BottomOfScreen({ }) {

    function sendValue(value) {
        console.log("here")
        socket.emit("PlanningPokerMove", { "moveType": "choice", "value": value });
    }
    const values = Array.from({ length: 15 }, (_, i) => i + 1);
    const max = Math.max(...values);
    const min = Math.min(...values);
    return (
        <div className="container">
            <div className="row fixed-bottom bg-light border">
                <div className="col text-center py-3 bg-dark d-flex justify-content-center gap-3 flex-wrap">
                    {values.map((value, index) => {
                        const percent = (value - min) / (max - min); // 0 to 1
                        const red = Math.round(128 + percent * 127); // from 128 (gray) to 255 (red)
                        const color = `rgb(${red}, ${128 - percent * 64}, ${128 - percent * 64})`; // from gray to red

                        return (
                            <button
                                key={index}
                                className="btn"
                                style={{
                                    backgroundColor: color,
                                    color: "white",
                                    border: "none"
                                }}
                                onClick={sendValue}
                            >
                                {value}
                            </button>
                        );
                    })}
                </div>
            </div>
        </div>
    );
}
