CatacombSDL
===========

CatacombSDL is a source port for The Catacomb (also known as Catacomb II). This
port compiles for Windows, OS X, and Linux for 32 and 64-bit X86. The only
dependency is SDL 2.0 and CMake. Supported compilers are GCC, Clang, and MSVC.

It is released under the GNU GPLv2. Please see COPYING for license details.

The source port is a drop in replacement for the DOS binary, and maintains full
compatibility with the DOS game files (demos, saves, and configuration). This
brings about a few limitations most obvious in control configuration. For
example, keyboard binds must correspond to a DOS scan code and joystick support
is limited to 2-axis and 2-buttons.

Also note on case sensitive file systems, this port currently requires that the
game data files (*.CA2) be in all upper case.
