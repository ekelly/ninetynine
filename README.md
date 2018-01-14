# Summary

This package is currently generating a binary, though that's primarily for testing/debug purposes right now. As it gets
more fully-featured, it will morph into a library that exposes methods that another package will call.

Essentially, this is the game engine for the trick taking card game 99, which has methods allowing actions such
as create a new game, play a card on an ongoing trick within the game, etc.

The actual API is detailed below.

# Game Flow


# Open Architectural questions?

* Should the number of players be hardcoded to 3?
* What information belongs to the player vs the round vs the game?
* Do we need to distinguish between "Game" (which is actually a Rubber) and the internal 'game's?
* What should the return values of most of these functions be? Right now I'm leaning towards they return a new Game,
  representing the new game state as a result of that action. Benefits: no mutation, making it easier to test.
  The component receiving the game can use that information to decide what to 'tell' the players.
* Do players need to be referenced by anything other than their number (index into player array)?

# API

* Create game (pass in optional settings?)
* Start game (Should be game.start()?)
* Submit bid for round
* ...