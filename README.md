# Harmony Explorer
Explore complex harmony without playing the chords. Type the name of any
chord and Harmony Explorer plays it for you!

## Features

- Hear any commonly used chord and see the notes the chord is composed of.
```
♪♪♪ C
Playing C4 E4 G4 C5
♪♪♪ Bdim
Playing B4 D5 F5 B5
♪♪♪ G7
Playing G4 B4 D5 F5
♪♪♪ Am
Playing A4 C5 E5 A5
```

- Use several commonly used names for each chord.
```
♪♪♪ Bbm
Playing Bb4 Db5 F5 Bb5
♪♪♪ Bb-
Playing Bb4 Db5 F5 Bb5
♪♪♪ Bb minor
Playing Bb4 Db5 F5 Bb5
```

- Choose whether to display accidentals using sharps or flats.
```
♪♪♪ F#dim
Playing Gb4 A4 C5 Gb5
♪♪♪ sharps
Notating accidentals using sharps.
♪♪♪ F#dim
Playing F#4 A4 C5 F#5
```
- Command history support: use the up- and down-arrow keys to navigate
  through your previous commands. The commands are remembered after you close
  the program.

- Press Enter to re-do the last command. Useful if you want to hear the chord
  that was last played without having to type it again.

## Building from source

First, you must install several dependencies in order to build and run the
project.

### Ubuntu & Debian:
`sudo apt-get install git libasound2-dev`

You will also need to install the Rust compiler. Instructions for doing so
[can be found here.](https://www.rust-lang.org/tools/install)

You may obtain the code using the following command:
```
git clone https://github.com/tiniuclx/harmony-explorer.git
```

Then, build & run it using Cargo, the Rust package manager & build system.
This step will take a few minutes. Once it is complete, Harmony Explorer will
start, you will see the prompt (three quavers, like in the example) and you
can start typing your commands.

```
cd harmony-explorer
cargo run
```

### Windows

For Windows, you will have to build `portaudio` manually. For this, you will
need CMake and Visual Studio Build Tools. Once you've downloaded the
`portaudio` source, configure it to build using Visual Studio. You can then
build it by running the folliwing inside a Windows developer shell:

```MSBuild.exe .\portaudio.sln /property:Configuration=Release```

Once the command finishes, the output can be found inside the `Release`
folder within your build directory.

You will also need to tell the linker where to find the library. There should
be a way to do this more cleanly, but I did not find it yet. Something like
[this
answer](https://stackoverflow.com/questions/43826572/where-should-i-place-a-static-library-so-i-can-link-it-with-a-rust-program)
should do the trick, but this didn't really work, or I did it incorrectly.


Copy the generated files into the `/LIBPATH` passed to the link command. In my case,
this is:
```path
C:\Users\tiniu\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib
```
The linker expects to find `portaudio.lib`, so you will also have to rename
all the files to remove "_x64": `portaudio_x64.lib` shall become `portaudio.lib` and so on. 