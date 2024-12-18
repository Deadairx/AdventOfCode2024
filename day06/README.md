# Part 1
pretty easy, I spent more time fighting the compiler.

# Part 2
Part 2 really has me scratching my head. My idea was to traverse the original
path and scan to the right for a visted position with that direction, if I find
that, I know I've found a place for obstruction in front of my current
position. This theory passes the example, but when run against the input, it
falls short. Apearantly there is another case I'm not covering but I can't test
against it in the given example...

## Looking for help
Looking at [reddit for
solutions](https://old.reddit.com/r/adventofcode/comments/1h7tovg/2024_day_6_solutions/),
I can't seem to find anyone that did my method but there are several who did a
brute force method where I assume they just place an obstruction somewhere
along the original path and then check to see if they go off grid (`false`) or
reach a visited position in the same direction (`true`).

[One interesting
solution](https://old.reddit.com/r/adventofcode/comments/1h7tovg/2024_day_6_solutions/m0ps8zl/)
I found was by keeping track of which side the original obstructions were hit
from, and if they were hit there again, that's a loop.

## a better example
Someone created [a better example map](https://old.reddit.com/r/adventofcode/comments/1h7tovg/2024_day_6_solutions/m0stkxk/)
that might cover all cases I'm missing. I'll have to give this a try.

### Update
Tried it and I must be missing something because my code only found 5 of the
supposed 19 obstructions!

