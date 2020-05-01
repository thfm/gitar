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

produces the following output:

```console
Open 1 string
String 2, fret 5
String 3, fret 9
String 4, fret 14
String 5, fret 19
```

If there are no occurences of the given note, the output will say as such:

```console
$ gitar find Bb20
```

```console
No occurences.
```

#### Adjusting the `tuning`

What if you want to find the locations of a certain note in a non-standard tuning? Simply specify open string note values for the `tuning` option:

```console
$ gitar find D2 --tuning D2 A2 D3 G3 B3 E4
```

```console
Open 6 string
```

You don't actually have to put in six values; in fact, you can type any number of notes, and the program will construct a guitar with that many strings. This means that you can use `find` for bass guitars as well, as is demonstrated below:

```console
$ gitar find Gb1 --tuning E1 A1 D2 G2
```

```console
String 4, fret 2
```

#### Changing the number of `frets`

Finally, you may encounter a scenario where the output of `find` is different from what you expect. Take the following, for example:

```console
$ gitar find E6
```

This outputs `No occurences`, which is true for *most* guitars, except for those with 24 or more frets (or a [different tuning](#adjusting-the-tuning)). You can use the `frets` option if this exception applies:

```console
$ gitar find E6 --frets 24
```

```console
String 1, fret 24
```

This option is also useful for when you want to simulate a guitar with less than the default number of frets (which, in this program, is 21):

```console
$ gitar find F4 -f 18
```

```console
String 1, fret 1
String 2, fret 6
String 3, fret 10
String 4, fret 15
```
