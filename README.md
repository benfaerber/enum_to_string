# enum_all_variants

Proc macro for generating impl Display for enums using serde_json

## Example
```rust
use enum_to_string::ToJsonString;
use serde::Serialize;

#[derive(Debug, Serialize, ToJsonString)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Direction {
    Left,
    Top,
    Right,
    Bottom,
}

fn main() {
    println!("{}", Direction::Left.to_string()); 
}
```

Outputs:
```
LEFT
```
