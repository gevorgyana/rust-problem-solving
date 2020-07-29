Here is a slighly optimized solution that takes M * M * N space and time
complexity.

dp[i][j][k] is the answer when we are at {i}-th node, the furthest node
that we can reach is {j}, and we also have {k} liters of gas as residue.
{i} and {j} are from 0..M, where M = 500. Then we observe that given no
residue can be greater than the distance between {j}-th and {j + 1}-th
stations. And the whole sum of distances is exactly N. Therefore this
memoization technique would give the above time and space complexity.

The naive solution would suggest that we choose the best option from
a set of 2^M * M if we wanted to construct the solutions and then check
their validity.

I managed to think of the previous solution, but it would not work.
Checked the solution, so it makes no sense to solve it here anymore...