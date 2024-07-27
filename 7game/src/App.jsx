import { useEffect, useState } from 'react';
import './App.css';
import { socket } from "./socket.js";
import StartPage from "./components/StartPage.jsx"
import Lobby from "./components/Lobby.jsx"

function App() {


    const [lobbyState, setLobbyState] = useState({});
    const [currentPage, setCurrentPage] = useState('StartPage');

    useEffect(() => {
        function leaveLobbyFunc(){
            setCurrentPage('StartPage')
        }
        function conToLobbyFunc(data){
            setLobbyState(data);
            console.table(data)
            setCurrentPage('Lobby')
        }

        socket.on('leaveLobby', leaveLobbyFunc);
        socket.on('conToLobby', conToLobbyFunc);
        return () => {
            socket.off("leaveLobby");
            socket.off("conToLobby");
        };
    }, []);

    switch (currentPage) {
        case 'StartPage':
            return <StartPage/>
        case 'Lobby':
            return <Lobby lobbyState={lobbyState}/>
        case 'Game':

            break;
        default:
            console.log("Default")
    }
}

export default App;

/*
return (
        <>
            <ul className="playingCards table">
        <li>
            <div className="card rank-2 spades"><span className="rank">2</span><span className="suit">&spades;</span></div>
        </li>
        <li>
            <div className="card rank-3 spades"><span className="rank">3</span><span className="suit">&spades;</span></div>
        </li>
        <li>
            <div className="card rank-4 spades"><span className="rank">4</span><span className="suit">&spades;</span></div>
        </li>
        <li>
            <div className="card rank-5 spades"><span className="rank">5</span><span className="suit">&spades;</span></div>
        </li>
        <li>
            <div className="card rank-6 spades"><span className="rank">6</span><span className="suit">&spades;</span></div>
        </li>
        <li>
            <div className="card rank-7 spades"><span className="rank">7</span><span className="suit">&spades;</span></div>
        </li>
        <li>
            <div className="card rank-8 spades"><span className="rank">8</span><span className="suit">&spades;</span></div>
        </li>
        <li>
            <div className="card rank-9 spades"><span className="rank">9</span><span className="suit">&spades;</span></div>
        </li>
        <li>
            <div className="card rank-10 spades"><span className="rank">10</span><span className="suit">&spades;</span></div>
        </li>
        <li>
            <div className="card rank-j spades"><span className="rank">J</span><span className="suit">&spades;</span></div>
        </li>
        <li>
            <div className="card rank-q spades"><span className="rank">Q</span><span className="suit">&spades;</span></div>
        </li>
        <li>
            <div className="card rank-k spades"><span className="rank">K</span><span className="suit">&spades;</span></div>
        </li>
        <li>
            <div className="card rank-a spades"><span className="rank">A</span><span className="suit">&spades;</span></div>
        </li>
    </ul>
        </>
    );
*/