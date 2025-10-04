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
            setCurrentPage('StartPage')
        }

        function leaveLobbyFunc() {
            setCurrentPage('StartPage')
        }
        function conToLobbyFunc(data, string) {
            data.lobbyState
            data.gameMode
            setTempData(data);
            setCurrentPage('Lobby')
        }

        function startedGameFunc(data) {
            console.log(data);
            setTempData(data);
            setCurrentPage(`GameMode: ${data.gameMode}`)
        }

        socket.on('disconnect', onDisconnect);
        socket.on('leaveLobby', leaveLobbyFunc);
        socket.on('conToLobby', conToLobbyFunc);
        socket.on('startedGame', startedGameFunc);
        return () => {
            socket.off("disconnect", onDisconnect);
            socket.off("leaveLobby");
            socket.off("conToLobby");
            socket.off('startedGame', startedGameFunc);
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
            return <GamePage500 
            startingTurn={tempData.turn} 
            startHand={tempData.hand} 
            startPlayerInfo={tempData.playersInfo} 
            startStackTop={tempData.stack} 
            startStackSize={tempData.stackSize} 
            deckSizeStart={tempData.deckSize} />
        case 'GameMode: Planning Poker':
            console.log("here");
            return <PlanningPoker lobbyStateStart={tempData} />
        default:
            console.log("Default")
    }
}

export default ScreenController;