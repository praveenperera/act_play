# ActPlay

Playing with the awesome [act-zero](https://github.com/Diggsey/act-zero) crate.

## Testing LongGreeter:

### Input:

```
 for i in {1..10}
do
 curl localhost:8081
done
```

### Output

```
Long Greeter: Number 1, Since Start: 1772ms
Long Greeter: Number 2, Since Start: 2778ms
Long Greeter: Number 3, Since Start: 3781ms
Long Greeter: Number 4, Since Start: 4787ms
Long Greeter: Number 5, Since Start: 5789ms
Long Greeter: Number 6, Since Start: 6790ms
Long Greeter: Number 7, Since Start: 7797ms
Long Greeter: Number 8, Since Start: 8803ms
Long Greeter: Number 9, Since Start: 9805ms
Long Greeter: Number 10, Since Start: 10807ms
```

Demonstrates that:

1. When borrowing actor state, an actor can only handle one message at a time. Therefore you should
   avoid awaiting across long await points. Instead you should use either `send_fut`, `send_fut_with`,
   `call_fut` or `call_fut_with` functions

2. The messages don't get lost, they get queued up and processed one at a time.

## Testing ShortGreeter:

### Input:

```
for i in {1..10}
do
curl localhost:8080
done
```

### Output

```
Short Greeter: Number 1, Since Start: 13594ms
Short Greeter: Number 2, Since Start: 13598ms
Short Greeter: Number 3, Since Start: 13602ms
Short Greeter: Number 4, Since Start: 13606ms
Short Greeter: Number 5, Since Start: 13611ms
Short Greeter: Number 6, Since Start: 13615ms
Short Greeter: Number 7, Since Start: 13619ms
Short Greeter: Number 8, Since Start: 13623ms
Short Greeter: Number 9, Since Start: 13628ms
Short Greeter: Number 10, Since Start: 13632ms
```
