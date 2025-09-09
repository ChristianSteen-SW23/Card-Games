import { expect, it, describe, vi } from "vitest";
import { createLobby, joinLobby } from "../Lobby";
import { start7Game, call7Move } from "../gamelogic/Battle7";
import { nextPlayer } from "../lib/TurnManagement";

describe("game 7 move types", () => {
    it("Right setup of game data", () => {
        const { roomData, socket1, socket2, socket3, io, roomID } = helperMake7Lobby();
        expect(roomData.box).toBe(null);
        expect(roomData.board).toStrictEqual([
            [null, null, null, null],
            [null, null, null, null],
            [null, null, null, null]
        ]);
        expect(roomData.players.has(roomData.startingPlayerID)).toBe(true);
        expect(roomData.players.has(roomData.turn.current)).toBe(true);
        expect(roomData.players.has(roomData.turn.next)).toBe(true);
        expect([...roomData.players.values()].reduce((acc, player) => acc + player.hand.length, 0)).toBe(52);
    });

    it("Play cards", () => {
        const { roomData, socket1, socket2, socket3, io, roomID } = helperMake7Lobby();

        roomData.players.get(socket1).hand = [19, 20, 21];
        roomData.players.get(socket2).hand = [6, 7, 8];
        roomData.players.get(socket3).hand = [32, 33, 34];
        roomData.turn.current = socket1;
        roomData.turn.next = socket2;
        call7Move(roomData, { moveType: "playCard", card: roomData.players.get(socket1).hand[0] }, socket1, io, roomID);
        call7Move(roomData, { moveType: "playCard", card: roomData.players.get(socket2).hand[0] }, socket2, io, roomID);
        call7Move(roomData, { moveType: "playCard", card: roomData.players.get(socket3).hand[0] }, socket3, io, roomID);
        call7Move(roomData, { moveType: "playCard", card: roomData.players.get(socket1).hand[0] }, socket1, io, roomID);
        call7Move(roomData, { moveType: "playCard", card: roomData.players.get(socket2).hand[0] }, socket2, io, roomID);
        call7Move(roomData, { moveType: "playCard", card: roomData.players.get(socket3).hand[0] }, socket3, io, roomID);
        call7Move(roomData, { moveType: "playCard", card: roomData.players.get(socket1).hand[0] }, socket1, io, roomID);

        expect(roomData.players.get(socket1).hand).toStrictEqual([]);
        expect(roomData.players.get(socket2).hand).toStrictEqual([8]);
        expect(roomData.players.get(socket3).hand).toStrictEqual([34]);
    });

    it("Skip turn", () => {
        const { roomData, socket1, socket2, socket3, io, roomID } = helperMake7Lobby();

        roomData.players.get(socket1).hand = [19, 20, 21];
        roomData.players.get(socket2).hand = [7, 8];
        roomData.players.get(socket3).hand = [33, 34];
        roomData.turn.current = socket1;
        roomData.turn.next = socket2;

        call7Move(roomData, { moveType: "playCard", card: roomData.players.get(socket1).hand[0] }, socket1, io, roomID);
        call7Move(roomData, { moveType: "skipTurn" }, socket2, io, roomID);
        call7Move(roomData, { moveType: "skipTurn" }, socket3, io, roomID);
        call7Move(roomData, { moveType: "playCard", card: roomData.players.get(socket1).hand[0] }, socket1, io, roomID);
        call7Move(roomData, { moveType: "skipTurn" }, socket2, io, roomID);
        call7Move(roomData, { moveType: "skipTurn" }, socket3, io, roomID);
        call7Move(roomData, { moveType: "playCard", card: roomData.players.get(socket1).hand[0] }, socket1, io, roomID);


        expect(roomData.players.get(socket1).hand).toStrictEqual([]);
        expect(roomData.players.get(socket2).hand).toStrictEqual([7, 8]);
        expect(roomData.players.get(socket3).hand).toStrictEqual([33, 34]);
    });

    it("End game and start again", () => {
        const { roomData, socket1, socket2, socket3, io, roomID } = helperMake7Lobby();

        roomData.players.get(socket1).hand = [19, 20, 21];
        roomData.players.get(socket2).hand = [6, 5, 7];
        roomData.players.get(socket3).hand = [33, 34, 35, 36, 37, 38, 39, 40];
        roomData.turn.current = socket1;
        roomData.turn.next = socket2;
        call7Move(roomData, { moveType: "playCard", card: roomData.players.get(socket1).hand[0] }, socket1, io, roomID);
        call7Move(roomData, { moveType: "playCard", card: roomData.players.get(socket2).hand[0] }, socket2, io, roomID);
        call7Move(roomData, { moveType: "skipTurn" }, socket3, io, roomID);
        call7Move(roomData, { moveType: "playCard", card: roomData.players.get(socket1).hand[0] }, socket1, io, roomID);
        call7Move(roomData, { moveType: "playCard", card: roomData.players.get(socket2).hand[0] }, socket2, io, roomID);
        call7Move(roomData, { moveType: "skipTurn" }, socket3, io, roomID);
        call7Move(roomData, { moveType: "playCard", card: roomData.players.get(socket1).hand[0] }, socket1, io, roomID);

        expect(roomData.players.get(socket1).hand).toStrictEqual([]);
        expect(roomData.players.get(socket2).hand).toStrictEqual([7]);
        expect(roomData.players.get(socket3).hand).toStrictEqual([33, 34, 35, 36, 37, 38, 39, 40]);
        expect(roomData.players.get(socket1).totalScore).toBe(0);
        expect(roomData.players.get(socket2).totalScore).toStrictEqual(5);
        expect(roomData.players.get(socket3).totalScore).toStrictEqual(90);

        call7Move(roomData, { moveType: "playAgain" }, socket1, io, roomID);
        expect(roomData.box).toBe(null);
        expect(roomData.board).toStrictEqual([
            [null, null, null, null],
            [null, null, null, null],
            [null, null, null, null]
        ]);
        expect(roomData.players.has(roomData.startingPlayerID)).toBe(true);
        expect(roomData.players.has(roomData.turn.current)).toBe(true);
        expect(roomData.players.has(roomData.turn.next)).toBe(true);
        expect([...roomData.players.values()].reduce((acc, player) => acc + player.hand.length, 0)).toBe(52);
    });

    it("Out of turn", () => {
        const { roomData, socket1, socket2, socket3, io, roomID } = helperMake7Lobby();

        roomData.players.get(socket1).hand = [19];
        roomData.turn.current = socket2;
        roomData.turn.next = socket3;
        roomData.players.get(socket3).hand = [20];
        call7Move(roomData, { moveType: "playCard", card: roomData.players.get(socket1).hand[0] }, socket1, io, roomID);
        call7Move(roomData, { moveType: "skipTurn" }, socket3, io, roomID);

        expect(roomData.players.get(socket1).hand).toStrictEqual([19]);
        expect(roomData.players.get(socket3).hand).toStrictEqual([20]);
    });

    it("Errors in play", () => {
        const { roomData, socket1, socket2, socket3, io, roomID } = helperMake7Lobby();

        roomData.players.get(socket1).hand = [19];
        roomData.turn.current = socket1;
        roomData.turn.next = socket2;
        roomData.players.get(socket2).hand = [21];
        call7Move(roomData, { moveType: "skipTurn" }, socket1, io, roomID);
        roomData.turn.current = socket2;
        roomData.turn.next = socket3;
        call7Move(roomData, { moveType: "playCard", card: roomData.players.get(socket2).hand[0] }, socket2, io, roomID);

        expect(roomData.players.get(socket1).hand).toStrictEqual([19]);
        expect(roomData.players.get(socket2).hand).toStrictEqual([21]);
    });

})

