import { expect, it, describe, vi } from "vitest";
import { createLobby, joinLobby, Rooms } from "../Lobby";
import { start500Game, call500Move } from "../gamelogic/Battle500.js"




describe("game 500 move types", () => {
    it("Handles 500Move (draw from stacktop)", () => {
        // Use helper function to create game state
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up test conditions
        roomData.gameData.stack = [1, 2, 3]; // Mock stack with 3 cards
        roomData.gameData.players[socket1].hand = []; // Empty player hand initially

        // Mock `call500Move`
        call500Move(roomData, { moveType: "draw", drawKind: "stacktop" }, socket1, io, roomID);
        // Check if the player received a card from the stack
        expect(roomData.gameData.players[socket1].hand.length).toBe(1);
        expect(roomData.gameData.players[socket1].hand[0]).toBe(3); // Last card from stack should be drawn
        expect(roomData.gameData.stack.length).toBe(2); // Stack should now have 2 cards
    });

    it("Handles 500Move (draw from stacktop) with no stack", () => {
        // Use helper function to create game state
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up test conditions
        roomData.gameData.stack = []; // Mock stack with 3 cards
        roomData.gameData.players[socket1].hand = [1, 2, 3]; // Empty player hand initially

        // Mock `call500Move`
        call500Move(roomData, { moveType: "draw", drawKind: "stacktop" }, socket1, io, roomID);
        // Check if the player received a card from the stack
        expect(roomData.gameData.players[socket1].hand.length).toBe(3);
        expect(roomData.gameData.players[socket1].hand[0]).toBe(1); // Last card from stack should be drawn
        expect(roomData.gameData.stack.length).toBe(0); // Stack should now have 2 cards
    });

    it("Handles 500Move (draw from stack)", () => {
        // Use helper function to create game state
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up test conditions
        roomData.gameData.stack = [1, 2, 3, 4]; // Mock stack with 3 cards
        roomData.gameData.players[socket1].hand = [5, 6, 7]; // Empty player hand initially

        // Mock `call500Move`
        call500Move(roomData, { moveType: "draw", drawKind: "stack" }, socket1, io, roomID);
        // Check if the player received a card from the stack
        expect(roomData.gameData.players[socket1].needsToTrick).toBe(true);
        expect(roomData.gameData.players[socket1].hand.length).toBe(7);
        expect(roomData.gameData.players[socket1].hand).toEqual([5, 6, 7, 1, 2, 3, 4]);
        expect(roomData.gameData.stack.length).toBe(0); // Stack should now have 2 cards
    });

    it("Handles 500Move (draw from stack) with empty stack", () => {
        // Use helper function to create game state
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up test conditions
        roomData.gameData.stack = []; // Mock stack with 3 cards
        roomData.gameData.players[socket1].hand = [5, 6, 7]; // Empty player hand initially

        // Mock `call500Move`
        call500Move(roomData, { moveType: "draw", drawKind: "stack" }, socket1, io, roomID);
        // Check if the player received a card from the stack
        expect(roomData.gameData.players[socket1].needsToTrick).toBe(false);
        expect(roomData.gameData.players[socket1].hand.length).toBe(3);
        expect(roomData.gameData.players[socket1].hand).toEqual([5, 6, 7]);
        expect(roomData.gameData.stack.length).toBe(0); // Stack should now have 2 cards
    });

    it("Handles 500Move (draw from decktop)", () => {
        // Use helper function to create game state
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        const currentDeckSize = roomData.gameData.deck.length;
        // Mock `call500Move`
        call500Move(roomData, { moveType: "draw", drawKind: "decktop" }, socket1, io, roomID);
        // Check if the player received a card from the stack
        expect(roomData.gameData.players[socket1].hand.length).toBe(8);
        expect(roomData.gameData.deck.length).toBe(currentDeckSize - 1); // Stack should now have 2 cards
    });

    it("Handles 500Move (draw from decktop) with no stack", () => {
        // Use helper function to create game state
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        roomData.gameData.deck = []; // Mock stack with 3 cards

        // Mock `call500Move`
        call500Move(roomData, { moveType: "draw", drawKind: "decktop" }, socket1, io, roomID);
        // Check if the player received a card from the stack
        expect(roomData.gameData.players[socket1].hand.length).toBe(7);
        expect(roomData.gameData.deck.length).toBe(0); // Stack should now have 2 cards
    });

    it("Handles endTurnMove when player has no cards left", () => {
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up test conditions
        roomData.gameData.players[socket1].hand = []; // No cards left
        const nextPlayer = roomData.turn.next;

        // Call function
        call500Move(roomData, { moveType: "endTurn" }, socket1, io, roomID);

        // Check that `nextPlayerTurn()` was called
        expect(roomData.turn.current).toBe(nextPlayer);
        expect(roomData.turn.next).toBe("hjdk2VCls8Zg1fya"); //Socket3 id
    });

    it("Handles endTurnMove when player plays a valid card", () => {
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up test conditions
        roomData.gameData.players[socket1].hand = [5, 6, 7]; // Player's hand before playing
        roomData.gameData.stack = [1, 2, 3]; // Stack before playing
        const playedCard = 6; // The card player will play
        const nextPlayer = roomData.turn.next;

        // Call function
        call500Move(roomData, { moveType: "endTurn", cardToPlay: playedCard }, socket1, io, roomID);

        // Check that the card was removed from the hand
        expect(roomData.gameData.players[socket1].hand).toEqual([5, 7]);

        // Check that the card was added to the stack
        expect(roomData.gameData.stack).toEqual([1, 2, 3, 6]);

        // Check that turn switched to the next player
        expect(roomData.turn.current).toBe(nextPlayer);
        expect(roomData.turn.next).toBe("hjdk2VCls8Zg1fya"); // Next player (socket3)
    });

    it("Handles endTurnMove when player plays a valid card but needs to trick", () => {
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up test conditions
        roomData.gameData.players[socket1].needsToTrick = true;
        roomData.gameData.players[socket1].hand = [5, 6, 7]; // Player's hand before playing
        roomData.gameData.stack = [1, 2, 3]; // Stack before playing
        const playedCard = 6; // The card player will play
        const nextPlayer = roomData.turn.next;

        // Call function
        call500Move(roomData, { moveType: "endTurn", cardToPlay: playedCard }, socket1, io, roomID);

        // Check that the card was removed from the hand
        expect(roomData.gameData.players[socket1].hand).toEqual([5, 7]);
        expect(roomData.gameData.players[socket1].gamePoints).toBe(-50);
        expect(roomData.gameData.players[socket1].needsToTrick).toBeTruthy;

        // Check that the card was added to the stack
        expect(roomData.gameData.stack).toEqual([1, 2, 3, 6]);

        // Check that turn switched to the next player
        expect(roomData.turn.current).toBe(nextPlayer);
        expect(roomData.turn.next).toBe("hjdk2VCls8Zg1fya"); // Next player (socket3)
    });

    it("Handles endTurnMove when player tries to play an invalid card", () => {
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up test conditions
        roomData.gameData.players[socket1].hand = [5, 6, 7]; // Player's hand before playing
        roomData.gameData.stack = [1, 2, 3]; // Stack before playing
        const invalidCard = 10; // The player does NOT have this card
        const originalTurn = roomData.turn.current;
        const originalStack = [...roomData.gameData.stack];

        // Call function
        call500Move(roomData, { moveType: "endTurn", cardToPlay: invalidCard }, socket1, io, roomID);

        // ✅ Hand should remain unchanged
        expect(roomData.gameData.players[socket1].hand).toEqual([5, 6, 7]);

        // ✅ Stack should remain unchanged
        expect(roomData.gameData.stack).toEqual(originalStack);

        // ✅ Turn should NOT change
        expect(roomData.turn.current).toBe(originalTurn);
    });

    it("Handles endTurnMove when player plays their last card (Game Over)", () => {
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up test conditions
        roomData.gameData.players[socket1].hand = [8]; // Player has only 1 card left
        roomData.gameData.stack = [1, 2, 3]; // Stack before playing
        const playedCard = 8;

        // Call function
        call500Move(roomData, { moveType: "endTurn", cardToPlay: playedCard }, socket1, io, roomID);

        // ✅ Hand should now be empty
        expect(roomData.gameData.players[socket1].hand).toEqual([]);

        // ✅ Card should be added to stack
        expect(roomData.gameData.stack).toEqual([1, 2, 3, 8]);


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
        expect(roomData.turn.current).toBe(socket1)
        expect(roomData.turn.next).toBe("ghu45DxGsxgy5VCls8Zs")
    });

});

