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

- Command history support: use the up- and down-arrow keys to navigate
  through your previous commands. The commands are remembered after you close
  the program.

- Choose whether to display accidentals using sharps or flats.
```
♪♪♪ F#dim
Playing Gb4 A4 C5 Gb5
♪♪♪ sharps
Notating accidentals using sharps.
♪♪♪ F#dim
Playing F#4 A4 C5 F#5
```

## Building from source

First, you must install several dependencies in order to build and run the
project.

### Ubuntu & Debian:
`sudo apt-get install git libasound2-dev libsqlite-dev`

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

