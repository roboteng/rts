# RTS

This project is primarily for learning and practicing things like:

- Continuous Integration / Continuous Delivery
- Test Driven Development and Behavior Driven Development
- Domain Driven Design
- Writing User Stories and Acceptance Criteria

A lot of my thoughts on the design and comparing options are in [`Notes.md`](Notes.md).

I want to make an RTS similar to [Age of Mythology](https://en.wikipedia.org/wiki/Age_of_Mythology).
I'm still not sure close the game play will comg to AoM, but it will probably be my default for gameplay choices.

## Parts of the Project

One of the reasons I wanted to work on a multiplayer real time strategy game, is that there are a lot of moving parts to worry about.
Addtionally, many of these parts have different trade-offs.
Things like the game server are pretty well defined, while the rules for the game are pretty loose, and will likely require more iteration.

### Game Server

This runs multplayer games, and manager connections between players

### Game Client

This is generally the player's interface.
It will be capable of running single player games, and connecting to a server for mulitplayer games.

### CI

I'm trying out [Dagger](https://dagger.io) for CI.
It currently runs locally, but I haven't gotten to the point of running in GitHub Actions.
It doesn't do any deployment, just CI.
