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

Currently, `gitar` only has one command (`find`); but functionality for note input will likely be the same for future commands as well. Thus, it is covered here once for convenience.

When inputting notes, you must supply a *name* and an *octave number*:

* note names must be one of the following (case-sensitive): `C`, `Db`, `D`, `Eb`, `E`, `F`, `Gb`, `G`, `Ab`, `A`, `Bb`, `B`

* and the octave number is simply any positive whole number

Examples of valid note inputs are `F5`, `Ab0`, and `C10`.

Examples of *invalid* note inputs are `D#3`, `Eb`, and `A-10`.

## Command usage

### `Find`

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
