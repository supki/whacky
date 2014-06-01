whacky
======

Yet another randomly failing program.

Give Whacky a chance and it will run another command if you are lucky, e.g.:

```
% whacky --chance 30 -- echo "Hello!"
```

will print "Hello!" three times in ten runs on average.

Installation
------------

To build the executable just run `make`

```
% make
```

That will put `whacky` under `build` directory. If you don't like `build`
for some reason, you can customize the location in a fairly simple way:

```
% BUILDDIR=$HOME/bin make
```
