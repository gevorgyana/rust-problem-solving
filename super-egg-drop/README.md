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

f(5; k) - f(0; k - 1)
f(4; k) - f(1; k - 1)

f(5; k) >= f(4; k)
f(0; k - 1) <= f(1; k - 1)
that is why the function is monotonic.
Following this logic, the index is always in the middle? But it is not
always the case. We should check the minimal value on a unimodal function.
I am not sure if the function is unimodal.... If it is, then run ternary
search to find the minimal value on the range [0; N / 2], or
[0; N / 2 + 1] for even number

Unimodal?
f(N; k) = f(N - 1; k) when?
not when k = 1,
....
It may be flat at all.... But it should be unimodal.

One more attempt to prove the property of a monotonic function for
d(f(up), f(down))
     breaks      | does not break
3 - f(2; k - 1)  | f(0; k)
2 - f(1; k - 1)  | f(1; k)
1 - f(0; k - 1)  | f(2; k)
d{i} = f(breaks{i}) - f(does not break{i})
k = 1
d1 > d2 > d3
3 - f(2; 0)  | f(0; k)
2 - f(1; 0)  | f(1; k)
1 - f(0; 0)  | f(2; k)
    these
    cancel
    out

k = 2
f(2; 1) - f(0; 2) = 2 - 0 = 2
>
f(1; 1) - f(1; 2) = 1 - 1 = 0 -- take this one
>
f(0; 1) - f(2; 2) = 0 - 2 = -2

So the numbers are [2, 0] - monotonic!
Proving by induction.
N is fixed.
with k = 3 will be the same, does not make sense to check it.
only makes sense to prove for k from 0 to log2(N).
for k = 0 already probved.

Say we proved for all Ni < N, and for each K in there. Now
we are proving for N and K.

f(N; K) = d(f(N - 1; K - 1), f(0, K))     = d1
f(N - 1; K) = d(f(N - 2; K - 1), f(1, K)) = d2
compare these...
by induction, f(N - 1; K - 1) > f(N - 2; K - 1).
Then f(0; K) < f(1; N). Which means that d1 > d2.
And so on....
Util we reach the middle point.

Update. The minimum is somewhere at index {i} with i from [N / 2; N].

       .                           .

                 .                .
		    .         .
		          .
			  minimum
			  this is a unimodal function

Usually, when there is enough K, we expect it to be in the middle, but it
can be anywhere, the only thing we can definitely say is that it makes
sense to expect it to be somewhere in the upper half of the array.
  N  K
f(2; 1) = 2
f(1; 1) = 1
----------------------------------------------------------------------
Proving that it is not monotonic, but that the function is sutable for
ternary search.

This table is right now! to the left are the floors
    breaks     |does not break
1 - f(2; k - 1)|f(0; k)
2 - f(1; k - 1)|f(1; k)
3 - f(0; k - 1)|f(2; k)

k = 1;
For k = 1, each f(N; K) = N.
So f(N) > f(1).
                                      or can also be explained by the rule
				      f(N; 1) = N
k = 2;                                   cannot branch here
f(2; 1) - f(0; 2) = f(2; 1) = 2. (f(2; 1) = 1+ f(1; 1) = 2)
f(1; 1) - f(1; 2) = 0
f(0; 1) - f(2; 2) = 0 - 2 = -2;
Unimodal.

Take 4 floors
    breaks     |does not break
1 - f(3; k - 1)|f(0; k)
2 - f(2; k - 1)|f(1; k)
3 - f(1; k - 1)|f(2; k)
4 - f(0; k - 1)|f(3; k)

k = 1
Done.

k = 2
f(3; 1) - f(0; 2) = f(3; 1) = 3
f(2; 1) - f(1; 2) = 2 - 1 = 1
f(1; 1) - f(2; 2) = 1 - 2 = -1
f(0; 1) - f(3; 2) = 0 - 2 = -2
Still unimodal

k = 3
f(3; 2) - f(0; 3) = 2 - 0 = 2
f(2; 2) - f(1; 3) = 2 - 1 = 1
f(1; 2) - f(2; 3) = 1 - 2 = -1
f(0; 2) - f(3; 3) = 0 - 2 = -2

Now that we have a little more K, we can do the full search and we have
a perfect unimodal funciton. The same is for k = 4 and greater.

N = 5.
Take 5 floors
    breaks     |does not break
1 - f(4; k - 1)|f(0; k)
2 - f(3; k - 1)|f(1; k)
3 - f(2; k - 1)|f(2; k)
4 - f(1; k - 1)|f(3; k)
5 - f(0; k - 1)|f(4; k)

k = 1
Done.