describe("game 500 play trick logic", () => {

    it("Does nothing when no card is selected", () => {
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Save initial state
        const initialHand = [...roomData.gameData.players[socket1].hand];
        const initialTricks = [...roomData.gameData.players[socket1].tricks];

        // Call function with no card
        call500Move(roomData, { moveType: "playTrick", cardToPlay: [] }, socket1, io, roomID);

        // ✅ Expect the game state to remain unchanged
        expect(roomData.gameData.players[socket1].hand).toEqual(initialHand);
        expect(roomData.gameData.players[socket1].tricks).toEqual(initialTricks);
    });

    it("Removes the card from hand and adds to tricks when a valid single trick is played", () => {
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up game state
        roomData.gameData.players[socket1].hand = [5, 6, 7]; // Player's hand
        roomData.gameData.players[socket1].tricks = [4]; // Empty tricks initially

        // Call function with a valid trick
        call500Move(roomData, { moveType: "playTrick", cardToPlay: [5] }, socket1, io, roomID);

        // ✅ Expect card to be removed from the hand
        expect(roomData.gameData.players[socket1].hand).toEqual([6, 7]);

        // ✅ Expect card to be added to tricks
        expect(roomData.gameData.players[socket1].tricks).toEqual([4, 5]);
    });

    it("Does not remove card from hand when trick is invalid (no adjacency)", () => {
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up game state
        roomData.gameData.players[socket1].hand = [5, 6, 7]; // Player hand
        roomData.gameData.players[socket1].tricks = [3]; // No tricks initially

        // Save initial hand
        const initialHand = [...roomData.gameData.players[socket1].hand];

        // Call function with a non-adjacent trick card
        call500Move(roomData, { moveType: "playTrick", cardToPlay: [5] }, socket1, io, roomID);

        // ✅ Expect the card to remain in hand
        expect(roomData.gameData.players[socket1].hand).toEqual(initialHand);

        // ✅ Expect tricks to remain unchanged
        expect(roomData.gameData.players[socket1].tricks).toEqual([3]);
    });

    it("Does not remove card from hand if the card is missing", () => {
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up game state
        roomData.gameData.players[socket1].hand = [6, 7]; // Player hand (5 is missing)
        roomData.gameData.players[socket1].tricks = [4];

        // Save initial hand
        const initialHand = [...roomData.gameData.players[socket1].hand];

        // Call function with a missing card
        call500Move(roomData, { moveType: "playTrick", cardToPlay: [5] }, socket1, io, roomID);

        // ✅ Expect hand to remain unchanged
        expect(roomData.gameData.players[socket1].hand).toEqual(initialHand);

        // ✅ Expect tricks to remain unchanged
        expect(roomData.gameData.players[socket1].tricks).toEqual([4]);
    });

});

