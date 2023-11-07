# baseten-rust
This is a simple Rust-based API for http://baseten.co.  There are three functions (really there are only two API endpoints in Baseten, anyhow)

```rust
call_model(model_id:&String, prompt:&String, opt_args:Option<HashMap<String, String>>)
```

```rust
wake(deployment_id:&String)
```

```rust
get_api_key()
```

# Example

```rust
let baseten = Baseten {
  api_key: api_key.to_string()
};

let prompt = String::from("A tree in a field under the night sky");

let mut opts = HashMap::new();
opts.insert(String::from("use_refiner"), String::from("true"));

let r: Result<String, Box<dyn Error>> = block_on(baseten.call_model_prompt(&model, &prompt, Some(opts)));
match r {
  Ok(s) => println!("Returned: {}", s),
  Err(e) => println!("Error: {}", e),
}
```

For a dedicated full working example, see https://github.com/tullytim/baseten-rust-test
You can invoke it and see example output from Stable Diffusion XL with the following (on your Mac only):
```
cargo run <api key> <model id> |  jq -r '.model_output.data' | base64 -D | open -a Preview -f
```

# Building
```rust cargo build ```

# Tests
``` cargo test ```

# Contributing
All contributions welcome but must have tests. Build and test instructions are above, and the most simple Rust oriented way to do things.  Feel free to ping me: tim@menlovc.com
