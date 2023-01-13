# CosmWasm Superbowl Square Smart Contract

## API

### Create Game (Contract Instantiation)

To create a game, simply instantiate the contract. The contract is initialized with a `has_started` flag set to `false`. Until the game creator executes the `start_game` function, they can register new players. Moreover, players can continue buying squares until the game creator starts the game.

### Start Game

Start the game. This means that new players cannot be added and no new squares can be sold.

### Add Player

The creator of the game can register new wallets as players until the game has started. They can start the game by executing the `start_game` function.

### Buy Cells

A player who has been registered with the contract can place an order to buy one or more squares by executing `buy_cells`. This is only possible before the game has started via execution of the `start_game` function.

### Resolve Winner

Once a game quarter has ended, the game creator may call this function to select the winning square. When this happens, the reward amount for each player in the square is calculated and sent. If no one bought the winning square, then this quarter's prize money rolls over into the remaining rounds, respecting the existing split.
