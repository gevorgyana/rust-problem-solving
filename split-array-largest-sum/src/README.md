Array elements can be negative.
TODO Think what happens if they are always positive. Can we simplify the
solution then?

Suppose we have N array elements.
1, 2, ..., N

The answer for m = 1 is the sum of all elements in input.

When m = 2, choose the best i from {1, .., N} such that
abs(sum(1, .., i - 1) - sum(i, .., N)) is minimal.
For convenience, calculate prefix sums of the array as a step of
preprocessing. We can actually use binary search to calculate {i} in
logarithmic time. The values that are going to be searched for are:
f{i} = sum(1, .., i - 1) - sum(i, .., N)
They will form increasing order.

Suppose m = 3:
Choose the division point from {3, ..., N} - call it {i}. We can
devise the following information now: the answer for m = 3 includes the
answer for all rightmost subarrays coming from a starting index greater
than 2 and less than N, and ending index that should always stay N. That
means that the following options are possible.

1, 2, |, 3, .., N {1}
1, 2, 3, | , 4, ..., N {2}
...
1, ..., |, N {3}
where '|' means the divsion between subarrays.

Here is how we calculate the answer for {1}:
dp[3][N]:
	 update_with(
		abs(
			pref_sum(3, N),
			dp[2][2] // means the answer for m = 2 when
				 // the amount of available elements from
				 // the left is 2
		)
	)

{2}:

dp[3][N]:
	 update_with(
		abs(
			pref_sum(4, N),
			dp[2][3] // means the answer for m = 2 when
				 // the amount of available elements from
				 // the left is 2
		)
	),
ans so on. For that, we already need to have calculated the values of
dp[2][2], dp[2][3], .., dp[2][N - 1]. This means that we have to add
the second dimension to out dp that corresponds to the amount of availablecontiguous array items from the left part.

If we use linear search to update the answer O(N) times, then we need
O(N) * O(M) * O(N) time, first two multipiers are explained by the fact
we are using 2-dimensional dp[*][*]. The last one appears because we
use binary search to update the best value for a single dp[*][*] value.

It seems to me that binary search can be used not only for case m = 1, but
also when m = *.