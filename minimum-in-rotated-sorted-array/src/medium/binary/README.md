`bare` is the same as `libs` except for `bare` not using external crates.

The algorithm uses binary search to find the first mismatch, which is guaranteed to be
followed only by mismatches.

Here, mismatch is deinined on the array of tuples constructed as follows.

First, fold the array in half, producing the following sequence, whose length is `N / 2`,
where `N` is the length of the original array:

Given
```
1, 2, ..., N
```
Produce
```
(1, N), (2, N - 1), ..., (N / 2 - 1, N / 2 + 1)[, (N / 2, N / 2), is N % 2 == 1]
```

For this structure, a mismatch is defined as follows.

If for some index `i`, the left part of the tuple `L = produced_array[i].0` is greater
than `R = produced_array[i].1`, then the array is not sorted at this point - for it to be
sorted, the right part `R` has to be greater than `L` (attention - no two elements in
this task can compare equal, see the hard version of the problem for the case where it
is possible). This is defined as a mismatch.

If we find the first mismatch, we are guaranteed that the next tuples in the produced
array are going to be mismatches, too. This is because the initial array looks like a
unimodal funciton, which has two extreme points, and the minimum is right after the
maximum (see the ternary search part of the explanation to get it how simple it is to use
ternary search here).

```
       c <- max value
    ..
  b
a
                e
	     ..
           h
         g <- min value, coming right after max value

```

For any array that is sorted and then rotated exactly once as per the problem statement,
the resulting array has the structure as described in the example above and by the
following relations:

```
a < b < c < d
e < f < g < h < a
d > e
```

Therefore, suppose we had a mismatch followed by a match. Then some of the previous
relations would not hold.

(In case we face the hard version of this problem, the array is non-decreasing, which is
more general. It complicates the task and makes ternary search or this binary search not
applicable).

```     max values
        c ... d
a ... b                 g ... h
                e ... f
		min values
```
And the following relations would hold.
```
a <= .. <= b <= c <= .. <= d
d <= .. <= f <= g <= .. <= h <= a
d > e
```

This approach stops working when non-distinguishable elemenets are allowed, because once
we see a tuple that gives a mismatch we do not know for sure if we are looking at the
local equality range or at the equality ranges separated by extreme points. Therefore we
have to try both scenarios, which is no better than O(N) time complexity.

Example: take the preceding picture. The length of local equlity ranges like [a, b] or
[c, d] can be arbitrary, so when we hit the two elements, can we say, only by comparing
them one to another, that they are from the same local equality range, or they are from
two ranges like [a, b] and [g, h], which are actually separated by the min point that we
are required to return, which are e, ..., f (any of them).
