import { useState, useEffect, useRef } from "react";
import { socket } from "./../socket";
import BottomOfScreen from "./subComponents/game500/BottomOfScreen";
import PlayersPage from "./subComponents/game500/PlayersPage";
import GameDoneModal from "./subComponents/game500/GameDoneModal";
import Popup from "./subComponents/helperComponents/Popup";

export default function GamePage500({ startHand, startPlayerInfo, startStackTop, startStackSize, startingTurn, deckSizeStart }) {
    const popupRef = useRef();

    // Correctly declare useState hooks
    const [yourTurn, setYourTurn] = useState(socket.id == startingTurn.current);
    const [turn, setTurn] = useState(startingTurn);
    const [hand, setHand] = useState(startHand);
    const [playerInfo, setPlayerInfo] = useState(startPlayerInfo);
    const [stackTop, setStackTop] = useState(startStackTop);
    const [stackSize, setStackSize] = useState(startStackSize);
    const [deckSize, setDeckSize] = useState(deckSizeStart);
    const [selectedCards, setSelectedCards] = useState([]);
    const [turnStep, setTurnStep] = useState("draw");
    const [winPop, setWinPop] = useState(false);
    const [winData, setWinData] = useState([]);

    useEffect(() => {
        function gameInformation(data) {
            let saveTurn = yourTurn;

            setYourTurn(socket.id == data.turn.current);
            setTurn(data.turn);
            setPlayerInfo(data.playersInfo);
            setStackTop(data.stack);
            setStackSize(data.stackSize);
            setDeckSize(data.deckSize);
            setSelectedCards([]);
            setTurnStep(data.gameStep);

            if (socket.id == data.turn.current && data.gameStep == "draw")
                popupRef.current.show("It is your turn🥳", "success");

        }

        function handInformation(data) {
            let newData = data
            setHand(newData.hand);
        }

        function error500Message(data) {
            popupRef.current.show(`Error ${data.type}: ${data.message}`, "error");
        }

        function gameEnded(data) {
            setWinPop(true);
            setWinData(data.winData);
        }

        function newRound(data) {
            console.log("NewRound")
            setWinPop(false);
            setHand(data.hand);
            gameInformation({ ...data, gameStep: "draw" });
        }

        socket.on("gameInformation", gameInformation);
        socket.on("handInformation", handInformation);
        socket.on("error500Message", error500Message);
        socket.on("gameEnded", gameEnded);
        socket.on("newRound", newRound);

        return () => {
            socket.off("gameInformation");
            socket.off("handInformation");
            socket.off("error500Message");
            socket.off("gameEnded");
        };
    }, []);

    return (
        <>
            {/* <div className="container text-center">
                <p><strong>Lobby:</strong> {JSON.stringify(turn)}</p>
                <p><strong>Hand:</strong> {JSON.stringify(hand)}</p>
                <p><strong>Player Info:</strong> {JSON.stringify(playerInfo)}</p>
                <p><strong>Stack Top:</strong> {stackTop}</p>
                <p><strong>Stack Size:</strong> {stackSize}</p>
                <p><strong>GameState:</strong> {turnStep}</p>
            </div> */}
            <div className="p-5">
                <PlayersPage
                    playerInfo={playerInfo}
                    turn={turn}
                />
            </div>
            <BottomOfScreen
                hand={hand}
                selectedCards={selectedCards}
                setSelectedCards={setSelectedCards}
                stackTop={stackTop}
                stackSize={stackSize}
                deckSize={deckSize}
                yourTurn={yourTurn}
                turnStep={turnStep}
                popupRef={popupRef}
            />

            <Popup ref={popupRef} duration={5000} />

            <GameDoneModal winPop={winPop} winData={winData} />

        </>
    );
}



