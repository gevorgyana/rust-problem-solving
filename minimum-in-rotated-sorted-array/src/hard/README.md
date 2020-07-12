There is nothing better than a simple linear algorithm. Here is why. Imagine we are
doing a simple ternary search on a unimodal funciton (see the `medium` section for
explanation). The ternary search algorithm supposes that if the two array elements
being checked at current step compare equal, that means that they are located at the
same distance from the extreme point. Here is an example

```
     +
... + + ...
    p q
```

if `p = q`, then the algorithm prunes everything except the [p, q] region.

Here is a coutner-example that shows that for a given the array that the task talks
about, it is not always correct to rely on comparing equal elements, meaning that it
bares absolutely no information in it, except that we know the two elements themselves
after looking at them.
```       +
+ + + + +
  p     q
```

suppose we are looking for the max value, and the last index in the array is the ansewer.
Here is the problem. We choose two equidistant elements (from each other and form the end
points). They compare equal. According to the ternary search procedure, we would shrink
the array range being searched to be [p, q]. But the max value index is the last one, so
we would miss it and return the wrong value.

The same argument can be extended.

Theorem. We cannot an algorithm with worst time complexity better than O(N).

Proof. Suppose we do. Then we have an iterating procedure that looks at some constant
number of elements k, with k > 1. If k == 1, we cannot use the relation between the
numbers, and therefore the only operation we can do is to look at each of them, which
will inevitably take O(N) time. So let's say we have k > 1 (we are comparing the elements
to each other). Suppose every of the k elements compare equal. Apply the previous lemma
here and we will get the conclusion that we cannot shrink the range of search in any way,
other than to exclude those k elements from it. Which will inevitably lead to O(N)
algorithm, which contadicts to the conjecture.