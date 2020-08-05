Proving that we can use binary search for finding the best way to split
an array.

Formally, we have an array that contains N elements, and a number K that
represents the amount of subarrays that must already exist. We are
therefore allowed to fit the (K+1)-th subarray at the end of the array
with its minimal starting position being equal to (K + 1) and the ending
position equaling N.

Say K = 1 and we have moved to K = 2. What can we say now? We are using
the prefix sum of the single subarray to the left and the prefix sum of
the subarray to the right from it to select the best border element {i}
that is the first index to go to the right subarray.

We can use binary search for the previous procedure - as prefix sums are
increasing functions with increasing order of indices, meaning that for
any {j} greater than {i}, the sum of the left subarray with respect to {j}
is always more than {i}? It actually works when the numbers are
non-negative!

So if numbers are non-negative, then we may use binary search in case
K = 2.

What if K = 3? We will have to find the border index {i} such that the
cost of solving the left subproblem is almost equal to the cost of solving
the right subproblem. The cost of the right subproblem is prefix sum,
which obeys the increasing ordering. The cost of the left subproblem can
is the best (minimal) way of splitting the left subarray into 2 parts. Of
course, greater size of the left subarray implies greater cost, so it
obeys the properties of increasing order too (at least, non-decreasing,
which is enough for binary search).

So in case we can use binary search.
This problem is similar to Super Egg Drop.