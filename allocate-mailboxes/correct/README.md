The previous solution was trying to find the minimal maximum distance
from any house to its adjacent mailbox, here we are trying to minimal the
total sum of houses.

k = 1
N = 1
place {K} near the house.

N = 2
place {K} somewhere in the range of [h(1), h(2)] - it doesn't matter
where.

N = 3
place {k} in the middle?

a1b100c
cost(a+1)
= 100 + 0 + 1 = 101
cost(a + 2)
= 99 + 1 + 2 = 102

N = 4
a b c d
somewhere in between b and c
and so on

So we have to choose centers where we place the mailboxes, they can be
either a house, or a range of adjacent houses. We can expand this process
further.

---

Here is why it is always optimal to place the mailbox at the center range,
meaning any place in between 2 most central values or the central
house itself, depending on the parity of N.

Suppose we have chosen any other place, then we have divided the total
cost into left and right parts. The left part is the cost of getting from
the mailbox to every house located to the left from it. The right cost is
calculated in a similar way. Suppose we move the the left now. The cost
change is described by the following formula. The step is 1.

d += L
d -= M
where L is the amount of nodes to the left, M is the amount of nodes to
the right. When we move to the left, we decrease L and increase M; so the
cost will be decreased by a smaller value.

So it does not change as long as M = L. If they are not equal, then it
is going to either be negative or positive. It reaches its extreme
negative(positive) value when we reach the leftmost(rightmost) edge.
What is so bad about that? When M != L, we can always alleviate the
problem by locating the mailbox in the following way:
suppose L < R, L = # nodes to the left from the mailbox,
 	       R = # nodes to the right from the mailbox,
we can move 1 point to the right and get -R to our total distance, with
a lesser cost of having to add L to out total distance. This process does
not make sense when we have L = R, which means it no longer becomes
optimal to do so. Therefore, for a given range, we should always pick a
place where the # nodes to the left = # nodes to the right.

a b c d
p(b) + 2 * d(b, c) / 2
p(b, d) + 2 * d(b, c) / 2

A + C
B + C

A + C - 1
B + C + 1
does not change the situation
they all add up to A + B + 2 * C;

substite 1 with any constant that does not change the A and B.
We have just shows that it does not matter where exactly we put the
mailbox, as long as we put it in a range - so we might put them anywhere
in a range - so ranges are what is important; A and B are adjacent
houses.

is there always a proper solution as to what range to choose, meaning what
A and B to choose from?

===Done===
So we can solve a problem in O(1) after we use some preprocessing on the
initial array. Just put the mailbox at the center of the range and the
cost will be: l(A) + l(B) + d(A, B). So we need to find an optimal
partition of the array into subranges such it is a split.

Update: l(a) + l(b) + d(a, b) * hml(b)

Naive approach would be to use complete search.
But on looking closer we recognize we can fix the left part and try to
solve the right one - we just need to remember the cost of solving the
left part. This is described by the following dynamic programming formula:

i = # of resources (in this case subranges we have)
j = if we only see the first j elements of the array
dp[i][j]
= best (
  dp[i - 1][i - 1] + cost(i, j)
  dp[i - 1][i] + cost(i + 1, j)
  ..,
  dp[i - 1][j - 1] + cost(j, j)
)

cost(*, *) is calculated in O(1), we iterate through dp[i - 1][*] before
dp[i][*], so we have the needed information by the time we are working
with the i-th row. Done?

LC example:

h = [1, 4, 8, 10, 20], k = 3
dp[1][1] = 0
dp[1][2] = 3
dp[1][3] = 7
dp[1][4] = 13
dp[1][5] = 4 + 4 + 3 + 2 + 2 + 10 = 8 + 4 + 3 + 10 = 25

dp[2][1] = none
dp[2][2]
 = min
   dp[1][1] + cost(2, 2) = 0
= 0
dp[2][3]
 = min
   dp[1][1] + cost(2, 3) = 4
   dp[1][2] + cost(3, 3) = 3
= 3
dp[2][4]
 = min
   dp[1][1] + cost(2, 4) = 0 + 6 = 6
   dp[1][2] + cost(3, 4) = 3 + 2 = 5
   dp[1][3] + cost(4, 4) = 7 + 0 = 7
= 5
dp[2][5]
 = min
   dp[1][1] + cost(2, 5) = 0  + 18 = 18
   dp[1][2] + cost(3, 5) = 3  + 12 = 15
   dp[1][3] + cost(4, 5) = 7  + 10 = 17
   dp[1][4] + cost(5, 5) = 14 + 0  = 14
= 14

if k > n then 0 - no need to calculate

dp[3][1] = none
dp[3][2] = none
dp[3][3] =
 min
  dp[2][1] + cost(2, 3) = no need
  dp[2][2] + cost(3, 3) = 0 + 0 = 0
= 0
dp[3][4] =
 min
  dp[2][1] + cost(2, 4) = no need
  dp[2][2] + cost(3, 4) = 0 + 2 = 2
  dp[2][3] + cost(4, 4) = 3 + 0 = 3
= 2
dp[3][5] =
 min
  dp[2][1] + cost(2, 5) = no need
  dp[2][2] + cost(3, 5) = 0 + 12 = 12
  dp[2][3] + cost(4, 5) = 3 + 10 = 13
  dp[2][4] + cost(5, 5) = 5 + 0 = 5
= 5

The same as in the example, so it should work.