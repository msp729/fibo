fibonacci numbers, done fast.


### recursive algorithm
- f(0) = 0
- f(1) = 1
- f(n) = f(n-1) + f(n-2)

### guarded recursion
a little more sophisticated, achieves much better performance.
- f(0) = 0
- f(1) = 1
- f(2) = 1
- f(10) = 55
- f(20) = 6765
- f(n) = 2\*f(n-2) + f(n-3)

### the iterative (also linear) algorithm
initialize with a=0, b=1
iteratively do a,b -> b,a+b

### matrix algorithm
S = [1 1; 1 0]
is the step matrix; S \* [1;0] = [1; 1], S \* [5; 3] = [8; 5], etc.

By taking powers of that matrix, we can get larger Fibonacci numbers faster.


### advanced matrix algorithm
observe that S² = S + I, so all powers of S can be written as linear combinations of S and I.

multiply them with less computation and less storage.

also note that the powers of S always are stored (in this encoding) as adjacent fibonacci numbers, so this is really just a way to take the n'th pair of fibonacci numbers and procure the 2n'th.
