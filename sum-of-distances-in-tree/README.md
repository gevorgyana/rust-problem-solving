The naive solution would look like this: connect the nodes in two
directions so that we can move from a child to a parent - to be able to
move from any node in the tree to any other node in it. Then run N BFS
searches to accumulate the sum of distances to all other nodes on the
fly. This would take quadratic time.

Here is the proposed solution that would work in linear time.
See the solution itself.
The submission: https://leetcode.com/submissions/detail/371884663/