# Data interfaces

## From frontend

### Start game
Start game using
`startGame`\
Data: `Planning Poker`

### Make move
Make move using 
`PlanningPokerMove`\
With data:
```JSON
{
    moveType: "choice",
    value: [1..9] 
}
```
```JSON
{
    moveType: "newRound",
}
```

## From backend
### Send player information

Send over information `gameInformation` and `startedGamePlanningPoker`
```JSON
{
  "playersInfo": [
    {
      "name": "123",
      "id": "iZARbfMKRIVCXskbAAAF",
      "value": [0..9],
      "ready": true/false
    },
    {
      "name": "77",
      "id": "qc7ePPHrFh9fpsGEAAAD",
      "value": [0..9],
      "ready": true/false
    }
  ],
  "mustNewRound": true/false,
} 
```


# Backend Data
````JSON
Map(1) {
  '/1' => {
    players: Map(2) {
      'inB93DTk-TxY9b3CAAAB' => {
      "value": [0..9] | null,
      "ready": true/false
      },
      'sZ0DMe-Cgu7V2Ze8AAAD' => [Object]
    },
    "MustNewRound": true/false,
    "gameStarted": true,
    "SM": "inB93DTk-TxY9b3CAAAB" // Is the lobby host
  }
}
````


