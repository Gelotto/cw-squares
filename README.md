# CosmWasm Superbowl Square Smart Contract

## API

### Create Game (Contract Instantiation)

To create a game, simply instantiate the contract. The contract is initialized with a `has_started` flag set to `false`. Until the game creator executes the `start_game` function, they can register new players. Moreover, players can continue buying squares until the game creator starts the game. See `InstantiateMsg` ins `msg.rs` for details on expected init args.

### Start Game

```rust
fn start_game();
```

The game creator can call `start_game` to close the game to further sales and signal that the first quarter has begun. At this point, existing players are locked in.

### Register Player

```rust
fn register_player(
    wallet: &Addr,
    name: Option<String>,
    color: Option<String>
);
```

The game creator can use this function to register additional players, provided that the game hasn't started. Note that the contract can be optionally initialized with players. This function doesn't need to be used if the game is "public", which can be determined by inspecting its `is_public` flag.

### Buy Squares

```rust
fn buy_squares(
    // NOTE: type GridCoordinates = (u8, u8);
    coordinates: Vec<GridCoordinates>,
    player_name: Option<String>,
    player_color: Option<String>,
);
```

A player who has been registered with the contract can place an order to buy one or more squares by executing `buy_cells`. This is only possible before the game has started via execution of the `start_game` function.

Note that the `player_name` and `player_color` optional args only come into play for public games. In this case, the purchasing wallet can provide values for their display name and color to use when instantiating their on-chain player's state.

### Choose Winner

```rust
fn choose_winner(
    // NOTE: type GridCoordinates = (u8, u8);
    winner: GridCoordinates
);
```

Once a game quarter has ended, the game creator may call this function to select the winning square. When this happens, the reward amount for each player in the square is calculated and sent. If no one bought the winning square, then this quarter's prize money rolls over into the remaining rounds, respecting the existing split.

### Claim Refund

```rust
fn claim_refund();
```

If the final round ends and no player has bought the winning square, the game goes into a refundable state, in which each player can claim their remaining funds by executing this function.
