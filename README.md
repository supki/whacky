whacky
======
[![Build Status](https://secure.travis-ci.org/supki/whacky.png?branch=master)](https://travis-ci.org/supki/whacky)

Yet another randomly failing program.

Give Whacky a chance and it will run another command if you are lucky, e.g.:

```
% whacky --chance 30 -- echo "Hello!"
```

will print "Hello!" three times in ten runs on average.

Build
-----

To build the executable with Cargo run:

```
% cargo build
```
