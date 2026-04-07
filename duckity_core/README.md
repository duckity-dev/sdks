# Duckity Core

This library contains primitives to decode, solve, and re-encode Duckity challenges.

It provides 3 functions, [`decode(&str)`], [`solve(&Challenge)`], and [`encode(&str, &Solution)`].

```rs
fn main() {
    let token = "<duckity-challenge-token>";
    let challenge = duckity_core::decode(token).unwrap();

    println!("Solving challenge...");
    let solution = duckity_core::solve(&challenge);
    println!("Solved!");

    let solution_token = duckity_core::encode(token, &solution).unwrap();

    println!("{solution_token}");
}
```
