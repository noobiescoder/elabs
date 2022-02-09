# Elabs Solc
Elabs Solc is a simple rust wrapper for solc(c++).

## Usage
```toml
[dependencies]
elabs-solc = "0.1.0"
```

```rust
use elabs_solc::Solc;

fn main() {
	let solc = Solc::new();
	let input_path = "contracts/Simple.sol";
	let output_path = "artifacts";
	match solc.compile(input_path, output_path, vec![]) {
		Ok(_) => println!("{} compiled", input_path),
		Err(e) => panic!("{}", e),
	}
}
```
