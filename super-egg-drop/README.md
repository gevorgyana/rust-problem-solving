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

Updates.

1. The solution should mimic binary serach in best case. Not ternary or
any other base search.

2. Binary search should attempt to divide the array in the middle.

3. 2 is not always optimial, as there can be different amount of K - steps
that we are allowed to use for optimizing the search process. Other words,
we may not be able to fully run binary search on some of the cases and
therefore it might not be optimial to mimic binary serach.

4. What to do then? Well, try somewhere around binary search and move
downwards until you reach the point where f(up) is most similar to
f(down). Not size(up) and size(down), which would be the case if we had
no parameter K and were trying to conduct binary search by excluding half
of the input from search.

Here is another update. We can use ternary search to search for the
point where the difference between the upper problem and the lower
problem is minimal. We just need to make sure that there is a unimodal
function, and I think it is. Why? Ternary search in the worst case gives
us N - 1 / 3 * N size of the new problem. But it works on unimodal
functions and it has to imply that there can be no equal elements to one
side of minimum or maximum. Indeed I think it works. At least because we
have the increase of the upper problem if we go down from top, and
decrease of the lower problem.

6 f(5; k) | f(0; k - 1)
5 f(4; k) | f(1; k - 1)
4 f(3; k) | f(2; k - 1)
3 f(2; k) | f(3; k - 1)
2 f(1; k) | f(4; k - 1)
1 f(0; k) | f(5; k - 1)

clearly we can see the increase and decrease. So what if we form the
sequence of differences between the upper and lower problems - the answers
on them actually - like this:

f(5; k) - f(0; k - 1) = f(5; k)
f(4; k) - f(1; k - 1)
f(3; k) - f(2; k - 1)
f(2; k) - f(3; k - 1)
f(1; k) - f(4; k - 1)
f(0; k) - f(5; k - 1) = -f(5; k - 1)

here it is {lower - upper}, don't get it confused!
so what we can see is the edges are polar to each other. So there won't be
any chance to use ternary search - but as the function should be
monotonic, we can use binary search here! Only need to prove the property
of a monotonic function.

If it is one, then we can say we have the following search tree:

  log2(N) - height thanks to smart selection
   +
+ ... log2(N) - width thanks to simulating binary search - we will do
                at MOST log2(N) steps to complete the search process
   +

So we would have a bounding box for this tree whose area is
log2(N) * log2(N), which is great!

The other approach is using dynamic programming to optimize the search.
Like this.

This is guaranteed to work. In the previous case I need to prove that
that function is monotonic to claim that it works.
Take the same example with 6 floors. The same function - difference
between the solutions to upper and lower problems - take the one that is
the lowest - I think it will be the optimal way to divide the subtasks.

Try every one of them - they will refer to already calculated values of
dp[i][j], where i is from 0 to N - 1 and j is K or K - 1.
Bottom-up or vice versa - does not matter.

It also makes sense to only choose from the first N / 2 floors.