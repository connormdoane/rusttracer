import itertools
import sys

# A tool to put one r/g/b value per line on a .ppm image file, for easier iterating in Rust
# After running it is necessary to manually remove the newline after line 2 and place a space between line 2 and (the old) line 3's contents

with open(sys.argv[1]) as f:
    lines = [line.split() for line in f]

lines = list(itertools.chain.from_iterable(lines))

with open(sys.argv[2], 'w+') as f:
    for line in lines:
        f.write(line)
        f.write("\n")
