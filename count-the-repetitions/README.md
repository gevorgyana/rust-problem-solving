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

N, M are 10^6 => it makes no sense for K * M to be greater than 10^6.
We can iterate through M in range from 1 to N / K using binary search.

So we really have the following problem:
A = [some 100-char pattern] * N
B = [some 100-char pattern] * K
Find B in A - answer yes/no.

If N = 1, K = 1, then it is simple.
If we want to check subsequences with these huge constants, then it takes
a lot of memory and runtime. So we work with sizes that do not depend on
K and N, or at least on one of them.

We only need one contatenation of A to itself to discover every reachable
pattern, and keep in mind that we will be getting closer to the desired
# of them with every concatenation.

So we have the 200-char string AA, it is going to be repeated N - 1 times,
and we have the 100-char string B. If we use dynamic programming now, then
we will end up with an array of length 200, where the last element will
correspond to the place where the match happens, if it happens. If it does
not, then we will have the maximum number of matched chars that we can get
from AA, then we will check if can go on with the word B[prefix..B.len()],
where prefix is the number we just mentioned. So we continue to do the
same thing. It will take us at most 100 iterations to cover the whole
string B, if we fall off, then there is no answer at all. If we get to the
end, we will know how many joints we need to get exactly one match B in
A * N1, where N1 is the counter that we kept increasing on every
concatenation of type A..AA. But we can start right from that moment and
start getting the next B without needing one more concatenation to start
over..

Will dp help with that?

Suppose the poisitions we will be at after the full B acquisition are
periodic, they have to be! So we have 100 slots at most to stop at some
position, and if we do, then we went through one full period! Now we got
the whole thing.

Seems correct.

So the really simplified problem. The rest is going to appear later.

1) try to find B in A and remember the results (the result is the # of
chars from B found in A) - call it prefix.

2) increase the counter N1, then try to find B[prefix..B.len()] in AA.
Remember the results - update the prefix.

...

F) increase the counter N1, then try to find B[prefix..B.len()] in AA.
Remember the results - update the prefix.

If at any step prefix reaches B.len(), then we have found B completely.
What can we say about the poisition we found it at? We need to choose the
minimal position - because we do not want to miss anything. Is it always
safe to do so? Yes, look at the previous examples, plus it makes sense -
we can choose the closest match and then use whatever we want next. If we
pick some other match, then we will have the same # of matches but at some
further position, which means we will have less chars to choose from. So
we indeed should pick the earliest full match.

So what is that minimal position? If it is just the end of the
concatenated A...A string, then we are good - we have started from this
offset of size 0, if not, then we are at some offset 1 < {l} <= 100.
We remember that and save the character coming from {offset}..A.len(),
then start trying to find B again - follow the same algorithm as above.

1) N1 = is the same; try to find B in what you have currently, which is
string S = A[offset..A.len()]. Run dynamic programming recognition of
the minimal substring, which takes 10^4 ops. You have the amount of chars
from B that are covered - prefix. You also have the new prefix if it was
already met - 0, for example, then stop.

In short, keep running until you end the period. Then multiply by whatever
you need and report the result.

To solve the original problem, use binary search to find {M}.

--- Maybe it is not even dp, but one more memoization problem.

First, I need to solve it here to understand what the code should look
like. Maybe there is no need in using dynamic programming at all to
find the subsequences.

A, B given

try to find the first letter in A, then try to find the second letter
there, then third, and so on. Stop when you have finished. If you have
not found the first character of B in A, then stop - there is no
answer. If you have found some characters, from 2 to B.len(), then if
B.len(), then see where we found the last one - if at some position
that we have already seen, then stop - we have found a cycle. If not,
then go on, but remember that we had been at this position.

-- See also the main.rs comments.
Don't even need to remember the position, just remember the proportions
of how many As we can get from how many Bs. That's it. Stop when you
recognize the cycle.
-- Example solutions

abcdefg
acf:
1 A = 1 B and stopped at one-before-the-last char
2 A = 2 B and stopped at one-before-the-last char
found cycle
Need N B's? take N A's.
How many A's are left? L = s1 - N.
We really need to get B N * M times.
We only got it N times.
Now we need to get it N * M times, where M is from 0 to whatever.
L / N = M

No! Get as many B's as you can from the # of A's that you have.
We got N ones. Then divide N / N and get one. For this case this is
indeed correct.

Generally, see how we can get B's from A's. We will have a rule that
has length up to 100 for consecutive values of the argument. Then we
count how many B's we can get from the # of A's that we have. Say it is
S. Then we know that the lenght of one block of B's is K, so we need
to know how many blocks we can get - divide S / K and you will get M,
which is the maximum value that problem asks for.

--
abcdefg
aca:
1 A = 1 B and stopped at the first char
4 A = 2 B and stopped at the first char
found the cycle

--
abcdefgaa
aca:
1 A = 1 B and stopped at one-before-the-last-character
2 A = 2 B and stopped at one-before-the-last character.
found the cycle

==
(use that greedy approach to get the most economic substring. we can't
do better than that, and we do at least as good as with all other
possibilities)

===
Huge update:
there will be only 1 cycle, BUT  it may not start from 0-th position,
thereofre we detect the first repeating node in the list, then we
naively try to do whta we intended to do until we reach the cache
barrier, which will mean we are about to enter the cycle. Then we are
free to fast-forward, then do the same thing again, or use the
information we have already about how we can get Bs from As for every
# of As that does not exceed the length of the cycle.

The length of the cycle <= 100, therefore we will have enough time.

We know everything that happens insdie the cycle, but we don't know
how to get there, so getting there will take O(100) time.