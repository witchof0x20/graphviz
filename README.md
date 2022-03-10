# graphviz

# How to use
Add to `Cargo.toml`
```toml
[dependencies]
graphviz = {git = https://github.com/witchof0x20/graphviz}
```
`example.rs`:
```rust
fn main() {
  let graph = graphviz::parser::GraphParser::new().parse(r"
    digraph G {
      A -> B;
      B -> C;
      C -> D -> subgraph E { EA -> EB -> EC -> EA } -> A;
    }"
  )?;
  dbg!(graph);
}
```
