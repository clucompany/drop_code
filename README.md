<div id="header" align="center">

  <b>[drop_code]</b>
  
  (Macro for ensuring critical code execution on function return or panics in Rust, making it easy to include essential code for reliable operation.)
  </br></br>

<div id="badges">
  <a href="./LICENSE_APACHE">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/apache2.png?raw=true" alt="apache2"/>
  </a>
  <a href="https://crates.io/crates/drop_code">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/cratesio.png?raw=true" alt="cratesio"/>
  </a>
  <a href="https://docs.rs/drop_code">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/docrs.png?raw=true" alt="docrs"/>
  </a>
  <a href="https://github.com/denisandroid">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/uproject.png?raw=true" alt="uproject"/>
  </a>
  <a href="https://github.com/clucompany">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/clulab.png?raw=true" alt="clulab"/>
  </a>
	
  [![CI](https://github.com/clucompany/drop_code/actions/workflows/CI.yml/badge.svg?event=push)](https://github.com/clucompany/drop_code/actions/workflows/CI.yml) 


</div>
</div>

## Usage:

Add this to your Cargo.toml:

```toml
[dependencies]
drop_code  = "1.0.0"
```

and this to your source code:
```rust
use drop_code::drop_code;
```

## Technical concept: 
The drop_code macro generates the drop function code for the Drop trait in Rust, creating a hidden structure that encapsulates user-supplied arguments from the macro and ensuring their transfer. This mechanism guarantees the execution of critical operations when the object is destroyed, facilitating reliable handling of essential code within Rust applications, with the order of code execution dictated by Rust's rules and conventions.

## Example:

```rust
use drop_code::drop_code;

#[allow(unreachable_code)]
fn main() {
	drop_code! {
		println!("Code that must be executed in any situation."); // 3
	}
	
	println!("your code"); // 1
	panic!("panic info"); // 2
}
```

<a href="./examples">
  See all
</a>

## License:
This project has a single license (LICENSE-APACHE-2.0).

<div align="left">
  <a href="https://github.com/denisandroid">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/uproject.png?raw=true" alt="uproject"/>
  </a>
  <b>&nbsp;Copyright (c) 2022-2024 #UlinProject</b>
	
  <b>&nbsp;(Denis Kotlyarov).</b>
  </br></br></br>
</div>

### Apache License:
<div align="left">
  <a href="./LICENSE_APACHE">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/apache2.png?raw=true" alt="apache2"/>
    
  </a>
  <b>&nbsp;Licensed under the Apache License, Version 2.0.</b>
  </br></br></br></br>
</div>
