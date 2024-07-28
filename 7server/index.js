import http from "http";
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
} from "./Battle.js";
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
      //Does the player have host status
      if (roomData.players.get(socket.id).host) {
        socket.to(roomID).emit("leaveLobby");
        deleteLobby(roomID, io);
      } else {
        // If game has already started, disconnect all clients from game, also if not a host and player is not dead.
        if (roomData.gameStarted && roomData.players.get(socket.id).lives > 0) {
          socket.to(roomID).emit("leaveLobby");
          deleteLobby(roomID, io);
        } else {
          const players = leaveLobby(socket, roomID);
          socket.to(roomID).emit("playerHandler", players);
        }
      }
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

  //Listens for a 'startGame' event and either emits a 'startedGame' event to all clients in a room if conditions are met, or sends a 'cantStartGame' event to the initiating client if not.
  socket.on("startGame", () => {
    const roomID = PlayerRooms.get(socket.id);
    const roomData = Rooms.get(roomID);

    roomData.turn.current = socket.id;
    roomData.turn.next = nextPlayer(roomData);
    roomData.gameStarted = true;

    // Gives players their hand
    for (let [playerid, player] of roomData.players.entries()) {
      player.cardsLeft = 2;
      io.to(playerid).emit("playerInfo", player);
    }

    const playersInfo = mapPlayerInfo(roomData.players);
    const startedGameData = {
      playersInfo,
      turn: roomData.turn,
      board: roomData.board
    };
    console.log(startedGameData)
    io.to(roomID).emit("startedGame", startedGameData);
    console.log("Started game made by", socket.id);
  });
});
// Start application server
server.listen(3000, () => {
  console.log("listening on *:3000");
});
