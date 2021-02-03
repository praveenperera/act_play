Playing with the awesome [act-zero](https://github.com/Diggsey/act-zero) crate.

- [ShortGreeter (no long await points)](#shortgreeter-no-long-await-points)
  - [Input](#input)
  - [Output](#output)
- [LongGreeter (long await points)](#longgreeter-long-await-points)
  - [Input](#input-1)
  - [Output](#output-1)
- [GreeterPool (long await with Actor pool)](#greeterpool-long-await-with-actor-pool)
  - [Input](#input-2)
  - [Output](#output-2)

## ShortGreeter (no long await points)

`cd short_greeter && cargo run`

### Input

```bash
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

## LongGreeter (long await points)

`cd long_greeter && cargo run`

Demonstrates that:

1. When borrowing actor state, an actor can only handle one message at a time. Therefore you should
   avoid awaiting across long await points. Instead you should use either `send_fut`, `send_fut_with`,
   `call_fut` or `call_fut_with` functions

2. The messages don't get lost, they get queued up and processed one at a time

### Input

```bash
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

## GreeterPool (long await with Actor pool)

`cd greeter_pool && cargo run`

This example is similar to the `LongGreeter` example in that each greet takes 1 second. However, instead of 1 actor handling all the messages a pool of 5 actors are created.

They act as a (FIFO) queue. As each greeter is used its added to the back of the queue.

The result is that 5 messages are handled immediately, then there is a pause and finally the last messages are handled as each greeter becomes available

### Input

```bash
for i in {1..10}
do
 curl localhost:8082
done
```

### Output

```
GreeterPool/Long Greeter: Number 1, Since Start: 1161ms
GreeterPool/Long Greeter: Number 2, Since Start: 1167ms
GreeterPool/Long Greeter: Number 3, Since Start: 1174ms
GreeterPool/Long Greeter: Number 4, Since Start: 1181ms
GreeterPool/Long Greeter: Number 5, Since Start: 1187ms
GreeterPool/Long Greeter: Number 6, Since Start: 2163ms
GreeterPool/Long Greeter: Number 7, Since Start: 2169ms
GreeterPool/Long Greeter: Number 8, Since Start: 2175ms
GreeterPool/Long Greeter: Number 9, Since Start: 2182ms
GreeterPool/Long Greeter: Number 10, Since Start: 2189ms
```
