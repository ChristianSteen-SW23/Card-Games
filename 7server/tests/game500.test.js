import { expect, it, describe } from "vitest";
import { createLobby, joinLobby, Rooms } from "../Lobby";
import { start500Game, call500Move } from "../gamelogic/Battle500.js"
import { vi, expect } from "vitest";


describe("game 500 move types", () => {
    it("Handles 500Move (draw from stacktop)", () => {
        // Use helper function to create game state
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up test conditions
        roomData.gameData.stack = [1, 2, 3]; // Mock stack with 3 cards
        roomData.gameData.players[socket1.id].hand = []; // Empty player hand initially

        // Mock `call500Move`
        call500Move(roomData, { moveType: "draw", drawKind: "stacktop" }, socket1.id, io, roomID);
        // Check if the player received a card from the stack
        expect(roomData.gameData.players[socket1.id].hand.length).toBe(1);
        expect(roomData.gameData.players[socket1.id].hand[0]).toBe(3); // Last card from stack should be drawn
        expect(roomData.gameData.stack.length).toBe(2); // Stack should now have 2 cards
    });

    it("Handles 500Move (draw from stacktop) with no stack", () => {
        // Use helper function to create game state
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up test conditions
        roomData.gameData.stack = []; // Mock stack with 3 cards
        roomData.gameData.players[socket1.id].hand = [1, 2, 3]; // Empty player hand initially

        // Mock `call500Move`
        call500Move(roomData, { moveType: "draw", drawKind: "stacktop" }, socket1.id, io, roomID);
        // Check if the player received a card from the stack
        expect(roomData.gameData.players[socket1.id].hand.length).toBe(3);
        expect(roomData.gameData.players[socket1.id].hand[0]).toBe(1); // Last card from stack should be drawn
        expect(roomData.gameData.stack.length).toBe(0); // Stack should now have 2 cards
    });

    it("Handles 500Move (draw from stack)", () => {
        // Use helper function to create game state
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up test conditions
        roomData.gameData.stack = [1, 2, 3, 4]; // Mock stack with 3 cards
        roomData.gameData.players[socket1.id].hand = [5, 6, 7]; // Empty player hand initially

        // Mock `call500Move`
        call500Move(roomData, { moveType: "draw", drawKind: "stack" }, socket1.id, io, roomID);
        // Check if the player received a card from the stack
        expect(roomData.gameData.players[socket1.id].needsToTrick).toBe(true);
        expect(roomData.gameData.players[socket1.id].hand.length).toBe(7);
        expect(roomData.gameData.players[socket1.id].hand).toEqual([5, 6, 7, 1, 2, 3, 4]);
        expect(roomData.gameData.stack.length).toBe(0); // Stack should now have 2 cards
    });

    it("Handles 500Move (draw from stack) with empty stack", () => {
        // Use helper function to create game state
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up test conditions
        roomData.gameData.stack = []; // Mock stack with 3 cards
        roomData.gameData.players[socket1.id].hand = [5, 6, 7]; // Empty player hand initially

        // Mock `call500Move`
        call500Move(roomData, { moveType: "draw", drawKind: "stack" }, socket1.id, io, roomID);
        // Check if the player received a card from the stack
        expect(roomData.gameData.players[socket1.id].needsToTrick).toBe(false);
        expect(roomData.gameData.players[socket1.id].hand.length).toBe(3);
        expect(roomData.gameData.players[socket1.id].hand).toEqual([5, 6, 7]);
        expect(roomData.gameData.stack.length).toBe(0); // Stack should now have 2 cards
    });

    it("Handles 500Move (draw from decktop)", () => {
        // Use helper function to create game state
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        const currentDeckSize = roomData.gameData.deck.length;
        // Mock `call500Move`
        call500Move(roomData, { moveType: "draw", drawKind: "decktop" }, socket1.id, io, roomID);
        // Check if the player received a card from the stack
        expect(roomData.gameData.players[socket1.id].hand.length).toBe(8);
        expect(roomData.gameData.deck.length).toBe(currentDeckSize - 1); // Stack should now have 2 cards
    });

    it("Handles 500Move (draw from decktop) with no stack", () => {
        // Use helper function to create game state
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        roomData.gameData.deck = []; // Mock stack with 3 cards

        // Mock `call500Move`
        call500Move(roomData, { moveType: "draw", drawKind: "decktop" }, socket1.id, io, roomID);
        // Check if the player received a card from the stack
        expect(roomData.gameData.players[socket1.id].hand.length).toBe(7);
        expect(roomData.gameData.deck.length).toBe(0); // Stack should now have 2 cards
    });

});

describe("game 500 functions", () => {

    it("generate unique lobby id", () => {
        expect(true).not.toBe(false);
    });

    it("Start 500 game", () => {
        const { roomData, socket1, io, roomID } = helperMake500Lobby();
        const gameData = roomData.gameData;

        // Assert that all players have exactly 7 cards
        for (const playerID in gameData.players) {
            expect(gameData.players[playerID].hand.length).toBe(7);
            expect(gameData.players[playerID].needsToTrick).toBe(false);
        }

        // Assert that the stack has at least 1 card
        expect(gameData.stack.length).toBeGreaterThanOrEqual(1);

        // Assert that the total number of cards (hands + stack + deck) is 52
        const totalCards =
            Object.values(gameData.players).reduce((sum, player) => sum + player.hand.length, 0) +
            gameData.stack.length +
            gameData.deck.length;

        expect(totalCards).toBe(52);
    });


    it("draw stack", () => {
        let roomData = helperMake500Lobby();



    });
});

function helperMake500Lobby() {
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
        id: "ghu45DxGsxgy5VCls8Zg",
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
    const lobby = createLobby(socket1, "testuser");
    const roomID = `/${lobby.id}`;

    joinLobby({ name: "testuser2", id: roomID }, roomID, socket2);
    joinLobby({ name: "testuser3", id: roomID }, roomID, socket3);
    let roomData = Rooms.get(roomID);

    // ✅ Pass the mocked io object
    start500Game(roomData, socket1, io, roomID);

    // ✅ Return an object containing all relevant data
    return { roomData, socket1, io, roomID };
}
