import http from "http";
import dotenv from "dotenv";
import path from "path";
import { Server } from "socket.io";
import {
  createLobby,
  joinLobby,
  leaveLobby,
  deleteLobby,
  mapToArrayObj,
  isUsernameValid
} from "./Lobby.js";
import {
  mapPlayerInfo,
  nextPlayer,
  dealCards as dealCards7,
  cardPlayable,
  playCard,
  possibleSkip
} from "./gamelogic/Battle7.js";
import {
  dealCards31,
  mapPlayerInfo31,
  cal31Move,
  start31Game
} from "./gamelogic/Battle31.js";
import {
  start500Game,
  call500Move,
} from "./gamelogic/Battle500.js"

const server = http.createServer();

// Socket server that makes use of the above http app server:
const io = new Server(server, {
  cors: {
    origin: "*",
  },
});

// This map contains all rooms and every room's states
const Rooms = new Map();
//This map contains all socket id's as keys and has the correlating Rooms key as the value
const PlayerRooms = new Map();

export { Rooms, PlayerRooms };

// Handle each socket connection
io.on("connection", (socket) => {
  console.log(`a user with the id: ${socket.id} has connected`);
  socket.on("disconnect", () => {
    console.log(`a user with the id: ${socket.id} has disconnected`);
    if (PlayerRooms.has(socket.id)) {
      const roomID = PlayerRooms.get(socket.id);
      const roomData = Rooms.get(roomID);

      socket.to(roomID).emit("leaveLobby");
      deleteLobby(roomID, io);
    }
  });

  //* ================================================= Lobby Handler ======================================================== *\\
  socket.on("createLobby", (username) => {
    if (isUsernameValid(username)) {
      console.log("Lobby was created by", socket.id);
      const createLobbyObj = createLobby(socket, username);
      socket.emit("conToLobby", createLobbyObj);
    } else {
      socket.emit("invalidUsername");
    }
  });

  socket.on("joinLobby", (joined) => {
    const roomID = `/${joined.id}`;
    const roomData = Rooms.get(roomID);
    if (roomData && !roomData.gameStarted) {
      if (isUsernameValid(joined.name)) {
        const playersArr = joinLobby(joined, roomID, socket);
        socket.to(roomID).emit("playerHandler", playersArr);

        //Adds the current settings to the Object for the joining player
        const joinedreturnData = {
          id: joined.id,
          players: playersArr,
        };
        socket.emit("conToLobby", joinedreturnData);
        console.log(joined.name, "/", socket.id, "has joined the lobby with id:", roomID);
      } else {
        socket.emit("invalidUsername");
      }
    } else if (roomData && !roomData.gameStarted) {
      socket.emit("roomFull");
    } else {
      socket.emit("roomNotExist");
    }
  });
  //7---------------------------------------------------------------------------------------------------------------------------------------------------------
  //Listens for a 'startGame' event and either emits a 'startedGame' event to all clients in a room if conditions are met, or sends a 'cantStartGame' event to the initiating client if not.
  socket.on("startGame", (gameMode) => {
    const roomID = PlayerRooms.get(socket.id);
    const roomData = Rooms.get(roomID);

    let startedGameData;
    let playersInfo;

    switch (gameMode) {
      case "7":
        roomData.turn.current = socket.id;
        roomData.turn.next = nextPlayer(roomData);
        roomData.gameStarted = true;

        dealCards7(roomData)
        roomData.turn.current = roomData.startingPlayerID
        roomData.turn.next = nextPlayer(roomData);

        // Gives players their hand
        for (let [playerid, player] of roomData.players.entries()) {
          player.cardsLeft = player.hand.length;
          io.to(playerid).emit("handInfo", player.hand);
        }

        playersInfo = mapPlayerInfo(roomData.players);
        startedGameData = {
          playersInfo,
          turn: roomData.turn,
          board: roomData.board
        };
        io.to(roomID).emit("startedGame7", startedGameData);
        console.log("Started game 7 made by", socket.id);
        break;
      case "31":
        start31Game(roomData, socket.id, io, roomID)
        break;
      case "500":
        start500Game(roomData, socket.id, io, roomID)
        break;
    }
    // console.log(Rooms)
    // console.log("\n\n\n")
    // console.log(PlayerRooms)
  });

  socket.on("playCard", (card) => {
    const roomID = PlayerRooms.get(socket.id)
    const roomData = Rooms.get(roomID)
    if (socket.id == roomData.turn.current) {
      if (cardPlayable(card, roomData)) {
        roomData.players.get(roomData.turn.current).cardsLeft--;
        playCard(card, roomData)
        const playersInfo = mapPlayerInfo(roomData.players);

        // Remove card from hand and sent it out
        let indexToRemove = roomData.players.get(roomData.turn.current).hand.indexOf(card);
        roomData.players.get(roomData.turn.current).hand.splice(indexToRemove, 1);
        io.to(socket.id).emit("playable", roomData.players.get(roomData.turn.current).hand)


        // Set next turn
        roomData.turn.current = roomData.turn.next;
        roomData.turn.next = nextPlayer(roomData);

        // Send out new game info
        const gameInfo = {
          playersInfo,
          turn: roomData.turn,
          board: roomData.board
        };
        io.to(roomID).emit("gameInfo", gameInfo);

      } else {
        io.to(socket.id).emit("notPlayable")
      }
    } else {
      io.to(socket.id).emit("outOfTurn")
    }

  });

  socket.on("skipTurn", (card) => {
    const roomID = PlayerRooms.get(socket.id)
    const roomData = Rooms.get(roomID)
    if (socket.id == roomData.turn.current) {
      if (possibleSkip(roomData, socket.id)) {

        // Set next turn
        roomData.turn.current = roomData.turn.next;
        roomData.turn.next = nextPlayer(roomData);

        // Send out new game info
        const playersInfo = mapPlayerInfo(roomData.players);
        const gameInfo = {
          playersInfo,
          turn: roomData.turn,
          board: roomData.board
        };
        io.to(roomID).emit("gameInfo", gameInfo);
      } else {
        io.to(socket.id).emit("noSkip")
      }

    } else {
      io.to(socket.id).emit("outOfTurn")
    }
  });

  socket.on("31Move", (data) => {
    console.log(data)
    const roomID = PlayerRooms.get(socket.id)
    const roomData = Rooms.get(roomID)

    cal31Move(roomData, data, socket.id, io, roomID);
  })

  socket.on("500Move", (data) => {
    console.log(data)
    const roomID = PlayerRooms.get(socket.id)
    const roomData = Rooms.get(roomID)

    call500Move(roomData, data, socket.id, io, roomID);

  });
});

// Start application server
dotenv.config({ path: path.resolve("./../.env") });
const PORT = 3069;
const HOST = process.env.BACKEND_IP || "0.0.0.0";
server.listen(PORT, HOST, () => {
  console.log(`Server started on LAN at: http://${HOST}:${PORT}`);
});
