# Catacomb II Rust port

![Screenshot](/misc/readme_screenshot.png?raw=true)

Catacomb II-64k is a experimental project in porting a moderately complex project, first from C to unsafe Rust, then to (fully) safe Rust.

~~A detailed article is going to be published [on my blog](https://saveriomiroddi.github.io/) in September 2022.~~ I've put my blog on hold, so I will likely not write any article.

The [game](https://en.wikipedia.org/wiki/Catacomb_(video_game)) has been written by the id Software team (J.Carmack, J.Romero, A.Carmack) before they founded their own company.

## The project

The idea is the same in spirit as [Rust out your C](https://www.youtube.com/watch?v=SKGVItFlK3w), but the port is significantly more complex, due to a few factors:

- the codebase has not been written following best practices (e.g. it uses many globals);
- the call graph is significantly more intricate;
- a C library is used (SDL2);
- the program is hard to continuously test, because of its nature and design.

The codebase used as starting point is [Catacomb SDL](https://github.com/Blzut3/CatacombSDL), a port that removed the assembler, and implemented SDL calls (and graphic/sound emulation), in order to make the project compatible with modern O/Ss.

The conversion from C to unsafe Rust has been performed via the [C2Rust transpiler](https://github.com/immunant/c2rust), which produced a running but unsafe Rust project (essentially, a "C project with Rust syntax").

I've then performed the conversion from unsafe to safe Rust, which took a very considerable effort, due to several factors:

- the C semantics differing from the (safe) Rust ones in many aspects (everything related to memory, essentially);
- the high amount of boilerplate produced by C2Rust, with no refactoring tools (currently) available.

I've performed the port in many small, incremental steps, leaving the program stable at (almost) at each step. An aiding factor has been that it's possible to use the C SDL library *and* the Rust SDL library at the same time; if this wasn't possible, porting the SDL calls would have required a large, monolithic, change.

Unsurprisingly, porting to Rust brought to light several bugs in the C codebase.

Several conclusions can be drawn, most importantly:

- it is generally possible to port a project from C to safe Rust, in an incremental fashion (note that I'm in no way implying that this is something that should be done);
- in particular, ports based on the SDL allow coexistence of calls to both the C library and the Rust one;
- porting to safe Rust likely brings to light bugs of the original C projects;
- porting from unsafe to safe Rust is currently a grueling task, due to lack of refactoring tooling; refactoring the boilerplate itself is not a large part of the project, but every small change counts, as this type of port requires a very high amount of precision.

## Project objective and tools

It's not an objective of the project, to redesign the codebase in order to make it clean; such operation is a generic software engineering task that is not inherent in this type of project.

By contrast, some transformations are (considered) inherent, for example:

- redesigning global data to be safely accessed;
- making data structures thread-safe;
- implementing de/serialization from in-memory data structures (types) from/disk (which in C are very cheap but very unsafe);
- and so on.

I developer a small serialization library, [Serdine](https://github.com/64kramsystem/serdine), in order to de/serialize data structures, while maintaining the same on-disk, binary, format (essentially, memcpy'd packed structs).

## Project status

The project itself is completed (the codebase is 100% safe), although I'm still performing further, minor, cleanups.

## Running the game

While the project itself has a FOSS license, the assets are proprietary.

In order to run, the asset (`*.CA2`) files must be copied from a licensed game; a pack with all the Catacomb games currently costs [just 4$ on GOG](https://www.gog.com/de/game/catacombs_pack)!
