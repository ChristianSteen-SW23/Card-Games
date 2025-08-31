import { getSetting } from "../../../js/settings";

const useFourColours = getSetting("fourColours");

export function CurvedHand({ hand }) {
    return (
        <>
            <div className={`playingCards rotateHand ${useFourColours ? "fourColours" : ""}`}>
                <ul className="hand">
                    {hand?.map((card, index) => (
                        <li key={card}>
                            {index === hand.length - 1 ? (
                                <CardWithFront card={card} />
                            ) : (
                                <CardWithoutFront card={card} />
                            )}
                        </li>
                    ))}
                </ul>
            </div>
        </>
    );
}


function getSuitEmoji(suit) {
    const suitMap = {
        diams: "♦",
        spades: "♠",
        hearts: "♥",
        clubs: "♣"
    };
    return suitMap[suit] || "?"; // Fallback in case of an error
}

function getSuit(card) {
    return ["diams", "spades", "hearts", "clubs"][(card - (card % 13)) / 13]; // Calculate suit
}

function getRank(card) {
    let rank = (card % 13) + 1;
    return ["a", "2", "3", "4", "5", "6", "7", "8", "9", "10", "j", "q", "k"][rank - 1];
}

function CardWithoutFront({ card, strong = false }) {
    const suit = getSuit(card);
    const rank = getRank(card);

    return (
        <>
            {strong ? (
                <strong>
                    <a className={`card ${suit}`} href="#">
                        <span className="rank">{rank.toLocaleUpperCase()}</span>
                        <span className="suit">{getSuitEmoji(suit)}</span>
                    </a>
                </strong>
            ) : (
                <a className={`card ${suit}`} href="#">
                    <span className="rank">{rank.toLocaleUpperCase()}</span>
                    <span className="suit">{getSuitEmoji(suit)}</span>
                </a>
            )}
        </>
    );
}

function CardWithFront({ card, strong = false }) {
    const suit = getSuit(card);
    const rank = getRank(card);

    return (
        <>
            {strong ? (
                <strong>
                    <a className={`card rank-${rank} ${suit}`} href="#">
                        <span className="rank">{rank.toLocaleUpperCase()}</span>
                        <span className="suit">{getSuitEmoji(suit)}</span>
                    </a>
                </strong>
            ) : (
                <a className={`card rank-${rank} ${suit}`} href="#">
                    <span className="rank">{rank.toLocaleUpperCase()}</span>
                    <span className="suit">{getSuitEmoji(suit)}</span>
                </a>
            )}
        </>
    );
}



