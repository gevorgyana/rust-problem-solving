https://leetcode.com/problems/count-the-repetitions/

Thinking process:
check if a string is a substring (not necessarily contiguous) of another
solved in linear time?

Let's recall how we solve the problem of finding all occurences, at least
the number of those. That takes quadratic time, where N is the size of
both the checked string W and containig word C.

Start from the prefix of W of size 1 - P.
Check if P is contained in the string to be checked - that takes O(N)
time. If no, then quit. If yes, keep growing the counter for every
character past the one that is P, incrementing it whenever we see P again.

Then how do we know how many 2-character words are there? Go through each
character of C and see if there is already 1 in there and the next one
matches the checked one, then increase the value for the next one by
one? Nope, multiply it by the current counter, which happens to be one.
Okay, go on and prove by induction. So this approach works indeed in
quadratic time and uses linear memory.

What if we only want to know if there is one occurence? Then we are going
to have a different problem.