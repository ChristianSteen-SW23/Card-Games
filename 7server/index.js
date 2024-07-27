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
  removeCardFromHand,
  drawHand,
  checkWinner,
  mapToPlayerLives,
  nextPlayer,
  switchRoles,
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

  socket.on("lobbyLeave", () => {
    const roomID = PlayerRooms.get(socket.id);
    const players = leaveLobby(socket, roomID);
    socket.to(roomID).emit("playerHandler", players);
    socket.emit("leaveLobby");
  });

  socket.on("changeDeck", (deck) => {
    const roomID = PlayerRooms.get(socket.id);
    const roomData = Rooms.get(roomID);

    const isPossible = changeDeckState(deck, socket.id, roomData);

    if (isPossible) {
      //Emit to other players that the host has readied up
      const playerArr = mapToArrayObj(roomData.players);
      io.to(roomID).emit("playerHandler", playerArr);

      socket.emit("changeDeck", deck.name);
    } else {
      socket.emit("deckNotAccepted");
    }
  });

  socket.on("deleteLobby", () => {
    const roomID = PlayerRooms.get(socket.id);
    io.to(roomID).emit("leaveLobby");
    deleteLobby(roomID, io);
  });

  //Listens for player ready and returns the players readyness status.
  socket.on("playerReady", () => {
    const roomID = PlayerRooms.get(socket.id);
    const playerReadyStatus = playerReady(socket.id, roomID);
    console.log("player", socket.id, "changed ready status");
    io.to(roomID).emit("playerHandler", playerReadyStatus);
  });

  //Listens for a 'startGame' event and either emits a 'startedGame' event to all clients in a room if conditions are met, or sends a 'cantStartGame' event to the initiating client if not.
  socket.on("startGame", () => {
    const roomID = PlayerRooms.get(socket.id);
    if (shouldStartGame(roomID)) {
      const roomData = Rooms.get(roomID);

      //give each player lives according to settings
      let lifeAmount = roomData.settings.life;

      //give players correct information and draw their hand
      for (let [playerid, player] of roomData.players.entries()) {
        let hand = drawHand(
          player.deck,
          roomData.settings.handSize,
        );
        player.lives = lifeAmount;
        player.hand = [...hand];

        io.to(playerid).emit("playerInfo", player);
      }

      roomData.turn.current = socket.id;
      roomData.turn.next = nextPlayer(roomData);

      // Calculate the minimum amount of cards present in all decks
      roomData.maxDeckSize = calculateMaxDeckSize(roomData);
      roomData.gameStarted = true;

      const playerLives = mapToPlayerLives(roomData.players);
      const startedGameData = {
        playerLives: playerLives,
        maxLives: lifeAmount,
        handSize: roomData.settings.handSize,
        turn: roomData.turn,
      };
      io.to(roomID).emit("startedGame", startedGameData);
      console.log("Started game made by", socket.id);
    } else {
      console.log("Can not start game made by", socket.id);
      socket.emit("cantStartGame");
    }
  });
});
// Start application server
server.listen(3000, () => {
  console.log("listening on *:3000");
});
