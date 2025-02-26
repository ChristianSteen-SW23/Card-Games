export default function MakeCard({ suit, rank, backSide = false }) {
    return (
        <>
            {!backSide ?
                <div className="playingCards fourColours">
                    <div className={"rank-" + rank + " " + suit + " card"}>
                        <span className="rank">{rank}</span>
                        {rank == "a" || rank == "j" || rank == "q" || rank == "k" ?
                            suit == "diams" ? <span className="suit">&diams;</span> :
                                suit == "spades" ? <span className="suit">&spades;</span> :
                                    suit == "hearts" ? <span className="suit">&hearts;</span> :
                                        <span className="suit">&clubs;</span>
                            : <span className="suit">&nbsp;</span>
                        }
                    </div>
                </div>
                :
                <div className="playingCards fourColours">
                    <div className="card back">*</div>
                </div>

            }
        </>
    );
}
