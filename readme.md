### Fast japanese deinflection.

```rust
use deinflect::Deinflections;

fn main() {
    let deinflections = Deinflections::from_word("聞かれました");
    // iterate over all possible deinflections
    for deinflection in deinflections.iter() {
        // get the deinflected word as a string
        let deinflected = deinflections.to_string(deinflection);
        println!("{}", deinflected);
    }
}
```

This library is based on the [yomichan japanese deinflector](https://github.com/FooSoft/yomichan).