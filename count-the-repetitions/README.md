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

Let's solve for this case;
Solution(A, B1) = max # of reps of B1 such that the resulting word can
still be found as a subsequence of A.
Solution(A, B2) = [by analogy],
Solution(A, B3) = [by analogy].

Solution(A, B1):
for all chars from A, check how we can get x from it; count the # of those
in one word, and multiply that by N. Done.

Solution(A, B2):
The answer for the previous lenght of the word B2, which is the answer
for B1, should be already calculated. So we know that. We also know that
now we can face with a situation where we would need more than one
instance of A to get at least one B2 (check one of the previous
paragraphs).

--
A aaa * 2
B aa * 1

answer for a * 1 is linear funciton y = 3 * x
answer for aa * 1 is y:
 1 -> 1
 2 -> 3
 3 -> 4
 4 -> 6,
 ...
it works like if we did a similar dp as the 'all substrings that match
a specific word', but here is a more interesting case:

A abcd * 2
B da * 1

just {d}:
abcd
   1

then {da}:
abcd
   1 - we saw it previously
       so we go on and check this out
       what if the next to it is something that we need?
       that means we have to look at the first letter,
       but that also means that we will only benefit from such a division
       on each joint that we have.

       so we mark this thing here as one.
1

now we have got to the full length and see the results. Multiply that
thing by the # of reps of A that we have and divide it by the # of reps
in B that we are given. Seems like done.

A remark. If we are at B-length that is greater than the position
(in 1-indexing) of the current character that is marked with one, we know
it is something that appears only when joints are allowed.

---

What if more than 1 concatenation is required to obtain the desired
string?

Like here:
A = abc * N
B = aaa * M
?
Can we handle that with dp?
Suppose we could have a whole array.
What is then?
We could just use the dynamic programming approach on the reduces strings
then. What's the difference? That would take too much time (10 ^ 16),
and a lot of space too.

So the problem is really that we have huge strings but we know that they
have cyclic nature. And we are required to answer what the maximum
number of times that we have to repeat one of them to fit into the other
is.

Let's say we can iterate the maximum number and do that in logarithmic
search, so that we have that number M.

So here is the problem. Find if there is a subsequece B in A, where
A = [some 100-char pattern] * N
B = ([some 100-char pattern] * K) * M
