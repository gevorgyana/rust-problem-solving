First, let's solve the problem for K = 1 and any N.
There are M houses.

For 1 house (N = 1), we just place the mailbox at its position.
For 2 houses, place it in the middle.
For 3 houses, two of which are equidistant from the third one, place it at
the third one. If the rightmost house is further from the central than the
leftmost one, then we need to shift the mailbox to the right a bit.

Say we have the following cituation:
x1x100x

We try to place the mailbox at position 1 + 50:
sum-distances-to-the-left = 51 + 50
sum-distances-to-the-right = 50
sum-total = 151
so we might the middle point does not work.
the solution should be shifted to the left from position 51 by
approximately 25:
place at position 1 + 25
sum-distances-to-the-left = 26 + 25 = 61
sum-distances-to-the-right = 75
sum-total = 131
already better, now to the right and so on...

This gives us a hint that the address space for the answer is from
0 to N, and not from 0 to M. So the asnwer indeed might be not guessable
at all times.

We were trying to minimize the difference between the left and the right
sum - when we achieve the optimal point, we have the best answer - the
minimal sum to the left and to the right - I don't know why btw, it just
seems a common thing in dp problems and somehow makes sense, though it
needs to be proven TODO.

The more general formula of calculating sum-total:
sum-total = sum-total-left + sum-total-right;
sum-total-left: can be dynamically calculated:
		say we know the answer for the left part;
		and how many nodes there are;
		(this is a similar idea to sum of distances in binary
		tree; see a related Leetcode problem)
		we can now say we have come further from F nodes by
		L units - so we increadse the current answer by
		F * L.
sum-total-rigth:similar idea, but move in the opposite direction

More than that, we can say that we can give an asnwer for a range
[N1, N2] in the following manner:
+sum-total-left(N1) - sum-total-left(N1 - 1)
+sum-total-right(N2) - sum-total-right(N2 + 1)

So we can now devise the value of the answer for any contiguous subrange
of [0; N].

Alright, now we can solve for any N and K = 1. What if K == 2?
We have to find the minimal value of a function on a given arrangement of
mailboxes. An arrangement of a mailbox is a range of houses O(M) that
are attached to the mailbox. The answer to the problem is the values of
the sum-total function on the optimal arrangement. It makes no sense for
one house to be served by 2 mailboxes, although in some cases in may
happen, we can just pick one of the nearest ones to it and go with it.
So we can say that the answer consists of non-intelaping contiguous
ranges, each of which is covered by a mailbox. The covering also should be
full.

This is a common pattern in dynamic programming. Now we want to solve for
K = 2, so let's say that we are now arranging the rightmost subrange, and
we can see that its leftmost end can be anything from 1 to N inclusive.
Its rightmost end is fixed for a given value N_. We will be calculating
for every N_ from [0, N]. So we pick the left end, and then update the
best answer (in terms of function value, as the task does not require us
to give the optimal arrangement):

dp[1][N_ - 1], which is already calculated, as we have all the values for
the second coordinate, which takes O(M) space.
+ sum-total(N_, N)

Calculate this for every N_ and you will have the new induction base,
keep going until you calculate everything.

a b c d
d(a, c) = d(a, b) + d(b, c)
l1(a) = 0
l1(b) = d(a, b) * 1 + l1(a)
l1(c) = d(b, c) * 2 + l1(b)
l1(d) = d(c, d) * 3 + l1(c) = d(c, d) * 3 + d(b, c) * 2 + d(a, b) * 1 + 0
= A

l2(a) = none
l2(b) = 0
l2(c) = d(b, c) * 1 + l2(b)
l2(d) = d(c, d) * 2 + l2(c) = d(c, d) * 2 + d(b, c) * 1 + 0
= B

l2(c) from l1(*) only?

B = A - d(c, d) - d(b, c) - d(a, b) - 0

l3(d) = d(c, d)
= C

l3 from l1(*) only?

so we might use the following formula to calculate l{i}(x):
use O(i) actions to subtract from l1(x) certain sums - it takes too long;
we can just maintain the right prefix and it will take us O(1)

Nope! Wrong! Here is how to recaclulate L{i}:

Suppose we have calulated L1(*),
now we need L2(*)
a --- b --- ... --- z
c --- d --- ... --- z
If we want to calcualte Lc(z), and we have La(*), then we do the
following:
there are exactly 2 extra ways coming from {z} to {c} - one of them uses
{c} to get to {a}, and the other one - to get to {b}. So we subtract the
amount of nodes to the left from {c} * the distance from {c} to {z} from
the values of La(z). Then we also need to remove the little paths from the
left - the path from {a} to {b}, which is d(a,b) + d(b,c), and then the
path from {b} to {c}, which is d(b,c). But this is the values of
La{c}.

Exactly, La{c} is calculated as follows:
La{a} = 0
La{b} = 0 + d(a, b)
La{c} = d(a, b) + 2 * d(b, c)
Yeah it works!

-----------------
Maybe we can use the left prefix sum to build the right prefix sum?
Yes, the left prefix sum is now mellable to represent any contiguous range in the array, so now we can just call it right prefix sum and use the
previous formulas for calculations.

        // right price is left price with a shift.