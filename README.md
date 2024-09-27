# CD GameDev
## The game plan
The way we'll go about developing the game, is through prototyping and
iterations. We want you to also be part of the decision process, so as you start
contributing to the codebase, we'd love to hear your ideas on the next features
we should implement.

To keep things simple, we have opted to go with the following parameters:
- Game will be 2D pixel art 
- The view will be a side view

As for genre, we are currently aiming for a metroidvania type game, but we might
mix and match elements from multiple genres as we progress.

## Tech stack
- Language:  Rust
- Framework: Bevy (ECS)

## What you'll learn
- How to use Rust
- How to use the Bevy framework
- Game design
- How to work in an ECS architecture
- Understand the different systems required to make a game

## Prerequisites 
- [Install Rust](https://www.rust-lang.org/tools/install)
- [Install Bevy dependencies](https://bevyengine.org/learn/quick-start/getting-started/setup/#installing-os-dependencies)

## How to run the game
1. Clone this repository
2. Move into the project directory
3. Run the command `cargo run game`

## Keeping compilation times short
The repo is already setup to tell the Rust compiler to use all the optimization possibles, but this mostly helps during compilation between iterations. The first time you compile, or if there are new crates added to the project, you might have to compile everything again, or compile a big part of it, which can be lengthy. But usually, compilation between iterations should only take a few seconds on average.

A way to keep compilation times short between iterations (or just get them started ahead of time), is to use a tool like [`cargo watch`](https://github.com/watchexec/cargo-watch) or [`bacon`](https://github.com/Canop/bacon). These tools allows you to set up a build/check/test/run command whenever a file is detected to have changed. This will ensure that the compilation process is completed as early as possible.

## Before contributing
Please read the [contribution guide](./CONTRIBUTING.md).


# Recommended Resources
## Rust
- [The Rust book:](https://doc.rust-lang.org/stable/book/) Hands-down the best resource to get started with Rust (I recommend completing through chapter 6 in the very least).
- [Data modeling in Rust:](https://www.youtube.com/watch?v=z-0-bbc80JM) Gives a good overview of structs and enums, the main building blocks for data modeling in Rust.
- [Rust by example:](https://doc.rust-lang.org/stable/rust-by-example/) Lots of great basic examples to use as reference.

## Bevy 
- [What is an ECS:](https://www.youtube.com/watch?v=AirfWcVOEHw) Brief overview of what is an ECS.
- [Quick start guide:](https://bevyengine.org/learn/quick-start/introduction/) The official quick start guide.
- [Zymartu Games' Bevy tutorial series](https://youtube.com/playlist?list=PL2wAo2qwCxGDp9fzBOTy_kpUTSwM1iWWd&si=4832EhayZTvKbysX) A great tutorial series that I highly recommend to get started with Bevy.

> [!NOTE]
> The following resource is a great starting point for any questions you might have on how to perform a specific task, or want to learn about a specific bevy feature:\
> [taintedcoders.com's guide to Bevy](https://taintedcoders.com/) 


# Additional Resources

<details>

<summary>Bevy and ECS</summary>

### Official resources
- [The Bevy GitHub repo:](https://github.com/bevyengine/bevy) As you get more comfortable with Rust and Bevy, I highly recommend looking at their open issues, which often have some great beginner friendly ones. A great way to contribute to open-source software.
- [The Bevy Discord:](https://discord.gg/bevy) A great community if you need any help on any Bevy related topics. Also has some great examples and showcase of what Bevy can do in their Showcase channel.
- [Bevy's official examples:](https://bevyengine.org/examples/) One-stop shop for bevy code examples.

### Unofficial resources
- [The Unofficial Bevy Cheat Book:](https://bevy-cheatbook.github.io/) Another great resource for learning Bevy.
- [A collection of other Bevy resources:](https://github.com/Anshorei/awesome-bevy) A compilation of Bevy related resources.
- [Good breakdown of Bevy ECS](https://www.youtube.com/watch?v=ocacUsyJXpg) A closer look to how Bevy works.
- [How Overwatch uses ECS](https://youtu.be/W3aieHjyNvw?si=2q0gQK82n_4zpCtf) How the popular game Overwatch uses ECS to power their game. Great watch!
- [Chris Biscardi:](https://www.youtube.com/@chrisbiscardi) Some tutorials, but nowadays mostly news about Bevy.
- [PhaestusFox:](https://www.youtube.com/@PhaestusFox) Tutorials and concepts explainers.

### Games made in Bevy
- [Tiny Glade](https://www.youtube.com/watch?v=QAUSBxxgIbQ) A beautiful, procedural sandbox game made in Bevy (although they use a custom renderer).
- [Games made in Bevy on itch.io](https://itch.io/games/tag-bevy) A collection of games made in Bevy.

</details>

<details>

<summary>Learning Rust</summary>

### Unofficial resources
- [Rustlings: Learn Rust through small exercises:](https://github.com/rust-lang/rustlings) A collection of interactive exercises to familiarize yourself with Rust concepts.
- [Why rust?:](https://www.youtube.com/watch?v=0rJ94rbdteE) An overview of what makes Rust different from other languages.
- [Learn Rust in 10 minutes](https://www.youtube.com/watch?v=br3GIIQeefY) A fast paced introduction to a lot of Rust concepts.
- [Let's Get Rusty:](https://www.youtube.com/@letsgetrusty) Great channel for learning Rust concepts.
- [Jeremy Chone:](https://www.youtube.com/@JeremyChone) Great channel for learning Rust concepts.
- [Jon Gjengset:](https://www.youtube.com/@jonhoo/videos) Great channel that does deep dives into different Rust crates.
- [No Boilerplate:](https://www.youtube.com/c/NoBoilerplate) Great channel for understanding the mindset needed to program Rust.

</details>
