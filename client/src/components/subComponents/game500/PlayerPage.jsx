import { socket } from "./../../../socket";
import { CurvedHand } from "./../cardMakeing/CurvedHand";


export default function PlayerPage({ info, turn }) {
    let tricksSorted = info.tricks.sort((a, b) => a - b);

    let trickGroups = [[], [], [], []];
    tricksSorted.forEach(element => {
        trickGroups[(element - (element % 13)) / 13].push(element);
    });

    function adjacentNumbers(arr) {
        if (arr.length === 0) return [];

        let result = [];
        let group = [arr[0]];

        for (let i = 1; i < arr.length; i++) {
            if (arr[i] === arr[i - 1] + 1) {
                group.push(arr[i]);
            } else if (arr[0] == 0 && arr[i] == 12) {
                result[0] = [arr[i], ...result[0]];
            } else {
                result.push(group);
                group = [arr[i]];
            }
        }

        result.push(group);

        return result;
    }
    let trickGroupsAdjacent = [adjacentNumbers(trickGroups[0]), adjacentNumbers(trickGroups[1]), adjacentNumbers(trickGroups[2]), adjacentNumbers(trickGroups[3])];

    return (
        <div className="container border p-3 bg-light rounded">
            <div className="d-flex align-items-center">
                <span className="me-2 fw-semibold">Name: {info.name}</span>
                <span className="badge text-bg-success rounded-pill">Handsize: {info.handSize}</span>
                {socket.id == info.id ? <span className="badge text-bg-danger rounded-pill">You</span> : ""}
                {info.id == turn.current ? <span className="badge text-bg-warning text-dark rounded-pill">Current</span> : ""}
                {info.id == turn.next ? <span className="badge text-bg-warning text-dark rounded-pill">Next</span> : ""}
            </div>
            {trickGroupsAdjacent.map((trickSuit, index) => (
                <div key={index} className="row">
                    {trickSuit.map((trick, index2) => (
                        <div key={index2} className={"col-" + (Math.floor(12 / trickSuit.length))}>
                            <CurvedHand key={index2} hand={trick} />
                        </div>
                    ))}
                </div>
            ))}
        </div>
    );
}