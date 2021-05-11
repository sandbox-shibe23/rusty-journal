# rusty-journal
[Rust の最初のステップ](https://docs.microsoft.com/ja-jp/learn/paths/rust-first-steps/)

## How to Use

```
cargo run -- -j test-journal.json add "buy milk"

cargo run -- -j test-journal.json add "take the dog for a walk"

cargo run -- -j test-journal.json add "water the plants"

cargo run -- -j test-journal.json list
1: buy milk                                           [2021-01-08 16:39]
2: take the dog for a walk                            [2021-01-08 16:39]
3: water the plants                                   [2021-01-08 16:39]

cargo run -- -j test-journal.json done 2

cargo run -- -j test-journal.json list
1: buy milk                                           [2021-01-08 16:39]
2: water the plants                                   [2021-01-08 16:39]
```

## Release

```
cargo run --release
```
