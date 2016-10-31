# cog-rust

Simple, opiniated library for building Cog commands in rust.

## Getting Started

These instructions will get you a copy of the project up and running for use in your own bundles.

### Prerequisities

```toml
# Cargo.toml
[dependencies]
cog = "0.1"
```

### Usage

Write your bundle:

```rust
// main.rs
extern crate cog;

fn main() {
  let res = cog::Bundle::new("my-bundle")
    .command("hello", &hello_world)
    .run()
    .expect("failed to run bundle");
}

// cog::Args is a Vec<String> and cog::Opts a HashMap<String,String>
fn hello_word(args: cog::Args, opts: cog::Opts) {
  match opts.get("name") {
    Some(name) =>
      cog::write(&format!("hello {}", name)),
    None =>
      cog::write("hello world"),
  }
}
```

Use the resulting executable in cog's `config.yaml`:

```yaml
commands:
  hello:
    executable: /usr/local/bin/my-bundle
    documentation: hello [--name=<name>]
    # ...
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us. **So far this file does not exist**, help welcome on how to handle this.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/pauldub/cog-rust/tags).

## Authors

* **Paul d'Hubert** - *Initial work* - [pauldub](https://github.com/pauldub)

See also the list of [contributors](https://github.com/pauldub/cog-rust/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* operable for bringing cog and the nice bundle system!
