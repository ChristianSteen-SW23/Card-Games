# Data interfaces

## From frontend

### Start game
Start game using
`startGame`\
Data: `500`

### Make move
Make move using 
`500Move`\
With data:
```JSON
{
    moveType: "draw",
    drawKind: "stacktop" 
}
```
```JSON
{
    moveType: "draw",
    drawKind: "stack" 
}
```
```JSON
{
    moveType: "draw",
    drawKind: "decktop" 
}
```
```JSON
{
    moveType: "endTurn",
    cardToPlay: cardNum from hand, 
}
```
```JSON
{
    moveType: "playTrick",
    cardsToPlay: [indexs from hand] 
}
```
```JSON
{
    moveType: "playAgain"
}
```

## From backend
### Send player information

Send over information `gameInformation`
```JSON
{
  "playersInfo": [
    {
      "name": "123",
      "id": "iZARbfMKRIVCXskbAAAF",
      "handSize": 7,
      "tricks": []
    },
    {
      "name": "77",
      "id": "qc7ePPHrFh9fpsGEAAAD",
      "handSize": 7,
      "tricks": []
    }
  ],
  "turn": {
    "current": "iZARbfMKRIVCXskbAAAF",
    "next": "qc7ePPHrFh9fpsGEAAAD"
  },
  "stack": 7, // The topcard. -1 if no card
  "stackSize": 1,
  "deckSize": 10,
  "gameStep": "draw"/"trick"
} 
```

Send over hand information `handInformation`:
```JSON
{
  hand:[]
}
```

### Start game
Send over information `startedGame500` and `newRound`
Same data as for gameInformation
```JSON
{
  "hand": [
    32,
    21,
    1,
    45,
    49,
    22,
    13
  ],
  "playersInfo": [
    {
      "name": "123",
      "id": "iZARbfMKRIVCXskbAAAF",
      "handSize": 7,
      "tricks": []
    },
    {
      "name": "77",
      "id": "qc7ePPHrFh9fpsGEAAAD",
      "handSize": 7,
      "tricks": []
    }
  ],
  "turn": {
    "current": "iZARbfMKRIVCXskbAAAF",
    "next": "qc7ePPHrFh9fpsGEAAAD"
  },
  "stack": 7,
  "stackSize": 1
} 
```

### Game over
Send over `gameEnded`
```JSON
{
    contineAllowed: false/true // Shall be false if one player have over 500
    playerPoints: [ { name, totalScore, roundScore } ]
}
```

# Backend Data
````JSON
Map(1) {
  '/1' => {
    players: Map(2) {
      'inB93DTk-TxY9b3CAAAB' => [Object],
      'sZ0DMe-Cgu7V2Ze8AAAD' => [Object]
    },
    turn: { current: 'inB93DTk-TxY9b3CAAAB', next: 'sZ0DMe-Cgu7V2Ze8AAAD' },
    gameData: {},
    startingPlayerID: 'inB93DTk-TxY9b3CAAAB',
    gameStarted: true
  }
}
````

````JSON
gameData: {
    deck: [cards],
    stack: [cards],
    players: {
        ID: {
            needsToTrick: false, // Used if entire stack have been drawn
            gamePoints: Num,
            totalScore: Num,
            hand:[cards],
            tricks: [],

        }
    },

}
````

# Frontend Data



# Plan For implementaion

## Backend
0. Setup testing for backend ✅
1. Game start setup ✅
2. Setup diff. socket connections to the right functions ✅
3. Implement logik for draw ✅
4. Implement end turn logik ✅
5. Implement trick logik ✅\
Frontend ends to follow up ✅
6. Winlogik ✅

## Frontend


