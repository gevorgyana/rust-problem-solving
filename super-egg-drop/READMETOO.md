the previous readme was wrong about the time complexity of performing
binary search on N values. Here is the search tree for N = 100.

           100
	    50 -- we are guaranteed to reduce the problem in half per one
	          step of binary serach. We will have O(log2(100)) of such
		  steps. How long will the longest of them, the first
		  step, take? Then we multiply that value and the
		  logarithm and get the time comlexity.

     50           to calculate f(50), we perform binary search again,
                  which means will have to perform O(log2(50)) steps.
   25

So the general patten here is: we have to perform logarithm steps, and
then to perform each step, we take logarithm of that the subproblem size
of functions of problem size, reduced in half and so on.

Like this:
   N

   N / 2
            N / 4
	            N / 8
   N / 4
            N / 8
   N / 8

So that forms a triangle, but anyway the complexity is quadratic
logarithm. But if we cache the results, we can reduce time complexity to
O(log2(N)).