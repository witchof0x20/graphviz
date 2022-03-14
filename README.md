# graphviz

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
