# Elabs
Elabs-solc is a wrapper around the Solidity compiler.
It is designed to be used as a library, and not as a command line tool.
It will wrap `solc` cli tools, and provide a simple interface
to compile solidity contracts.
## Usage
To use the library, you need to import it in your project:
```toml
[dependencies]
elabs-solc = "0.1"
```
## Example
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
