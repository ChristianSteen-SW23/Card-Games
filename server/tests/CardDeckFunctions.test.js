import { describe, it, expect } from "vitest";
import {
    drawDeck,
    dealFromDeckToHand,
    randomShuffle,
    isAdjacentCardsNoOverflow,
    isAdjacentCardsOverflow,
    isSameSuit,
    isCardAdjacentToStack,
    areCardsAdjacentSet,
} from "../lib/CardDeckFunctions.js"

describe("game 500 functions", () => {
    it("drawDeck correctly initializes deck", () => {
        let gameData = {};
        drawDeck(gameData);

        expect(gameData.deck.length).toBe(52);
        expect(new Set(gameData.deck).size).toBe(52); // Ensures all cards are unique
    });

    it("dealFromDeckToHand correctly moves cards", () => {
        let deck = Array.from({ length: 52 }, (_, i) => i + 1);
        let hand = [];
        dealFromDeckToHand(hand, deck, 5);

        expect(hand.length).toBe(5);
        expect(deck.length).toBe(47);
        expect(new Set([...hand, ...deck]).size).toBe(52); // Ensures all cards are unique
    });

    it("randomShuffle produces a shuffled deck", () => {
        let deck = Array.from({ length: 52 }, (_, i) => i);
        let shuffledDeck = [...deck];
        randomShuffle(shuffledDeck);

        expect(shuffledDeck.length).toBe(52);
        expect(new Set(shuffledDeck).size).toBe(52); // Ensures all cards are unique
        expect(shuffledDeck).not.toEqual(deck); // The deck should be shuffled
    });

    it("isAdjacentCardsNoOverflow correctly identifies adjacent cards", () => {
        expect(isAdjacentCardsNoOverflow(3, 4)).toBe(1); // Same suit, consecutive
        expect(isAdjacentCardsNoOverflow(5, 7)).toBe(0); // Not consecutive
        expect(isAdjacentCardsNoOverflow(12, 13)).toBe(0); // Different suits
    });

    it("isSameSuit correctly identifies cards in the same suit", () => {
        expect(isSameSuit(0, 12)).toBe(1); // Both are spades
        expect(isSameSuit(13, 25)).toBe(1); // Both are hearts
        expect(isSameSuit(2, 15)).toBe(0); // Different suits
    });

    it("isAdjacentCardsOverflow correctly identifies adjacent cards including overflow cases", () => {
        // ✅ Regular adjacent cards (same suit)
        expect(isAdjacentCardsOverflow(3, 4)).toBe(1); // Same suit, consecutive
        expect(isAdjacentCardsOverflow(9, 10)).toBe(1); // Same suit, consecutive
        expect(isAdjacentCardsOverflow(5, 7)).toBe(0); // Not consecutive

        // ✅ Overflow cases (Ace and King in the same suit)
        expect(isAdjacentCardsOverflow(0, 12)).toBe(1); // Ace and King of the same suit
        expect(isAdjacentCardsOverflow(12, 0)).toBe(1); // King and Ace of the same suit

        // ✅ Different suits (should always return 0)
        expect(isAdjacentCardsOverflow(2, 15)).toBe(0); // Different suits
        expect(isAdjacentCardsOverflow(10, 23)).toBe(0); // Different suits

        // ✅ Edge case: Same card (not adjacent)
        expect(isAdjacentCardsOverflow(7, 7)).toBe(0);
    });

    it("Returns 1 when a card is adjacent to at least one card in the stack", () => {
        const stack = [2, 3, 4, 10, 25]; // Example stack
        expect(isCardAdjacentToStack(1, stack)).toBe(1); // 1 is adjacent to 2
        expect(isCardAdjacentToStack(5, stack)).toBe(1); // 5 is adjacent to 4
        expect(isCardAdjacentToStack(9, stack)).toBe(1); // 9 is adjacent to 10
        expect(isCardAdjacentToStack(24, stack)).toBe(1); // 24 is adjacent to 25
    });

    it("Returns 0 when no card in the stack is adjacent", () => {
        const stack = [10, 20, 30, 40]; // Example stack with no adjacent numbers
        expect(isCardAdjacentToStack(5, stack)).toBe(0); // 5 is not adjacent to any
        expect(isCardAdjacentToStack(35, stack)).toBe(0); // 35 is not adjacent to any
    });

    it("Returns 0 when stack is empty", () => {
        expect(isCardAdjacentToStack(7, [])).toBe(0);
    });

    it("Handles edge cases with overflow adjacency", () => {
        const stack = [0, 12]; // Example stack with Ace (0) and King (12)
        expect(isCardAdjacentToStack(12, stack)).toBe(1); // King (12) is adjacent to Ace (0)
        expect(isCardAdjacentToStack(0, stack)).toBe(1); // Ace (0) is adjacent to King (12)
    });

});


describe("areCardsAdjacentSet function", () => {

    it("Returns 1 for an adjacent set in order", () => {
        expect(areCardsAdjacentSet([3, 4, 5])).toBe(1);  // ✅ Adjacent set
        expect(areCardsAdjacentSet([10, 11, 12])).toBe(1); // ✅ Adjacent set
    });

    it("Returns 1 for an adjacent set out of order", () => {
        expect(areCardsAdjacentSet([4, 6, 5])).toBe(1);  // ✅ Still adjacent after sorting
        expect(areCardsAdjacentSet([12, 11, 10])).toBe(1); // ✅ Still adjacent after sorting
    });

    it("Returns 1 for King-Ace wraparound adjacency", () => {
        expect(areCardsAdjacentSet([11, 12, 0])).toBe(1);  // ✅ King (12) and Ace (0) are adjacent
    });

    it("Returns 0 for non-adjacent sets", () => {
        expect(areCardsAdjacentSet([7, 9, 10])).toBe(0);  // ❌ 8 is missing
        expect(areCardsAdjacentSet([2, 5, 7])).toBe(0);   // ❌ Not consecutive
    });

    it("Returns 1 for a single card or empty array", () => {
        expect(areCardsAdjacentSet([2])).toBe(1);  // ✅ Single card is trivially adjacent
        expect(areCardsAdjacentSet([])).toBe(1);   // ✅ Empty list is trivially adjacent
    });

    it("Returns 1 for a single card or empty array", () => {
        expect(areCardsAdjacentSet([3, 6, 4, 7])).toBe(0);
        expect(areCardsAdjacentSet([1, 1, 1])).toBe(0);
    });

    it("Returns 1 for a single card or empty array123", () => {
        expect(areCardsAdjacentSet([1, 1, 1, 2])).toBe(0);
        expect(areCardsAdjacentSet([1, 2])).toBe(1);
    });

});