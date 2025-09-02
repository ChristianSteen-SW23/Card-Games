import PlayerPage from "./PlayerPage";

export default function PlayersPage({ playerInfo, turn }) {
    const cols = {
        2: "col-6",
        3: "col-4",
        4: "col-3"
    }[playerInfo.length] ?? "col-2";

    return (
        <div className="border p-3 bg-light rounded">
            <div className="row">
                {playerInfo.map((player) => (
                    <div key={player.id} className={cols}>
                        <PlayerPage info={player} turn={turn} />
                    </div>
                ))}
            </div>
        </div>
    );
}