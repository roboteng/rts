# RTS

This project is primarily for learning and practicing things like:

- Continuous Integration / Continuous Delivery
- Test Driven Development and Behavior Driven Development
- Domain Driven Design

A lot of my thoughts on the design and comparing options are in [`Notes.md`](Notes.md).

## Parts of the Project

One of the reasons I wanted to work on a multiplayer real time strategy game, is that there are a lot of moving parts to worry about.
Addtionally, many of these parts have different trade-offs.
Things like the game server are pretty well defined, while the rules for the game are pretty loose, and will likely require more iteration.

### Game Server

This runs multplayer games, and manager connections between players

### Game Client

This is generally the player's interface.
It will be capable of running single player games, and connecting to a server for mulitplayer games.
