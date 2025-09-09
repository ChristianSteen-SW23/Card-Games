import { expect, it, describe, vi } from "vitest";
import { createLobby, joinLobby } from "../Lobby";
import { start7Game } from "../gamelogic/Battle7";
import { nextPlayer } from "../lib/TurnManagement";

describe("game 7 move types", () => {
    it("Right setup of game data", () => {
        const { roomData, socket1, io, roomID } = helperMake7Lobby();
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
    return { roomData, socket1: socket1.id, io, roomID };
}