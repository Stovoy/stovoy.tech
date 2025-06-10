# Evades in 2025: The Architecture Behind the New Engine

Date: 2025-06-09

## A Brief History of Evades.io

On Friday, January 13th, 2017, there was a [Reddit post that changed my life](https://www.reddit.com/r/WebGames/comments/5npu91/evadesio_travel_down_a_single_lane_evading/). A user by the name of MiceLee posted on /r/WebGames about his new game - "[Evades.io](https://evades.io/) - Travel down a single lane, evading enemies, saving friends, upgrading stats & abilities, and unlocking new heroes."

I clicked. I played. A real-time multiplayer web game? My engineer brain immediately jumped to seeing how it worked under the hood. I opened the chrome console, inspected the websocket protocol, and I was impressed with what I found. Authoritative server? Check. Input-only client messages? Check. It was solid. Unoptimized - the initial version only supported 25 players at a time, and with some notable lag at that scale - but solid nonetheless.

I was enamored immediately. The game hit all the right notes for me, and reminded me of one of my favorite Warcraft 3 custom maps, Run Kitty Run. I'd later find out it was directly inspired by that.

MiceLee was online at the same time I played. We spoke on the in-game chat, and by the end of the day we were working together on making Evades.io better and stronger.

I was 23, an engineer at Nextdoor, just looking for a new side project. Who would've thought that more than 8 years later - now doing AI safety at OpenAI - I'd still be working on Evades?

In that time, it's grown tremendously. From a single map and five heroes on a laggy 25-player server with a handful of enemy types, Evades has evolved into a sprawling world: two physical servers hosting 8 shards of 65 players each, with over 50 regions, 31 unique heroes, and over a hundred kinds of enemies.

However, this post isn't about where Evades has been. I'll save that for another post. It's about where it's going. 

## The Case For A New Engine

Not too surprisingly, Evades is already not on the original engine, and hasn't been since 2018. Originally, Evades was a Node.js server using Socket.io and MongoDB. We changed to my preferred tech stack at the time, asyncio Python with Postgres, and protobufs over websockets for the transport layer. The client in both cases was a "dumb" HTML5 Canvas layer that simply rendered what the server said.

This held up for a while, especially after we rewrote core components like [collision detection in Rust](https://github.com/spacebrook/bolt).

As it grew in complexity, however, it imposed some deep, systemic limitations that couldn't be patched over. Having experienced firsthand these limitations, it became clear a complete overhaul was needed. Let's explore exactly why:

### No Client-Side Prediction or Lag Compensation
The original architecture was fully server-authoritative. The client simply rendered what the server told it, with no local simulation. That made the game simple and cheat-resistant, but it also caused input lag, and requires a very stable internet connection to play smoothly.

We attempted to implement prediction and reconciliation, but failed because there was no shared code for client simulation, and no clean way to replay server corrections.

The new engine is being built around deterministic simulation, shared logic, and rollback-compatible architecture from the ground up.

### Lack of Graphical Fidelity
Even simple visuals were a chore in the old engine. The rendering layer was hand-rolled and imperative: no scene graph, no animation curves, no blending, no shaders. Features like screen shake, tweening, fancy lighting, or even basic UI transitions were skipped entirely.

The new engine gives us access to a proper rendering pipeline, animation tools, and shaders, meaning we can finally make Evades look and feel like a modern game.

### No Advanced Physics
Anything other than circles and rectangles is not supported in our custom, extremely simple physics engine.

Moving forward, we'll be using the [Rapier 2d physics engine](https://rapier.rs/), which is much more powerful.

### Map Editing was tedious
Without any dedicated tooling, making or modifying a map was a painful, external process involving [huge YAML files](https://drive.google.com/drive/folders/1qpVSpEc2C4z0fytWpRS38Vzvx0keXt8a). There was no way for players to create their own content, and there was no editor tooling. The only people who could really contribute were myself and the handful of junior developers, since they could run the game and test their map.

We want everyone to be able to create maps, abilities, heroes, enemies, and even new game modes entirely. We want to empower the community, and make it beginner-friendly.

The new engine will support a fully in-game editor, and all official content will use the same pipeline modders use.

### Unmaintainable Legacy Code
The old engine was built over years of iteration and experimentation. Every core system was custom and deeply coupled. player.py ballooned to over 4,000 lines. Enemy logic became fragmented and duplicated. Performance suffered, and debugging was a nightmare.

Starting from scratch helps clean this up. Now, we've redesigned everything around [ECS](https://www.richardlord.net/blog/ecs/what-is-an-entity-framework), enabling modular, reusable systems.

## The New Engine

We're moving to [Godot](https://github.com/godotengine/godot)! I've experimented with Unity, Unreal Engine, and even Bevy ECS over the years. Unity had too much drama. Unreal was too heavyweight. Bevy required extensive custom tooling. Godot, however, is the perfect fit for Evades. It's lightweight, fast to iterate on, great for 2D, supports web and multi-platform exports, and has native extension support.

Though, we're not using Godot the usual way.

To support full user modding - maps, abilities, heroes, even entire game modes - we've embedded [Luau](https://luau.org/) into Godot. Nearly all gameplay and rendering logic runs through Luau, giving us sandboxed scripting with performance and safety. Under the hood, the Luau runtime is entirely written in Rust, and we batch communication across the Rust–Luau boundary every tick: physics and network events go into Rust, and rendering data flows out.

This setup lets us do something powerful: embed the entire game logic directly inside map files, fully sandboxed and user-editable. Players can create anything. It's highly scalable, and moddable, for those brave enough. There'll also be a beginner-friendly map editor for recombining existing components, no code required.

On top of that, the entire core is rebuilt around a clean, performant ECS architecture. Systems are modular, components are reusable, and legacy code bloat is gone.

One of the biggest wins of this new architecture is shared simulation. Because Godot runs on both the client and server, we finally have a unified environment for game logic. That unlocks features we've wanted for years: accurate rollback networking, real client-side prediction, and even singleplayer support. Both ends of the connection finally speak the same language.

## What's Next?

We're still working out the rough edges, but it's finally starting to feel real.

An early singleplayer demo is just weeks away. It'll be offline-only at first, focused on finding bugs, polish issues, and weird edge cases, but it's your first hands-on look at the new engine.

Multiplayer stress tests will follow. The map editor comes after launch.

Evades has always been about dodging, adapting, and surviving. Now, the engine does the same.

But this is more than a new engine. It's a new foundation - one that turns Evades into a platform.

Create your own worlds. Define your own rules.

The next era of Evades isn't just playable. It's buildable.

And it's almost here.

Want to be among the first to test the new Evades engine or create something remarkable of your own? [Join our Discord community](https://discord.gg/s5aunm6).

The future of Evades belongs to all of us.