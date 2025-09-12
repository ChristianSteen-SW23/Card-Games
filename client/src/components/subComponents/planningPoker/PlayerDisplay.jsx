import { socket } from "../../../socket";


export default function PlayerDisplay({ player, mustNewRound, valueRange }) {
    // Clamp value between 0 and valueRange
    const normalized = Math.min(player.value ?? 0, valueRange) / valueRange;

    // Generate color: 0% → grey (#ccc), 100% → red (#f44336)
    const backgroundColor = `hsl(${(1 - normalized) * 0}, 70%, ${80 - normalized * 40}%)`;

    return (
        <div className="container border p-3 bg-light rounded" style={{ minHeight: "200px" }}>
            {/* Top Text Section */}
            <div className="d-flex align-items-start justify-content-start mb-2">
                <span className="me-2 fw-semibold">Name: {player.name}</span>
                {socket.id === player.id && (
                    <span className="badge text-bg-danger rounded-pill ms-2">You</span>
                )}
                {player.ready && (
                    <span className="badge text-bg-primary rounded-pill ms-2">Ready</span>
                )}
            </div>

            <hr className="my-2" />

            {/* Centered Colored Content Section */}
            <div
                className="d-flex justify-content-center align-items-center rounded"
                style={{
                    backgroundColor: mustNewRound ? backgroundColor : "transparent", // only apply when false
                    height: "80%",
                    minHeight: "150px",
                    width: "100%",
                    transition: "background-color 0.3s ease",
                }}
            >
                <span
                    className="text-center fw-bold text-white"
                    style={{
                        fontSize: "clamp(2rem, 10vw, 8rem)",
                        lineHeight: 1,
                    }}
                >
                    {mustNewRound && player.value}
                </span>
            </div>
        </div>
    );
}
