export default function SelectableHand({ initialHand, selectedCards, setSelectedCards }) {

    function toggleCardSelection(card) {
        setSelectedCards(prevSelected =>
            prevSelected.includes(card)
                ? prevSelected.filter(c => c !== card) // Remove if already selected
                : [...prevSelected, card] // Add if not selected
        );
    }

    function isSelected(card) {
        return selectedCards.includes(card);
    }

    return (
        <>
            {/*<div className="playingCards rotateHand fourColours">*/}
            <div className="playingCards fourColours">
                <ul className="hand">
                    {initialHand?.map((card, index) => (
                        <li key={card} onClick={() => toggleCardSelection(card)}>
                            {index === initialHand.length - 1 ? (
                                <CardWithFront card={card} strong={isSelected(card)} />
                            ) : (
                                <CardWithoutFront card={card} strong={isSelected(card)} />
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



