# Gitar

A command-line tool for programmer-guitarists.

*Git*ar? Do you *git* it?

## Download and installation

Note: you will need Rust on your machine in order to use this tool. If you don't, you can visit https://rustup.rs/ to install it.

```console
$ git clone https://github.com/thfm/gitar.git
$ cd gitar
$ cargo install --path . --force
```

## A note regarding note input

Functionality for note input is common between commands; thus, it is covered here once for convenience.

You only need to supply a name when inputting notes, but can optionally specify an octave number as well (for situations where it matters):

* note names must be one of the following (case-sensitive): `C`, `Db`, `D`, `Eb`, `E`, `F`, `Gb`, `G`, `Ab`, `A`, `Bb`, `B`

* and octave numbers are simply any positive whole number

Examples of valid note inputs are `C`, `F`, `Ab0`, and `B10`.

Examples of *invalid* note inputs are `D#3` and `A-10`.

## Command usages

### `find`

The `find` command is for finding the different fretboard locations of a note. For example,

```console
$ gitar find E4
```

produces a fretboard diagram that looks like this:

```console
5 occurences:
-----∗ 0
││││││ 1
││││││ 2
││││││ 3
││││││ 4
││││∗│ 5
││││││ 6
││││││ 7
││││││ 8
│││∗││ 9
││││││ 10
││││││ 11
││││││ 12
││││││ 13
││∗│││ 14
││││││ 15
││││││ 16
││││││ 17
││││││ 18
│∗││││ 19
```

Each fretboard location is  shown as a `∗`, with the numbers on the side indicating the fret number.

If there are no occurences of the given note, the output will say as such:

```console
$ gitar find Bb20
No occurences.
```

#### Adjusting the `tuning`

What if you want to find the locations of a certain note in a non-standard tuning? Simply specify open string note values for the `tuning` option:

```console
$ gitar find D2 --tuning D2 A2 D3 G3 B3 E4
1 occurence:
∗----- 0
```

You don't actually have to put in six values; in fact, you can type any number of notes, and the program will construct a guitar with that many strings. This means that you can use `find` for bass guitars as well, as is demonstrated below:

```console
$ gitar find Gb1 --tuning E1 A1 D2 G2
1 occurence:
∗│││ 2
```

#### Adding a `capo`

You can add a capo by specifying the fret that you want it to be placed on after the `capo` option:

```console:
$ gitar find F2 --capo 1
∗----- 0
```

Note that the fret numbering will change to treat the capo as if it were the nut.

#### Changing the number of `frets`

Finally, you may encounter a scenario where the output of `find` is different from what you expect. Take the following, for example:

```console
$ gitar find E6
```

This outputs `No occurences`, which is true for *most* guitars, except for those with 24 or more frets (or a [different tuning](#adjusting-the-tuning)). You can use the `frets` option if this exception applies:

```console
$ gitar find E6 --frets 24
1 occurence:
│││││∗ 24
```

This option is also useful for when you want to simulate a guitar with less than the default number of frets (which, in this program, is 21):

```console
$ gitar find F4 -f 18
4 occurences:
│││││∗ 1
││││││ 2
││││││ 3
││││││ 4
││││││ 5
││││∗│ 6
││││││ 7
││││││ 8
││││││ 9
│││∗││ 10
││││││ 11
││││││ 12
││││││ 13
││││││ 14
││∗│││ 15
```

### `notes`

The `notes` command simply outputs the notes in a given key. You use it like this:

```console
$ gitar notes C
C D E F G A B
```

The default mode is ionian, but you can easily change that using the `mode` option:

```console
$ gitar notes Eb --mode Aeolian
Eb F Gb Ab Bb B Db
```

### `key`

`key` is essentially the opposite of `notes`: you give it a list of notes, and it outputs possible keys (called 'candidates') to which they belong:

```console
$ gitar key C D E F A
14 candidates:
C Ionian (C D E F G A B)
C Lydian (C D E F G A Bb)
D Dorian (D E F G A B C)
D Aeolian (D E F G A Bb C)
E Phrygian (E F G A B C D)
E Locrian (E F G A Bb C D)
F Ionian (F G A Bb C D E)
F Mixolydian (F G A B C D E)
G Dorian (G A Bb C D E F)
G Lydian (G A B C D E F)
A Phrygian (A Bb C D E F G)
A Aeolian (A B C D E F G)
Bb Mixolydian (Bb C D E F G A)
B Locrian (B C D E F G A)
```

You can filter candidates by their root note with the `root` option:

```console
$ gitar key C D E F A --root D
2 candidates:
D Dorian (D E F G A B C)  
D Aeolian (D E F G A Bb C)
```
