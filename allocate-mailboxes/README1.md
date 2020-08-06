When we have one mailbox, we don't really have any choice other than to
try to minimize this expression's value:
| a - i | + | b - i | + .. +  | z - i |
with picking the best {i}.

This can be done in O(dist({z}, {a})) by picking each point and
calculating the left and right burdens. We are trying to minimize the
difference between these 2 values. If we store with each node its
distances to all of the nodes coming from it in each direction, then we
have made our task a bit easier.

When we have a tree instead of a line, we have a similar situation, we
will store K values per node to specify the sum of distances that we
need to cover when we go into this node from another connected node.

So we know the answer for left and right burden for every node.
We can calculate that as a step of preprocessing in O(N).
Then we minimize the distance function described above.
We go to each node and see its left burden and its right burden, updating
the answer. We can use binary search here.
We can then select the two nodes that have the two discriminant values
that appear to be the closes ones to the 0 difference between the left and
right burden. We may then go in between these 2 and try to find the best
answer by doing a binary search in O(log2(N)). So we spent O(log(M)) to
find the best pair of adjacent nodes and then O(log(N)) to find the best
point in between them that minimizes the sum.

O(log2(N)) * O(log2(M)) time
O(M)                    space
M                       # houses
N                       upper bound for any distance

So we know how to solve a problem for K = 1 and any N.
The problem can be described as distributing K mailboxes among
contiguous ranges of houses.
Say we know the answer for every N, where N is the amount of houses to the
right that we have, and K = 1. How do we calculate the answer for any N,
and K = 2? We want to find the best way to split the N houses into 2
ranges [1, S - 1] and [S, N] such that they are best covered by the
2 mailboxes that we have. The possible range of divisions is going to be
O(N). And the cost of one such division is calculated as the cost of
solving the subproblem having first S - 1 houses and K - 1 mailboxes,
and solving the range of [S, N] with 1 mailbox. The latter problem can
be calculated easily, we just need to spend that logarithmic search again.
We can actually maintain prefix sums with left and right burdens, as in
one of previous problems.