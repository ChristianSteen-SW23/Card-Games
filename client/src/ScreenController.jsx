import { useEffect, useState } from 'react';
import './css/cards.css';
import { socket } from "./socket.js";
import StartPage from "./components/StartPage.jsx"
import Lobby from "./components/Lobby.jsx"
import GamePage7 from "./components/7game.jsx"
import GamePage31 from './components/31game.jsx';
import GamePage500 from './components/500game.jsx';
import PlanningPoker from './components/PlanningPoker.jsx';

function ScreenController() {
    const [tempData, setTempData] = useState({});
    const [currentPage, setCurrentPage] = useState('StartPage');

    useEffect(() => {
        function onDisconnect() {
            console.log("here123123")
            setCurrentPage('StartPage')
        }

        function leaveLobbyFunc() {
            setCurrentPage('StartPage')
        }
        function conToLobbyFunc(data) {
            console.log(data)
            setTempData(data);
            setCurrentPage('Lobby')
        }
        function startedGameFunc7(data) {
            setTempData(data)
            setCurrentPage('GameMode: 7')
        }
        function startedGameFunc31(data) {
            setTempData(data)
            setCurrentPage('GameMode: 31')
        }
        function startedGameFunc500(data) {
            setTempData(data);
            setCurrentPage('GameMode: 500')
        }
        function startedGameFuncPlanningPoker(data) {
            console.log("HEre123")
            setTempData(data);
            setCurrentPage('GameMode: Planning Poker')
        }

        socket.on('disconnect', onDisconnect);
        socket.on('leaveLobby', leaveLobbyFunc);
        socket.on('conToLobby', conToLobbyFunc);
        socket.on('startedGame7', startedGameFunc7);
        socket.on('startedGame31', startedGameFunc31);
        socket.on('startedGame500', startedGameFunc500);
        socket.on('startedGamePlanningPoker', startedGameFuncPlanningPoker);
        return () => {
            socket.off("disconnect", onDisconnect);
            socket.off("leaveLobby");
            socket.off("conToLobby");
            socket.off("startedGame7");
            socket.off("startedGame31");
            socket.off("startedGame500");
            socket.off("startedGamePlanningPoker");
        };
    }, []);

    switch (currentPage) {
        case 'StartPage':
            return <StartPage />
        case 'Lobby':
            return <Lobby lobbyState={tempData} />
        case 'GameMode: 31':
            return <GamePage31 lobbyStateStart={tempData} />
        case 'GameMode: 7':
            return <GamePage7 lobbyStateStart={tempData} />
        case 'GameMode: 500':
            return <GamePage500 startingTurn={tempData.turn} startHand={tempData.hand} startPlayerInfo={tempData.playersInfo} startStackTop={tempData.stack} startStackSize={tempData.stackSize} deckSizeStart={tempData.deckSize} />
        case 'GameMode: Planning Poker':
            console.log("here");
            return <PlanningPoker lobbyStateStart={tempData} />
        default:
            console.log("Default")
    }
}

export default ScreenController;