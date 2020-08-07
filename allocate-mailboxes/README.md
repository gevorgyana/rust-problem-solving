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