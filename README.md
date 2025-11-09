# disarium number finder

Rust program to find all 20 disarium numbers in under 30 seconds.

```
0
1
2
3
4
5
6
7
8
9
89
135
175
518
598
1306
1676
2427
2646798
12157692622039623539
found numbers in 12423617458 ns, or 12423 ms, or 12 secs
```

A disarium number is a number where the sum of the digits exponentiated to their position in the number equals the number. For example, 135 is a disarium number because $1^1 + 3^3 + 5^5 = 135$.

There are a finite number of disarium numbers, because once you reach 23 digits, the largest possible position-exponentiation-sum only has 22 digits:

```
99999999999999999999999 <- 23 digit number with max digit sizes
9960805384609063732919  <- position-exponentiation-sum of previous number
```

This digit difference means that there cannot exist any disarium numbers with more than 22 digits because all position-exponentiation-sums will be too small.

The above output came from checking all `u64`s. However, theoretically, there could exist a number between 18446744073709551616 ($2^{64}$) and 9999999999999999999999 (22 digits of 9). Thus, to thoroughly check for all disarium numbers, I've also run the program with number type `u128` with an upper bound of 10000000000000000000000 (23 digits).

```
0
1
2
3
4
5
6
7
8
9
89
135
175
518
598
1306
1676
2427
2646798
12157692622039623539
found numbers in 683032245750 ns, or 683032 ms, or 683 secs
```

Thus, we successfully confirm that there are only 20 disarium number.
