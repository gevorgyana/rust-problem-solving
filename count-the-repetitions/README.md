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
quadratic time and memory (we need separate arrays for each prefix of
length 1, 2, .., N).

One last gotcha. What if there are equal elements? Then each of them can
either influence the arrays for the first, second, etc. occurence of the
character, so everything seems fine.

What if we only want to know if there is one occurence?
We can then check for it using an automata-based approach, which would
require linear time. Remember the word we are looking for if we have met
the character that sits previous to it in W. The automata approach would
take linear time and memory too. We would need to differentiate equal
chracters though.

Here is my simplified understanding of the problem.
A is a string.
B is a string.
Find the maximum integer M such a new string S, formed by repeating A M
times, is a subsequence of B.

A naive solution: iterate M = 1, 2, ... until there is now answer. The
cost of a check is O(B).

Use binary search on the answer, the predicate is_true() has the following
form: 0, 0, ..., 0, 1, 1, ..., 1, so we might use it indeed.

Okay, is that suitable? The length of A and B are 10 ^ 8 each. Idk...
Should try it. But there must be a dynamic programming approach.

---

Draft.
First: abc
Second: bcd
Clearly, bcd cannot be obtained from any abcabc...abc string.

First: abc
Second: ab
1 abc can produce 1 ab

First aba
Second: ab
1 aba can produce 1 ab and leave 1 character for reuse.
2 aba can produce 2 ab in the following ways:
ABAaBa
AbaABa
the residue will always remain 1, so it does not matter now, but maybe
it matters in some cases?

First: abaa
Second: aba
1 abaa
ABa
ABaA

2 abaaabaa
ABAa
 ABAa + abaa
  ABAaABAa
  ABAaABaA
ABaA

Clearly, we can blocks the use of some letter. Is there a case where it
matters? Maybe this?

First: aabaa
Second: aaba

AABaA
 AABaAaabaa:
  AABaAAABaA
  AABaAAABAa
   ...

Nope, if we want to increase the amount of similar prefix letters, then we
need to increase the prefix of the word that we are using for search, too.

---

Is it always true that if there are N occurences of B in A, then there are
N * M occurences in [A, M]?
A: aaa
B: aa
1 A = 1 B
2 A = 3 B
No longer a linear function!

---
A is repetition of p
B is repetition of q
What if there are always 2 reps of p and 1 rep of q?
The question is the same. How many times can we push B to the end of
itself so that we find the resulting string in A?

A: aaaa = aa * 2
B: aa = a * 2

B?
yes we can do that
BB?
yes we can do that
BBB?
no.

Okay, let's pick arbitrary characters and solve them.
A: abcd * 2
B: cd * 1
B1: da * 1

So we will benefit from another addition in case of B1, as we clearly need
those 2 guys from the polar sides of A to stick to each other.

2 B is max we can get.
1 B1 is max we can get.

Here is a more general example:
A: abcdefg * 5
B: a * 5
5 A = 1 B
So indeed the function looks like anything we could imagine - y < x,
y = x, and y > x.

---
So let's simplify our job and say that the # of reps for A can be
arbitrary, while the # of reps for B is fixed to be one. I think that it
is a fair assumption, because we will probably just multiply the answer
for this case by the actual # of reps of B and still get the correct
result, but that is an assumption.

General example:
A = abcdefg * N
B1 = x
B2 = xy
B3 = xyz

let's solve for this case;
Solution(A, B1) = max # of reps of B1 such that the resulting word can
still be found as a subsequence of A.
Solution(A, B2) = [by analogy],
Solution(A, B3) = [by analogy].
