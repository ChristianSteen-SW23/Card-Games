import { useEffect, useState } from 'react';
import './cards.css';
import { socket } from "./socket.js";
import StartPage from "./components/StartPage.jsx"
import Lobby from "./components/Lobby.jsx"
import GamePage7 from "./components/7game.jsx"
import GamePage31 from './components/31game.jsx';
import GamePage500 from './components/500game.jsx';

function App() {
    const [tempData, setTempData] = useState({});
    const [lobbyState, setLobbyState] = useState({});
    const [currentPage, setCurrentPage] = useState('StartPage');
    const [hand, setHand] = useState([]);

    useEffect(() => {
        function onDisconnect() {
            setCurrentPage('StartPage')
        }

        function leaveLobbyFunc(){
            setCurrentPage('StartPage')
        }
        function conToLobbyFunc(data){
            setLobbyState(data);
            setCurrentPage('Lobby')
        }
        function startedGameFunc7(data){
            setLobbyState(data)
            setCurrentPage('GameMode: 7')
        }
        function handInfoFunc(data){
            console.log(data)
            let newHand = data;
            newHand.sort((a, b) => a - b)
            setHand(newHand);
        }
        function outOfTurnFunc(){
            alert("Not your turn...")
        }
        function notPlayableFunc(){
            alert("You can not play that card")
        }
        function playableFunc(data) {
            console.log(data)
            let newHand = data;
            newHand.sort((a, b) => a - b)
            setHand(newHand);
        }
        function gameInfoFunc(data){
            setLobbyState(data)
        }
        function noSkipFunc(){
            alert("You can play a card. Please stop the cheating")
        }
        function startedGameFunc31(data){
            setLobbyState(data)
            setCurrentPage('GameMode: 31')
        }
        function startedGameFunc500(data) {
            setTempData(data);
            setCurrentPage('GameMode: 500')
        }

        socket.on('disconnect', onDisconnect);
        socket.on('leaveLobby', leaveLobbyFunc);
        socket.on('conToLobby', conToLobbyFunc);
        socket.on('startedGame7', startedGameFunc7);
        socket.on('startedGame31', startedGameFunc31);
        socket.on('handInfo', handInfoFunc);
        socket.on('outOfTurn', outOfTurnFunc);
        socket.on('notPlayable', notPlayableFunc);
        socket.on('playable', playableFunc);
        socket.on('gameInfo', gameInfoFunc);
        socket.on('noSkip', noSkipFunc);
        socket.on('startedGame500', startedGameFunc500);
        return () => {
            socket.off('disconnect', onDisconnect);
            socket.off("leaveLobby");
            socket.off("conToLobby");
            socket.off("startedGame7");
            socket.off("startedGame31");
            socket.off("handInfo");
            socket.off("outOfTurn");
            socket.off("notPlayable");
            socket.off("playable");
            socket.off("gameInfo");
            socket.off("noSkip");
            socket.off("startedGame500");
        };
    }, []);

    switch (currentPage) {
        case 'StartPage':
            return <StartPage/>
        case 'Lobby':
            return <Lobby lobbyState={lobbyState}/>
        case 'GameMode: 31':
            return <GamePage31 lobbyState={lobbyState} hand={hand} setHand={setHand} />
        case 'GameMode: 7':
            return <GamePage7 lobbyState={lobbyState} hand={hand} setHand={setHand} />
        case 'GameMode: 500':
            return <GamePage500 startHand={tempData.hand} startPlayerInfo={tempData.playersInfo} startStackTop={tempData.stack} startStackSize={tempData.stackSize} />
        default:
            console.log("Default")
    }
}

export default App;