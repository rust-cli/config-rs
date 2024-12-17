# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

### Compatibility

- `ConfigError` is no longer exhaustive
- `ConfigError::Parse`s type changed
- `FileFormat` is no longer exhaustive
- Deprecated functions removed
- Case insensitivity from 0.14.0 was reverted

### Fixes

- Struct fields, variants, and lookup paths with uppercase letters can be used again (regression from 0.14.0)

## [0.14.1] - 2024-10-23

### Compatibility

- MSRV is now 1.75

### Internal

- Remove dependency on `lazy_static`
- Update rust-ini from 0.19 to 0.20
- Switch from yaml-rust to yaml-rust2

## 0.14.0 - 2024-02-01

This is a maintenance release, mostly to get some dependency updates out, but
also with some fixes and changes that have piled up over a rather long time.

### Features

 - [#100] Fix #99: expose file::source::FileSource
 - [#318] Add Value::origin()
 - [#379] Add clone to builder state types
 - [#381] env: add a 'convert_case' field to ease dealing with kebab-case
 - [#402] Gate async-traits behind a feature
 - [#413] Attach key to type error generated from Config::get_<type>()
 - [#480] Hide and deprecate AsyncConfigBuilder

### Fixes

 - [#306] fix: dot in config name
 - [#334] errors: clarify names of integer types
 - [#343] fix yaml to parse int keys
 - [#353] Use TryInto for more permissive deserialization for integers
 - [#354] Fix uppercase lowercase issues
 - [#374] Fix FIXME in de.rs and value.rs
 - [#395] Fix: Do not use deprecated function
 - [#429] Make the parse list key to lowercase when insert the keys
 - [#465] Fix nested arrays (by reworking array handling)
 - [#481] Fix docs warnings

### Dependency updates

 - [#271] Update warp requirement from =0.3.1 to =0.3.2
 - [#316] test: Adopt test-env to fix random test failure
 - [#335] Update MSRV: 1.56.0 -> 1.56.1
 - [#350] Bump actions/checkout from 2.4.0 to 3.0.2
 - [#367] Update ron requirement from 0.7 to 0.8
 - [#373] Update notify (4.0.0 -> 5.0.0)
 - [#375] Update temp-env requirement from 0.2.0 to 0.3.0
 - [#378] Update warp requirement from =0.3.2 to =0.3.3
 - [#382] Bump actions/checkout from 3.0.2 to 3.1.0
 - [#389] Update MSRV: 1.56.1 -> 1.59.0
 - [#403] Bump actions/checkout from 3.1.0 to 3.2.0
 - [#411] Bump actions/checkout from 3.2.0 to 3.3.0
 - [#420] Update MSRV: 1.59.0 -> 1.60.0
 - [#421] Update toml requirement from 0.5 to 0.7
 - [#422] Update MSRV in cron workflow
 - [#425] Bump actions/checkout from 3.3.0 to 3.4.0
 - [#426] Update MSRV: 1.60.0 -> 1.64.0
 - [#427] Bump actions/checkout from 3.4.0 to 3.5.0
 - [#430] Update warp requirement from =0.3.3 to =0.3.4
 - [#433] Bump actions/checkout from 3.5.0 to 3.5.1
 - [#434] Bump actions/checkout from 3.5.1 to 3.5.2
 - [#436] Update warp requirement from =0.3.4 to =0.3.5
 - [#437] Update rust-ini requirement from 0.18 to 0.19
 - [#438] Update notify requirement from ^5.0.0 to ^6.0.0
 - [#440] Bump actions/checkout from 3.5.2 to 3.5.3
 - [#441] Update indexmap requirement from 1.7.0 to 2.0.0
 - [#451] Bump actions/checkout from 3.5.3 to 3.6.0
 - [#453] Bump actions/checkout from 3.6.0 to 4.0.0
 - [#455] MSRV: 1.64.0 -> 1.66.0
 - [#456] Update toml requirement from 0.7 to 0.8
 - [#458] Update MSRV in cron job
 - [#459] Bump actions/checkout from 4.0.0 to 4.1.0
 - [#462] Update warp requirement from =0.3.5 to =0.3.6
 - [#477] Bump actions/checkout from 4.1.0 to 4.1.1
 - [#483] Update MSRV: 1.66.0 -> 1.70.0
 - [#503] Bump actions/setup-python from 4 to 5

#### Misc

 - [#188] Add test for log::Level deserialization
 - [#274] move 'must_use' attribute to struct for 'builder' types
 - [#283] Add cron job
 - [#299] docs(builder): fix typo in doc comment
 - [#344] Fix clippy: Derive Eq as well
 - [#347] Fix clippy: use first() instead of get(0)
 - [#348] actions: Remove "minimal" setting, as workflow does not support this key
 - [#356] refactoring deserialize-any in config
 - [#359] Add test to deserialize unsigned int
 - [#360] Duplicate test for type conversion with unsigned int
 - [#362] Run clippy only on MSRV
 - [#363] Backport CHANGELOG entry for 0.13.2
 - [#388] Add documentation to File required setter
 - [#392] Add simple example using lazy_static
 - [#393] More clippy fixes
 - [#396] Replace actions rs
 - [#401] Backport changelog
 - [#404] Replace fixupmerge with gitlint
 - [#406] Fix clippy: Remove unnecessary cast
 - [#410] Copy member docs to builder functions
 - [#416] Replace actions-rs with run scripts
 - [#423] Fix clippy: Remove needless borrowed reference
 - [#445] Update license field following SPDX 2.1 license expression standard
 - [#460] Use weak features for preserve_order
 - [#469] chore: Use a common method in parsers to check root is a table
 - [#471] Clippy exact toolchains
 - [#479] docs: Example for conditionally loading sources
 - [#485] Add DCO
 - [#488] Unify deser impl (redux)
 - [#489] deserialize: strings: Introduce string_serialize_via_display macro
 - [#507] Check external types
 - [#511] Fix: cargo-check-external-types must use nightly 2023-10-10

[#100]: https://github.com/mehcode/config-rs/pull/100
[#188]: https://github.com/mehcode/config-rs/pull/188
[#271]: https://github.com/mehcode/config-rs/pull/271
[#274]: https://github.com/mehcode/config-rs/pull/274
[#283]: https://github.com/mehcode/config-rs/pull/283
[#299]: https://github.com/mehcode/config-rs/pull/299
[#306]: https://github.com/mehcode/config-rs/pull/306
[#316]: https://github.com/mehcode/config-rs/pull/316
[#318]: https://github.com/mehcode/config-rs/pull/318
[#334]: https://github.com/mehcode/config-rs/pull/334
[#335]: https://github.com/mehcode/config-rs/pull/335
[#343]: https://github.com/mehcode/config-rs/pull/343
[#344]: https://github.com/mehcode/config-rs/pull/344
[#347]: https://github.com/mehcode/config-rs/pull/347
[#348]: https://github.com/mehcode/config-rs/pull/348
[#350]: https://github.com/mehcode/config-rs/pull/350
[#353]: https://github.com/mehcode/config-rs/pull/353
[#354]: https://github.com/mehcode/config-rs/pull/354
[#356]: https://github.com/mehcode/config-rs/pull/356
[#359]: https://github.com/mehcode/config-rs/pull/359
[#360]: https://github.com/mehcode/config-rs/pull/360
[#362]: https://github.com/mehcode/config-rs/pull/362
[#363]: https://github.com/mehcode/config-rs/pull/363
[#367]: https://github.com/mehcode/config-rs/pull/367
[#373]: https://github.com/mehcode/config-rs/pull/373
[#374]: https://github.com/mehcode/config-rs/pull/374
[#375]: https://github.com/mehcode/config-rs/pull/375
[#378]: https://github.com/mehcode/config-rs/pull/378
[#379]: https://github.com/mehcode/config-rs/pull/379
[#381]: https://github.com/mehcode/config-rs/pull/381
[#382]: https://github.com/mehcode/config-rs/pull/382
[#388]: https://github.com/mehcode/config-rs/pull/388
[#389]: https://github.com/mehcode/config-rs/pull/389
[#392]: https://github.com/mehcode/config-rs/pull/392
[#393]: https://github.com/mehcode/config-rs/pull/393
[#395]: https://github.com/mehcode/config-rs/pull/395
[#396]: https://github.com/mehcode/config-rs/pull/396
[#401]: https://github.com/mehcode/config-rs/pull/401
[#402]: https://github.com/mehcode/config-rs/pull/402
[#403]: https://github.com/mehcode/config-rs/pull/403
[#404]: https://github.com/mehcode/config-rs/pull/404
[#406]: https://github.com/mehcode/config-rs/pull/406
[#410]: https://github.com/mehcode/config-rs/pull/410
[#411]: https://github.com/mehcode/config-rs/pull/411
[#413]: https://github.com/mehcode/config-rs/pull/413
[#416]: https://github.com/mehcode/config-rs/pull/416
[#420]: https://github.com/mehcode/config-rs/pull/420
[#421]: https://github.com/mehcode/config-rs/pull/421
[#422]: https://github.com/mehcode/config-rs/pull/422
[#423]: https://github.com/mehcode/config-rs/pull/423
[#425]: https://github.com/mehcode/config-rs/pull/425
[#426]: https://github.com/mehcode/config-rs/pull/426
[#427]: https://github.com/mehcode/config-rs/pull/427
[#429]: https://github.com/mehcode/config-rs/pull/429
[#430]: https://github.com/mehcode/config-rs/pull/430
[#433]: https://github.com/mehcode/config-rs/pull/433
[#434]: https://github.com/mehcode/config-rs/pull/434
[#436]: https://github.com/mehcode/config-rs/pull/436
[#437]: https://github.com/mehcode/config-rs/pull/437
[#438]: https://github.com/mehcode/config-rs/pull/438
[#440]: https://github.com/mehcode/config-rs/pull/440
[#441]: https://github.com/mehcode/config-rs/pull/441
[#445]: https://github.com/mehcode/config-rs/pull/445
[#451]: https://github.com/mehcode/config-rs/pull/451
[#453]: https://github.com/mehcode/config-rs/pull/453
[#455]: https://github.com/mehcode/config-rs/pull/455
[#456]: https://github.com/mehcode/config-rs/pull/456
[#458]: https://github.com/mehcode/config-rs/pull/458
[#459]: https://github.com/mehcode/config-rs/pull/459
[#460]: https://github.com/mehcode/config-rs/pull/460
[#462]: https://github.com/mehcode/config-rs/pull/462
[#465]: https://github.com/mehcode/config-rs/pull/465
[#469]: https://github.com/mehcode/config-rs/pull/469
[#471]: https://github.com/mehcode/config-rs/pull/471
[#477]: https://github.com/mehcode/config-rs/pull/477
[#479]: https://github.com/mehcode/config-rs/pull/479
[#480]: https://github.com/mehcode/config-rs/pull/480
[#481]: https://github.com/mehcode/config-rs/pull/481
[#483]: https://github.com/mehcode/config-rs/pull/483
[#485]: https://github.com/mehcode/config-rs/pull/485
[#488]: https://github.com/mehcode/config-rs/pull/488
[#489]: https://github.com/mehcode/config-rs/pull/489
[#503]: https://github.com/mehcode/config-rs/pull/503
[#507]: https://github.com/mehcode/config-rs/pull/507
[#511]: https://github.com/mehcode/config-rs/pull/511


## 0.13.3 - 2022-12-04

Please note that we had to update the MSRV for this crate from 1.56.0 to 1.59.0
for this patch release being possible, because a transitive dependency did
update its MSRV.

 - Backport of commit [d54986c54091e4620c199d3dfadde80b82958bb3] from [#362] for
   using float_cmp for testing floats
 - Backport of [#379] adding `Clone` trait derive to builder states

[d54986c54091e4620c199d3dfadde80b82958bb3]: https://github.com/mehcode/config-rs/commit/d54986c54091e4620c199d3dfadde80b82958bb3
[#362]: https://github.com/mehcode/config-rs/pull/362
[#379]: https://github.com/mehcode/config-rs/pull/379

## 0.13.2 - 2022-08-02

 - Backport of [#316] to be testing with temp_env. The backport was necessary to
   be able to backport the next change. This change shouldn't be user-visible.
 - Backport of [#353] to use TryInto for more permissive deserialization of
   integers
 - Backport of commit [518a3cafa1e62ba7405709e5c508247e328e0a18] from [#362] to
   fix tests

[#316]: https://github.com/mehcode/config-rs/pull/316
[#353]: https://github.com/mehcode/config-rs/pull/353
[518a3cafa1e62ba7405709e5c508247e328e0a18]: https://github.com/mehcode/config-rs/commit/518a3cafa1e62ba7405709e5c508247e328e0a18
[#362]: https://github.com/mehcode/config-rs/pull/362

## 0.13.1 - 2022-04-13

 - typo in doc comment for ConfigBuilder [#299]
 - dot in config file name handling fixed [#306]

[#299]: https://github.com/mehcode/config-rs/pull/299
[#306]: https://github.com/mehcode/config-rs/pull/306

## 0.13.0 - 2022-04-03

 - Prefix-Separator support was added [#292]
 - Environment lists can now be parsed [#255]
 - Setting an overwrite from an Option was added [#303]
 - Option to keep the prefix from an environment variable was added [#298]
 - Some small doc/CI fixes [#307], [#309]
 - MSRV was updated to 1.56.0 [#304]
 - Dependencies were updated [#289], [#301]

[#292]: https://github.com/mehcode/config-rs/pull/292
[#255]: https://github.com/mehcode/config-rs/pull/255
[#303]: https://github.com/mehcode/config-rs/pull/303
[#298]: https://github.com/mehcode/config-rs/pull/298
[#307]: https://github.com/mehcode/config-rs/pull/307
[#309]: https://github.com/mehcode/config-rs/pull/309
[#304]: https://github.com/mehcode/config-rs/pull/304
[#289]: https://github.com/mehcode/config-rs/pull/289
[#301]: https://github.com/mehcode/config-rs/pull/301

## 0.12.0 - 2022-02-10

### Format support changes in this version

 - HJSON support was removed [#230]
 - JSON5 format support [#206]
 - RON format support [#202]

### Other noteworthy changes

 - A new ConfigBuilder interface for building configuration objects [#196]
 - Asynchronous sources [#207]
 - Custom ENV separators are now supported [#185]
 - Loads of dependency updates and bugfixes of course
 - Preserved map order [#217]
 - Support for parsing numbers from the environment [#137]
 - Support for unsigned integers [#178]
 - `Format` trait for (custom) file formats [#219]

### Deprecated

 - `Environment::new()` - see [#235]
 - Large parts of the `Config` interface - see [#196]
     - `Config::merge()`
     - `Config::with_merged()`
     - `Config::refresh()`
     - `Config::set_default()`
     - `Config::set()`
     - `Config::set_once()`
     - `Config::deserialize()`

[#137]: https://github.com/mehcode/config-rs/pull/137
[#178]: https://github.com/mehcode/config-rs/pull/178
[#185]: https://github.com/mehcode/config-rs/pull/185
[#196]: https://github.com/mehcode/config-rs/pull/196
[#202]: https://github.com/mehcode/config-rs/pull/202
[#206]: https://github.com/mehcode/config-rs/pull/206
[#207]: https://github.com/mehcode/config-rs/pull/207
[#217]: https://github.com/mehcode/config-rs/pull/217
[#219]: https://github.com/mehcode/config-rs/pull/219
[#230]: https://github.com/mehcode/config-rs/pull/230
[#235]: https://github.com/mehcode/config-rs/pull/235

## 0.11.0 - 2021-03-17
 - The `Config` type got a builder-pattern `with_merged()` method [#166].
 - A `Config::set_once()` function was added, to set an value that can be
   overwritten by `Config::merge`ing another configuration [#172]
 - serde_hjson is, if enabled, pulled in without default features.
   This is due to a bug in serde_hjson, see [#169] for more information.
 - Testing is done on github actions [#175]

[#166]: https://github.com/mehcode/config-rs/pull/166
[#172]: https://github.com/mehcode/config-rs/pull/172
[#169]: https://github.com/mehcode/config-rs/pull/169
[#175]: https://github.com/mehcode/config-rs/pull/169

## 0.10.1 - 2019-12-07
 - Allow enums as configuration keys [#119]

[#119]: https://github.com/mehcode/config-rs/pull/119

## 0.10.0 - 2019-12-07
 - Remove lowercasing of keys (unless the key is coming from an environment variable).
 - Update nom to 5.x

## 0.9.3 - 2019-05-09
 - Support deserializing to a struct with `#[serde(default)]` [#106]

[#106]: https://github.com/mehcode/config-rs/pull/106

## 0.9.2 - 2019-01-03
 - Support reading `enum`s from configuration. [#85]
 - Improvements to error path (attempting to propagate path). [#89]
 - Fix UB in monomorphic expansion. We weren't re-exporting dependent types. [#91]

[#85]: https://github.com/mehcode/config-rs/pull/85
[#89]: https://github.com/mehcode/config-rs/pull/89
[#91]: https://github.com/mehcode/config-rs/issues/91

## 0.9.1 - 2018-09-25
 - Allow Environment variable collection to ignore empty values. [#78]
   ```rust
   // Empty env variables will not be collected
   Environment::with_prefix("APP").ignore_empty(true)
   ```

[#78]: https://github.com/mehcode/config-rs/pull/78

## 0.9.0 - 2018-07-02
 - **Breaking Change:** Environment does not declare a separator by default.
    ```rust
    // 0.8.0
    Environment::with_prefix("APP")

    // 0.9.0
    Environment::with_prefix("APP").separator("_")
    ```

 - Add support for INI. [#72]
 - Add support for newtype structs. [#71]
 - Fix bug with array set by path. [#69]
 - Update to nom 4. [#63]

[#72]: https://github.com/mehcode/config-rs/pull/72
[#71]: https://github.com/mehcode/config-rs/pull/71
[#69]: https://github.com/mehcode/config-rs/pull/69
[#63]: https://github.com/mehcode/config-rs/pull/63

## 0.8.0 - 2018-01-26
 - Update lazy_static and yaml_rust

## 0.7.1 - 2018-01-26
 - Be compatible with nom's verbose_errors feature (#50)[https://github.com/mehcode/config-rs/pull/50]
 - Add `derive(PartialEq)` for Value (#54)[https://github.com/mehcode/config-rs/pull/54]

## 0.7.0 - 2017-08-05
 - Fix conflict with `serde_yaml`. [#39]

[#39]: https://github.com/mehcode/config-rs/issues/39

 - Implement `Source` for `Config`.
 - Implement `serde::de::Deserializer` for `Config`. `my_config.deserialize` may now be called as either `Deserialize::deserialize(my_config)` or `my_config.try_into()`.
 - Remove `ConfigResult`. The builder pattern requires either `.try_into` as the final step _or_ the initial `Config::new()` to be bound to a slot. Errors must also be handled on each call instead of at the end of the chain.


    ```rust
    let mut c = Config::new();
    c
        .merge(File::with_name("Settings")).unwrap()
        .merge(Environment::with_prefix("APP")).unwrap();
    ```

    ```rust
    let c = Config::new()
        .merge(File::with_name("Settings")).unwrap()
        .merge(Environment::with_prefix("APP")).unwrap()
        // LLVM should be smart enough to remove the actual clone operation
        // as you are cloning a temporary that is dropped at the same time
        .clone();
    ```

    ```rust
    let mut s: Settings = Config::new()
        .merge(File::with_name("Settings")).unwrap()
        .merge(Environment::with_prefix("APP")).unwrap()
        .try_into();
    ```

## 0.6.0 – 2017-06-22
  - Implement `Source` for `Vec<T: Source>` and `Vec<Box<Source>>`

    ```rust
    Config::new()
        .merge(vec![
            File::with_name("config/default"),
            File::with_name(&format!("config/{}", run_mode)),
        ])
    ```

  - Implement `From<&Path>` and `From<PathBuf>` for `File`

  - Remove `namespace` option for File
  - Add builder pattern to condense configuration

    ```rust
    Config::new()
        .merge(File::with_name("Settings"))
        .merge(Environment::with_prefix("APP"))
        .unwrap()
    ```

 - Parsing errors even for non required files – [@Anthony25] ( [#33] )

[@Anthony25]: https://github.com/Anthony25
[#33]: https://github.com/mehcode/config-rs/pull/33

## 0.5.1 – 2017-06-16
 - Added config category to Cargo.toml

## 0.5.0 – 2017-06-16
 - `config.get` has been changed to take a type parameter and to deserialize into that type using serde. Old behavior (get a value variant) can be used by passing `config::Value` as the type parameter: `my_config.get::<config::Value>("..")`. Some great help here from [@impowski] in [#25].
 - Propagate parse and type errors through the deep merge (remembering filename, line, etc.)
 - Remove directory traversal on `File`. This is likely temporary. I do _want_ this behavior but I can see how it should be optional. See [#35]
 - Add `File::with_name` to get automatic file format detection instead of manual `FileFormat::*` – [@JordiPolo]
 - Case normalization [#26]
 - Remove many possible panics [#8]
 - `my_config.refresh()` will do a full re-read from the source so live configuration is possible with some work to watch the file

[#8]: https://github.com/mehcode/config-rs/issues/8
[#35]: https://github.com/mehcode/config-rs/pull/35
[#26]: https://github.com/mehcode/config-rs/pull/26
[#25]: https://github.com/mehcode/config-rs/pull/25

[@impowski]: https://github.com/impowski
[@JordiPolo]: https://github.com/JordiPolo

## 0.4.0 - 2017-02-12
 - Remove global ( `config::get` ) API — It's now required to create a local configuration instance with `config::Config::new()` first.

   If you'd like to have a global configuration instance, use `lazy_static!` as follows:

   ```rust
   use std::sync::RwLock;
   use config::Config;

   lazy_static! {
       static ref CONFIG: RwLock<Config> = Default::default();
   }
   ```

## 0.3.0 - 2017-02-08
 - YAML from [@tmccombs](https://github.com/tmccombs)
 - Nested field retrieval
 - Deep merging of sources (was shallow)
 - `config::File::from_str` to parse and merge a file from a string
 - Support for retrieval of maps and slices — `config::get_table` and `config::get_array`

## 0.2.0 - 2017-01-29
Initial release.

<!-- next-url -->
[Unreleased]: https://github.com/rust-ci/config-rs/compare/v0.14.1...HEAD
[0.14.1]: https://github.com/rust-cli/config-rs/compare/0.14.0...v0.14.1
