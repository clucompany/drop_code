<div id="header" align="center">

  <b>[drop_code]</b>
  
  (Macro for ensuring critical code execution on function return or panics in Rust, making it easy to include essential code for reliable operation.)
  </br></br>

<div id="badges">
  <a href="./LICENSE_APACHE">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/apache2.png?raw=true" alt="apache2"/>
  </a>
  <a href="https://crates.io/crates/include_tt">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/cratesio.png?raw=true" alt="cratesio"/>
  </a>
  <a href="https://docs.rs/include_tt">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/docrs.png?raw=true" alt="docrs"/>
  </a>
  <a href="https://github.com/denisandroid">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/uproject.png?raw=true" alt="uproject"/>
  </a>
  <a href="https://github.com/clucompany">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/clulab.png?raw=true" alt="clulab"/>
  </a>
	
  [![CI](https://github.com/clucompany/include_tt/actions/workflows/CI.yml/badge.svg?event=push)](https://github.com/clucompany/include_tt/actions/workflows/CI.yml) 


</div>
</div>

## Usage:

Add this to your Cargo.toml:

```toml
[dependencies]
drop_code  = "0.1.1"
```

and this to your source code:
```rust
use drop_code ::drop_code;
```
