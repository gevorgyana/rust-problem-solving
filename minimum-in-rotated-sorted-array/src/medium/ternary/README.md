This is the most straighforward way to solve this problem. Say we have an array that does
not contain any repeating elements and is sorted by increasing order. Then this array is
divided into two halved.

```
Arr = XY
```

Some k from [0, N - 2], where N is the length of the initial array, is chosen, then the
array is split into `X` and `Y`, and the parts are shifted like this and then
concatenated so that we have

```
NewArr = YX
```

Then we have a unimodal funciton with respect to both miminum and maximum. The suggested
algorithm is to use ternary search to find the index that gives the minimum element.

The structure of the array looks like this.

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

To find the index of `g` - the minimal value, we can maintain the two pointers to the
indices which are equally distant from the edge elements and from each other (in other
words, we split the array in 3 parts by these 2 integer indices). Then we compare those,
which may give three possible outcomes:

1) The elements are equal -> this is really not possible, as the problem statement
explicitly talks about there not being any repetition of a single value in the given
array.

2) The left index is greater than the right one -> then the right index is actually
closer to the minimum point than the left one, but there are 2 options - it may be to the
left from the minimum or to the right from it, so the minimum value may be located right
in between the two chosen indices, or to the right from the rightmost one. Therefore we
choose the left edge of the search to be shifter to where the leftmost pointer currently
is.

3) The left index is less than the right one -> symmetric to the prev. case.

Once we hit a range of a small constant, like 2, we can choose the min value from them.