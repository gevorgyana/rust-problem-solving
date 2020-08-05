allocating {k} mailboxes.

There are N houses.

a      b             c | houses
   y                   | 1 mailbox;

This task is similar to Codeforces "Коровники" in one of SPBGU gym dp
contests.

Complete search?
Trying each {y} position takes O(N!) where N is the distance between the
leftmost and rightmost values. Clearly unacceptable.

1 station. O(M) to find the best answer - where M is the amount of houses.
The approach is to try splitting the distance between adjacent houses.
This will not work.

Counter-example.

a_b             z
 1 _____________
        INF
So we have only 2 points to place the mailbox at, if we follow the
previous approach. Let's say we place it between the two {a} and {b},
the cost then is |b - a| + |(b - a) / 2| + |z - b|, and the last value
is greater than anything else, say INF. So we get INF plus 1.5. Place the
mailbox between b and z - |INF / 2| + |1|, which is clearly better. Does
it make sense to shift the mailbox to any direction? At this point, the
amount of houses to each side matters. If there were 2 houses located
at positions INF and INF+1 to the right from {b}, then it would make sense
to shift the point a bit to the right. We would satisfy 2 houses, and have
-2 to the value that we are trying to minimize, and +1. Would it be more
profitable to place the mailbox at the third position between the two
furthest from the beginning houses? No, as the cost would then include
INF, which is unwanted. So we conclude that it is not always optimal to
place the mailbox at the middle position between some values.

Sometimes it makes sense to place the mailbox near some house.

a INF b INF c
Here, the optimal solution would be to place it at {b}.
----------------------------------

- Can use binary search?
We can use binary search on M to find the best range to search in, then
go inside and do a binary search on N to get the best specific value.
But we can do better and find the center of masses.

- is center of masses the answer? Better come to this result in a
constructive way.

But suppose yes. Then we are equally distant from each of the houses. Then
we have equal or almost equal left and right burdens, which is what we
want. Seems correct.

Each new node will pull the center of masses in one way. So we can
construct the center of masses dynamically.

- is it always profitable to place the thing either in some node, or in
between some of them.
Show an example when it is not the case.
a 2 b
a 1 x 1 b
a 1 x 1 b 100 c
it is now profitable to do this instead
a b x c
but where exactly? how should we divide the distance between b and c?
there is definitely no sense in doing 50|50, as then the left burden
will be 50 + 2 and the right one will be 50. we will then move left and
have 50 + 1 vs. 51, which will give us the desired result - literally the
left and right burderns are equal. this example shows that it is not
always optimal to place the mailbox at a specifically defined place - it
may be anything other than center of two adjacent houses or the houses
themselves.

- how does the result change from some optimal point?
How the result changes. When we only start, we have the left and right
burdens. We start at the leftmost house and the left burden is 0, while
the rightmost burden is the sum of all houses' coords - (N - 1) * coord of
the leftmost house. If we move just one step right, we will be closer to
the optimum by (N - 1) points. If we move 2 points to the right, the
left burden will be 2 and the right one will decrease by (N - 1) * 2,
compared to the initial state. We will continue this process until we
reach the second house. Then the left burden.

We want minimum total distance, this constraint is satisfied when we have
the minimal difference between the right and left burdens. Why? Suppose
not. Then we might go one step to the right and the left burden will
increase by N, the right will decrease by M. So the value left_burden
- right_burden will move to either negative or positive side from 0.
If negative, then taking absolute values of these values will actually
give greater values of burden from the right, and the absolute value of
the left burden will preserve the values. So what do we get? We have just
increased the values that we are trying to minimize. Which makes me think
that we must be willing to equalize the left and right burden.

So for each point there is left and right burdern. If we move from it to
the right, we will have the left burden increased by {K}, where K is the
amount of houses that we have to the left, and decreased by
