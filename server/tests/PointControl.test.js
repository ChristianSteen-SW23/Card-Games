import { describe, it, expect } from "vitest";
import {
    countPointScore
} from "../lib/PointCounter.js"

describe("card point tests", () => {
    it("count full suit in 500", () => {
        const ace = 15, picture = 10, number = 5;
        const cards = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        expect(countPointScore(cards, picture, ace, number)).toBe(90);
    });
});