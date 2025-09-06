import http from "http";
import dotenv from "dotenv";
import path from "path";
import { Server } from "socket.io";
import {
  lobbyController,
  deleteLobby,
} from "./Lobby.js";
import {
  start7Game,
  call7Move
} from "./gamelogic/Battle7.js";
import {
  call31Move,
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
  socket.on(("lobbyControl"), (data) => {
    console.log(data);
    lobbyController(data, io, socket);
  });

  //* ================================================= Game Logic Handler ======================================================== *\\
  //Listens for a 'startGame' event and either emits a 'startedGame' event to all clients in a room if conditions are met, or sends a 'cantStartGame' event to the initiating client if not.
  socket.on("startGame", (gameMode) => {
    const roomID = PlayerRooms.get(socket.id);
    const roomData = Rooms.get(roomID);
    switch (gameMode) {
      case "7":
        start7Game(roomData, socket.id, io, roomID);
        break;
      case "31":
        start31Game(roomData, socket.id, io, roomID)
        break;
      case "500":
        start500Game(roomData, socket.id, io, roomID)
        break;
    }
  });

  socket.on("7Move", (data) => {
    console.log(data)
    const roomID = PlayerRooms.get(socket.id)
    const roomData = Rooms.get(roomID)

    call7Move(roomData, data, socket.id, io, roomID);
  });

  socket.on("31Move", (data) => {
    console.log(data)
    const roomID = PlayerRooms.get(socket.id)
    const roomData = Rooms.get(roomID)

    call31Move(roomData, data, socket.id, io, roomID);
  });

  socket.on("500Move", (data) => {
    console.log(data)
    const roomID = PlayerRooms.get(socket.id)
    const roomData = Rooms.get(roomID)

    call500Move(roomData, data, socket.id, io, roomID);
  });

});

// Start application server
dotenv.config({ path: path.resolve("./../.env") });
const PORT = 3100;
const HOST = process.env.BACKEND_IP || "0.0.0.0";
server.listen(PORT, HOST, () => {
  console.log(`Server started on LAN at: http://${HOST}:${PORT}`);
});

