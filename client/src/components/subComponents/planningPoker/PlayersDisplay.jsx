import PlayerDisplay from "./PlayerDisplay";

export default function PlayersDisplay({ players, mustNewRound, valueRange }) {
    const half = Math.ceil(players.length / 2);
    const topRow = players.slice(0, half);
    const bottomRow = players.slice(half);

    // Determine column width based on largest row
    const maxRowLength = Math.max(topRow.length, bottomRow.length);
    const colWidth = Math.floor(12 / maxRowLength);
    const colClass = `col-${colWidth}`;

    return (
        <div style={{ overflow: "hidden" }}>
            <div className="m-3 border p-3 bg-light rounded">
                <div className="row mb-3 justify-content-center">
                    {topRow.map((player) => (
                        <div key={player.id} className={colClass}>
                            <PlayerDisplay player={player} mustNewRound={mustNewRound} valueRange={valueRange} />
                        </div>
                    ))}
                </div>
                <div className="row justify-content-center">
                    {bottomRow.map((player) => (
                        <div key={player.id} className={colClass}>
                            <PlayerDisplay player={player} mustNewRound={mustNewRound} valueRange={valueRange} />
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
}