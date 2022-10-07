![Build](https://github.com/a1akris/overwrite/actions/workflows/build.yml/badge.svg)

# Overwrite
A simple trait that defines how to overwrite a type by another types.
Mainly useful to create an app configuration from different sources.

### Example

```rust
use overwrite::Overwrite;

struct Config {
    url: String,
    user: String,
    api_token: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            url: "default".to_owned(),
            user: "default".to_owned(),
            api_token: "default".to_owned(),
        }
    }
}

struct CliArgs {
    url: Option<String>,
    user: Option<String>,
    api_token: Option<String>
}

impl Overwrite<CliArgs> for Config {
    fn overwrite_mut(&mut self, cli_args: CliArgs) -> &mut Self {
        // There is a blanket impl to overwrite values with Options.
        // Overwrite happens if the Option is Some.
        self.url.overwrite_mut(cli_args.url);
        self.user.overwrite_mut(cli_args.user);
        self.api_token.overwrite_mut(cli_args.api_token);
        self
    }
}

struct ConfFile {
    url: Option<String>,
    user: Option<String>,
    api_token: Option<String>,
}

impl Overwrite<ConfFile> for Config {
    fn overwrite_mut(&mut self, conf_file: ConfFile) -> &mut Self {
        self.url.overwrite_mut(conf_file.url);
        self.user.overwrite_mut(conf_file.user);
        self.api_token.overwrite_mut(conf_file.api_token);
        self
    }
}

struct EnvVars {
    url: Option<String>,
    user: Option<String>,
    api_token: Option<String>,
}

impl Overwrite<EnvVars> for Config {
    fn overwrite_mut(&mut self, env_vars: EnvVars) -> &mut Self {
        self.url.overwrite_mut(env_vars.url);
        self.user.overwrite_mut(env_vars.user);
        self.api_token.overwrite_mut(env_vars.api_token);
        self
    }
}

fn main() {
    let cli_args = parse_args();
    let conf_file = read_conf_file();
    let env_vars = parse_env();

    // Note the precedense. `cli_args` have the highest precedense while default config values
    // have the lowest precedense. `conf_file` values will be overwritten by `env_vars` if env
    // vars are present.
    let config = Config::default()
        .overwrite(conf_file)
        .overwrite(env_vars)
        .overwrite(cli_args);

    assert_eq!(config.url, "default");
    assert_eq!(config.user, "from_env_vars");
    assert_eq!(config.api_token, "from_cli_args");
}

fn parse_args() -> CliArgs {
    CliArgs {
        url: None,
        user: None,
        api_token: Some("from_cli_args".to_owned()),
    }
}

fn parse_env() -> EnvVars {
    EnvVars {
        url: None,
        user: Some("from_env_vars".to_owned()),
        api_token: None,
    }
}

fn read_conf_file() -> ConfFile {
    ConfFile {
        url: None,
        user: Some("from_conf_file".to_owned()),
        api_token: Some("from_conf_file".to_owned()),
    }
}
```

As you have noticed the trait implementation is quite repetetive. There will be
a proc macro to automatically derive it in the future versions.

#### License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT
license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