describe("game 500 multi trick logic", () => {

    it("Does nothing when no cards are selected", () => {
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Save initial state
        const initialHand = [...roomData.gameData.players[socket1].hand];
        const initialTricks = [...roomData.gameData.players[socket1].tricks];

        // Call function with an empty trick
        call500Move(roomData, { moveType: "playTrick", cardToPlay: [] }, socket1, io, roomID);

        // ✅ Expect the game state to remain unchanged
        expect(roomData.gameData.players[socket1].hand).toEqual(initialHand);
        expect(roomData.gameData.players[socket1].tricks).toEqual(initialTricks);
    });

    it("Removes multiple adjacent cards from hand and adds them to tricks", () => {
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up game state
        roomData.gameData.players[socket1].hand = [5, 6, 7, 8, 9]; // Player's hand
        roomData.gameData.players[socket1].tricks = [4]; // Existing tricks
        const trickCards = [6, 7, 8]; // Valid multi-trick (adjacent)

        // Call function with a valid trick
        call500Move(roomData, { moveType: "playTrick", cardToPlay: trickCards }, socket1, io, roomID);

        // ✅ Expect cards to be removed from the hand
        expect(roomData.gameData.players[socket1].hand).toEqual([5, 9]);

        // ✅ Expect cards to be added to tricks
        expect(roomData.gameData.players[socket1].tricks).toEqual([4, 6, 7, 8]);
    });

    it("Does not remove cards when multi-trick is invalid (not adjacent)", () => {
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up game state
        roomData.gameData.players[socket1].hand = [5, 6, 7, 10]; // Player hand
        roomData.gameData.players[socket1].tricks = [4]; // Existing tricks
        const trickCards = [6, 7, 10]; // ❌ Not fully adjacent

        // Save initial hand and tricks
        const initialHand = [...roomData.gameData.players[socket1].hand];
        const initialTricks = [...roomData.gameData.players[socket1].tricks];

        // Call function with an invalid multi-trick
        call500Move(roomData, { moveType: "playTrick", cardToPlay: trickCards }, socket1, io, roomID);

        // ✅ Expect hand to remain unchanged
        expect(roomData.gameData.players[socket1].hand).toEqual(initialHand);

        // ✅ Expect tricks to remain unchanged
        expect(roomData.gameData.players[socket1].tricks).toEqual(initialTricks);
    });

    it("Does not remove cards when at least one is missing", () => {
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up game state
        roomData.gameData.players[socket1].hand = [6, 8]; // ❌ Missing 7
        roomData.gameData.players[socket1].tricks = [4]; // Existing tricks
        const trickCards = [6, 7, 8]; // Would be valid, but 7 is missing

        // Save initial hand and tricks
        const initialHand = [...roomData.gameData.players[socket1].hand];
        const initialTricks = [...roomData.gameData.players[socket1].tricks];

        // Call function with missing cards
        call500Move(roomData, { moveType: "playTrick", cardToPlay: trickCards }, socket1, io, roomID);

        // ✅ Expect hand to remain unchanged
        expect(roomData.gameData.players[socket1].hand).toEqual(initialHand);

        // ✅ Expect tricks to remain unchanged
        expect(roomData.gameData.players[socket1].tricks).toEqual(initialTricks);
    });

    it("Sets needsToTrick to false after a valid multi-trick", () => {
        const { roomData, socket1, io, roomID } = helperMake500Lobby();

        // Set up game state
        roomData.gameData.players[socket1].hand = [5, 6, 7, 8]; // Player's hand
        roomData.gameData.players[socket1].tricks = [4]; // Existing tricks
        roomData.gameData.players[socket1].needsToTrick = true; // ✅ Initially true
        const trickCards = [6, 7, 8]; // Valid multi-trick (adjacent)

        // Call function with a valid trick
        call500Move(roomData, { moveType: "playTrick", cardToPlay: trickCards }, socket1, io, roomID);

        // ✅ Expect cards to be removed from the hand
        expect(roomData.gameData.players[socket1].hand).toEqual([5]);

        // ✅ Expect cards to be added to tricks
        expect(roomData.gameData.players[socket1].tricks).toEqual([4, 6, 7, 8]);

        // ✅ Expect `needsToTrick` to be set to false
        expect(roomData.gameData.players[socket1].needsToTrick).toBe(false);
    });

});



function helperMake500Lobby() {
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
    start500Game(roomData, socket1.id, io, roomID);

    // ✅ Return an object containing all relevant data
    return { roomData, socket1: socket1.id, io, roomID };
}