k = 2
f(4; 1) - f(0; 2) = 4 - 0 = 4
f(3; 1) - f(1; 2) = 3 - 1 = 2
f(2; 1) - f(2; 2) = 2 - 2 = 0
f(1; 1) - f(3; 2) = 1 - 2 = -1
f(0; 1) - f(4; 2) = 0 - 3 = -3

k = 3
f(4; 2) - f(0; 3) = 3 - 0 = 3
f(3; 2) - f(1; 3) = 2 - 1 = 1
f(2; 2) - f(2; 3) = 2 - 2 = 0
f(1; 2) - f(3; 3) = 1 - 2 = -1
f(0; 2) - f(4; 3) = 0 - 3 = -3

k = 4
f(4; 3) - f(0; 4) = 3 - 0 = 3
f(3; 3) - f(1; 4) = 2 - 1 = 1
f(2; 3) - f(2; 4) = 2 - 2 = 0
f(1; 3) - f(3; 4) = 1 - 2 = -1
f(0; 3) - f(4; 4) = 0 - 3 = -3

So it is unimodal when K is big enough. And still unimodal when there is
not enough K. It is perfectly distributed when K is enough.

Let's try out 9.
N = 9.
Take 9 floors
    breaks     |does not break
1 - f(8; k - 1)|f(0; k)
2 - f(7; k - 1)|f(1; k)
3 - f(6; k - 1)|f(2; k)
4 - f(5; k - 1)|f(3; k)
5 - f(4; k - 1)|f(4; k)
6 - f(3; k - 1)|f(5; k)
7 - f(2; k - 1)|f(6; k)
8 - f(1; k - 1)|f(7; k)
9 - f(0; k - 1)|f(8; k)

k = 2
f(8; 1) - f(0; 2) = 8 - 0 = 8
f(7; 1) - f(1; 2) = 7 - 1 = 6
f(6; 1) - f(2; 2) = 6 - 2 = 4
f(5; 1) - f(3; 2) = 5 - 2 = 3
f(4; 1) - f(4; 2) = 4 - 3 = 1
f(3; 1) - f(5; 2) = 3 - 3 = 0 -- works indeed - we need to select minimum
                                 of the function
f(2; 1) - f(6; 2) = 2 - 3 = -1
f(1; 1) - f(7; 2) = 1 - 4 = -3
f(0; 1) - f(8; 2) = 0 - 4 = -4

Unimodal.

First we calculate the difficult part, then reuse it in the next step,
and we finish calculating the new difficult part when we hit K > N, where
it makes sense to just take the binary logarithm.

k = 3 ( reusing the calculated values from k = 2)
f(8; 2) - f(0; 3) = 4 - 0 = 4
f(7; 2) - f(1; 3) = 4 - 1 = 3
f(6; 2) - f(2; 3) = 3 - 2 = 1
f(5; 2) - f(3; 3) = 3 - 2 = 1 ---- not unimodal! two equal elements
                                   to one side
f(4; 2) - f(4; 3) = 3 - 3 = 0
f(3; 2) - f(5; 3) =
f(2; 2) - f(6; 3) =
f(1; 2) - f(7; 3) =
f(0; 2) - f(8; 3) =

Alright, not unimodal, but at least sorted so we can chase the 0. Look for
0 in the array. Do binary search on it.

Proving that it is sorted.
Say K = 1.
Proved, as this is literally 1 < 2 < .. < N

K = 2
We reuse part of the already calculated values in this fashion.
f(N; 1) - f(0; 2) = N - f(0; 2) = (N) - 0 = N
f(N - 1; 1) - f(1; 2) = (N - 1) - 1 = N - 2
f(N - 2; 1) - f(2; 2) = (N - 2) - 2 = N - 4
..
in case of odd we hit N / 2 + 1 and then calculate for it these:
f(N / 2; 1) - f(N / 2; 2) = (N / 2) - f(N / 2; 2)
f(N; 0) - f(N - 1; 2)

The idea: the left part of the expressions is calculated at first step,
and the claim hold true. Then at the second step we can say that the left
parts of the expression hold the order (equality is allowed). Then we can
say that f(N - 1; K) <= f(N; K), which hold true. Then it follows that
we have the following scheme:

L1 - R1
L2 - R2
...
LN - RN

where L1 <= L2 <= .. <= LN and R1 <= R2 <= .. <= RN.
Which proves that the function is monotonic!
Done

--------------------------------------------------------------
So we can use binary search. Therefore the time complexity is going to
be O(log2(N) * log2(N))

The alternative is to use dynamic programming to calculate the answer in
O(N * N) time and using the same amount of space.