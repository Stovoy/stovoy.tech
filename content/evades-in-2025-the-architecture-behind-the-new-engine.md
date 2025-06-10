# Evades in 2025: The Architecture Behind the New Engine

Date: 2025-06-09

## A Brief History of Evades.io

On Friday, January 13th, 2017, there was a [Reddit post that changed my life](https://www.reddit.com/r/WebGames/comments/5npu91/evadesio_travel_down_a_single_lane_evading/). A user by the name of MiceLee posted on /r/WebGames about his new game - "Evades.io - Travel down a single lane, evading enemies, saving friends, upgrading stats & abilities, and unlocking new heroes."

I clicked. I played. A real-time multiplayer web game? My engineer brain immediately jumped to seeing how it worked under the hood. I opened the chrome console, inspected the websocket protocol, and I was impressed with what I found. Authoratitive server? Check. Input-only client messages? Check. It was solid. Unoptimized - the initial version only supported 25 players at a time, and with some notable lag at that scale - but solid nonetheless.

I was enamored immediately. The game hit all the right notes for me, and reminded me of one of my favorite Warcraft 3 custom maps, Run Kitty Run. I'd later find out it was directly inspired by that.

MiceLee was online at the same time I played. We spoke on the in-game chat, and by the end of the day we were working together on making Evades.io better and stronger.

I was 23 years old at the time, an engineer at Nextdoor looking for a fun project in my free time. Who would have thought that more than 8 years later, I'd still be working on it?

In that time, it's grown tremendously. From a single map and five heroes on a laggy 25-player server, Evades has evolved into a sprawling world: two physical servers hosting 8 shards of 65 players each, with over 50 regions and 31 unique heroes.

However, this post isn't about where Evades has been. I'll save that for another post. It's about where it’s going. 

## The Case For A New Engine

Evades is already not on the original engine, and hasn't been since 2018. Originally, Evades was a Node.js Socket.io server with MongoDB. I changed to my preferred tech stack at the time, asyncio Python. The client was still a 