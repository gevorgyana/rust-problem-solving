The algorithm uses the binary search procedure to find the first mismatch, which is
guaranteed to be followed only by mismatches. This approach stops working where there
are also non-distinguishable elemenets in the array, in this case equal numbers. It stops
working because generally once we hit a tuple (see the algorithm itself) whose left part
equals its right part, it can be actually a mismatch or a match - we do not know for
sure and therefore have to try both scenarios, which is no better than O(N) in worst.
