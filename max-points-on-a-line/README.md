The naive solution would take cubic time. The suggested approach works
for quadratic time. First thing to notice here is that there is actually
no way we can use sorting. It is not a formal argument, but I have tried
sorting the points in clockwise/counter-clockwise order and that gives
no valuable information to solve the problem.

Quadratic solution:
Take each pair of points and draw a line between them. Remember the
slope of the resulting line coming from the first point to the second,
and maintain a cache of already seen slopes. This is it.

Submission:
https://leetcode.com/submissions/detail/367390297/