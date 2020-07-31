This solution is simple. I don't know why would anybody want to use
dynamic programming.

First of all, here are the ideas I used to devise the solution. Whenever
we have more than one egg, we should use it to optimize the search
process by removing a half of the options and thus reducing the task size.
Why half? Because we are trying to equalize the sizes of the two subtasks
that appear after such a division. So there may be 2 options: an odd/even
number N.

If N is odd, then we have only one way to divide the linear piece of
memory to get the equal sides. Everything is simple here, one such
transition takes O(1) time and memory, and we proceed by log2(N).

If N is even, then there are two options on how to divide the array. Say
we choose to try the upper part of the median indices (the one closer to
N) - then we have two subtasks:

(N = 2 * K, P = the amount of eggs)

f(K, P - 1), f(K - 1, P),
Which of them is clearly better than the other? f(K, P - 1) is better
because it has more eggs and the lesser amount of options to choose from.
Let's look at the other option - what if we choose the bottom median
point?

f(K - 1, P - 1), f(K, P)
We can't say (at least I do not know how to) which one is better, so we
will have to try both.

When K > log2(N), we can complete binary search on the imaginary array of
floors, which is the best way we can do and generally something we are
trying to mimic.

When we hit K = 1, we cannot optimize the search process
anymore, so we have to play carefully - drop the egg from the first floor,
then second floor etc. util it cracks. So

f(1, N) = N

Now we are ready to write the recursive caluclation rule (see the
program). What time complexity does it fit it? We have at worst case
2 options to choose from, each of them works with N divided in half, so
there the recursion tree has height O(log2(N)) and the amount of heaves
is 2 ^ (log2 (N) ) = N. Therefore we have O(N * log(N)) as time
complexity.

-----

Dynamic programming. We can use the following procedure to caluclate
f(K, N) from bottom to top.

f(1, N) = N
f(V, N) = log2(N), where V >= log2(N)
use the previous recursive caluclation rule to go on... It is guaranteed
that we will only use the precalculated values if we go from bottom to
top with regard to N.

The table looks like

K1 K2 ... min(K, N)
                    N
		    ...
		    1

Required memory and time complexity: O(N * M). But the constant is at
least 1 / 2, because we do need half the them (the triangle).