function helperMake7Lobby() {
    // This map contains all rooms and every room's states
    const Rooms = new Map();
    //This map contains all socket id's as keys and has the correlating Rooms key as the value
    const PlayerRooms = new Map();
    const socket1 = {
        id: "ojIckSD2jqNzOqIrAGzL",
        join: vi.fn(),
        emit: vi.fn(), // ✅ Mock emit function
    };
    const socket2 = {
        id: "ghu45DxGsxgy5VCls8Zs",
        join: vi.fn(),
        emit: vi.fn(),
    };
    const socket3 = {
        id: "hjdk2VCls8Zg1fya",
        join: vi.fn(),
        emit: vi.fn(),
    };

    // ✅ Correct way to mock io.to(roomID).emit() using Vitest
    const io = {
        to: vi.fn().mockReturnValue({
            emit: vi.fn(), // ✅ Mock emit inside `to()`
        }),
    };

    // ✅ Create the lobby
    const lobby = createLobby(socket1, "testuser", Rooms, PlayerRooms);
    const roomID = `/${lobby.id}`;

    joinLobby({ name: "testuser2", id: roomID }, roomID, socket2, Rooms, PlayerRooms);
    joinLobby({ name: "testuser3", id: roomID }, roomID, socket3, Rooms, PlayerRooms);
    let roomData = Rooms.get(roomID);

    // ✅ Pass the mocked io object
    start7Game(roomData, socket1.id, io, roomID);

    // ✅ Return an object containing all relevant data
    return { roomData, socket1: socket1.id, socket2: socket2.id, socket3: socket3.id, io, roomID };
}