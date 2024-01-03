# ckb-native-build-sample

A collection of CKB contract samples that can and should be compiled natively using latest stable Rust and LLVM.

**Notice the include contracts are for illustration only, they are NOT secure enough to guard CKB cells. Don't ever use them directly in production.**

## Rationale

The included samples here are put together under the following set of rationale:

* Native, official distribution(without any patches required) of Rust and LLVM must be used to build smart contracts.
    + One might leverage docker to lock Rust & LLVM versions for reproducible build, but normal day-to-day development should be doable only with natively installed Rust & LLVM
* Readable, simple, editable makefiles are augmented to each crate to simply CKB contract building.
    + The include makefiles should work on commonly defined conventions, and a minimal set of features. Tricks should be limited to absolutely minimum.
    + The goal here, is that any developer should be comfortable modifying the makefiles with ease, suiting their special requirements.

## Usage

First, clone the repo with submodules:

```
$ git clone --recursive https://github.com/xxuejie/ckb-native-build-sample
$ cd ckb-native-build-sample
```

### Directory Structure

Generally speaking, the directory structure, is simply a standard Rust workspace with a few added makefiles. However, it is built with some conventions to simpify development tasks:

* `crates`: Platform independent Rust crates for common utilities. Chances are many of them come with their own unit tests that can run on any platforms supported by Rust compiler.
    + `crates/big-cell-fetcher`: A pure Rust crate, what is interesting, is that it leverages [ckb-x64-simulator](https://github.com/nervosnetwork/ckb-x64-simulator) to build native runnable unit tests.
    + `crates/big-witness-hasher`: A sample building and gluing C code in a Rust crate. Going into the future, this is the layout I personally recommended, if you have C code to glue to a Rust-based CKB smart contract.
* `contracts`: Actual CKB contracts go here, it is expected that each contract form its own crate with its own folder here.
    + `contracts/minimal-log`: A minimal contract example that does nothing but prints log lines. This can serve as a template example if one wants to build CKB smart contracts following guidelines shown here.
    + `contracts/loads-of-hashes`: A non-trivial example that loads external Rust-only dependencies, as well as dependency that contains C code but in a proper organization(`crates/big-witness-hasher`).
    + `contracts/legacy-c-dependency`: Every once in a while, you might run into dependency that was from the old time, hence does not respect the conventions here well. Using `blake2b-rs` at `v0.2.0` as such a dependency, this sample shows how you can introduce code from the legacy days to a proper native build setup. One can do a diff between `contracts/legacy-c-dependency/Makefile` and `contracts/minimal-log/Makefile` to learn exactly what is needed to take care of the legacy crates.
    + `contracts/stack-reorder`: An example showcasing how to reorder stack to the lower address, and keep heap at higher address for better memory overflow protection in an absence of MMU. Notice the actual required allocated stack size is depending on individual contracts, so the Makefile included for this contract has an additional task for tweaking allocated stack size. Similarly to the above, one can do a diff between `contracts/stack-reorder/Makefile` and `contracts/minimal-log/Makefile` for all the details.
    + `contracts/atomics-without-a`: Certain Rust code, such as [bytes](https://docs.rs/bytes/latest/bytes/) crate, [log](https://docs.rs/log/latest/log/) crate, or [Box::from_raw](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw) would require certain atomic builtins to be available. Current Rust uses RISC-V's A extension to implement those builtins. However, A extension is not yet available in current CKB. This sample provides a solution to have atomic builtins without A extension being available.
* `deps`: All git submodules should go here.
* `tests`: Top level contract tests. Typically one would want to build full CKB transactions including the smart contracts in development, then run them in CKB's verifier for assurance of behaviors.
* `docker`: Sample docker files for reproducible build. You might or might not need this in your project.
* `Makefile`: Top, workspace level makefile for firing up commands.

Note for the sample here, I am not locking Rust toolchain versions, since I believe latest stable Rust is good enough to build smart contracts. However for individual projects one is working on, it might or might not make sense to lock Rust toolchain version for stability.

### Building

To build the contracts, use the following command:

```
$ make build
```

By default, release builds (with debug assertions) will be generated. You can find the resulting binaries at `build/release` directory.

We can also customize the build process:

```
$ make build MODE=debug                   # for debug build
$ make build CUSTOM_RUSTFLAGS=""          # release build without debug assertions
$ make build CARGO_ARGS="--verbose"       # release build with `--verbose` attached to cargo command, you can use other arguments accepted by cargo
$ make build CONTRACT=minimal-log         # build a single contract
$ make build CLEAN_BUILD_DIR_FIRST=false  # keep old untouched binaries
```

You can also combine all the arguments here, suppose in a previous build you have build all the binaries, now you only want to build minimal-log binary. Doing `make build CONTRACT=minimal-log` will erase other binaries, you can do `make build CONTRACT=minimal-log CLEAN_BUILD_DIR_FIRST=false` to both build the minimal-log binary, and also keep the old ones untouched.

### Reproducible Build

The workspace level makefile has a `checksum` task, which can help you generate a checksum file for reproducible build:

```
$ make checksum
```

By default, this generates a checksum file at `build/checksums-release.txt`, containing the sha256 hash for all generated binaries.

We can leverage this task to build a reproducible workflow. Using docker as an example:

Locate a docker image that has both Rust and LLVM (of your used version) installed. I strongly recommended that one spends some extra time to build his/her own, for maximum security. But if one isn't available or if one wants simple testing, I have one available [here](https://hub.docker.com/r/xxuejie/rust-n-llvm), which is built from `docker/bookworm.dockerfile`.

Now use command like following to build the contract, then obtain the checksum:

```
$ docker run --rm -v `pwd`:/code \
  docker.io/xxuejie/rust-n-llvm@sha256:71e98a25eb0350c779cdea18c296d101c4ddc375b8fd96531b63f3105ca64ca2 \
  bash -c "cd /code; make checksum MODE=release CHECKSUM_FILE=checksums.txt"
```

Notice `CHECKSUM_FILE` is altered so the checksum file is generated outside of `build` directory, this way we can keep it in source control.

With `checksums.txt` checked in, one can then first build the contract, and use the following command to verify the checksums:

```
$ sha256sum -c checksums.txt
build/release/legacy-c-dependency: OK
build/release/loads-of-hashes: OK
build/release/minimal-log: OK
build/release/stack-reorder: OK
```

In case the checksums do not match, errors would occurs:

```
$ sha256sum -c checksums.txt
build/release/legacy-c-dependency: FAILED
build/release/loads-of-hashes: FAILED
build/release/minimal-log: FAILED
build/release/stack-reorder: FAILED
sha256sum: WARNING: 4 computed checksums did NOT match
```

This whole process can be integrated in CI for reproducible build checking.

Notice that using docker to do reproducible build is a method of doing reproducible build, it is NOT THE method to do reproducible build. Fundamentally, the requirements of reproducible build, is simply the same source code + the same compiler toolchain. There might well be many other methods that can be used to achieve the goal of reproducible build. Here I'm simply illustrating one example of doing it.

### Testing

Tests is supported in different levels:

* Rust crates can have their own unit tests, see `crates/big-cell-fetcher` for an example.
* Individual contracts are also organized as plain Rust crates, they can have unit tests in their own crates as well. See `contracts/loads-of-hashes` for an example.
* At workspace level, we also have a crate for higher level testing, see `tests` for details.

As all Rust crates here are organized in a unified workspace, a single command can fire up all the tests here:

```
$ make test
```

Notice this is a simple wrapper of cargo command, so there nothing stopping you from running:

```
$ cargo test
```

To further refine the testing process, arguments can also be added to the command:

```
$ # The following 2 commands are identical
$ make test CARGO_ARGS="--all --verbose -- --nocapture"
$ cargo test --all --verbose -- --nocapture
$
$ # Run a single test
$ make test CARGO_ARGS="test_any_data"
$ cargo test test_any_data
$
$ # Run a single test with stdout printed
$ make test CARGO_ARGS="test_any_data -- --nocapture"
$ cargo test test_any_data -- --nocapture
```

### Auxiliary Tasks

The provided makefiles also include definitions for common cargo tasks, however, it is never a requirement to use the make tasks, one can simply use the corresponding cargo commands:

```
$ make check CARGO_ARGS="--all-targets"
$ cargo check --all-targets
$
$ make clippy
$ cargo clippy
$
$ make fmt CARGO_ARGS="--check"
$ cargo fmt --check
```

A designated make task has been provided to run arbitrary cargo command:

```
$ # The following 2 commands are identical
$ make cargo CARGO_CMD=tree CARGO_ARGS="--color always"
$ cargo tree --color always
```

Again, it is not required to use `make cargo` task, one is always free to simply use `cargo tree`.

At top repo level, another designated make task is provided to run a single make task on a single contract. For instance, the `stack-reorder` example has an `adjust_stack_size` task to, as the name hints, adjust the allocated stack size of the generated binary. One way to do this, is to manually chdir to `contracts/stack-reorder` folder, and run this directly:

```
$ cd contracts/stack-reorder
$ make adjust_stack_size STACK_SIZE=0x200000 TOP=../..
$ cd ../..
```

We will explain how `TOP` is used here in the next section.

Another way of doing this, is using the `run` make task at top repo level:

```
$ make run CONTRACT=stack-reorder TASK=adjust_stack_size STACK_SIZE=0x200000
```

This helps achieve the same result. But this way it is not need to change current folder, nor maintaining `TOP` per conventions discussed below.

### Workspace vs Single Crate

Current repository is organized as a Rust workspace to showcase as much detail as possible. However this is not always the case: when all one needs, is a single smart contract, there is not need to setup a full workspace to do it. As a result, the makefiles here are designed to work both in a workspace environment, and as individual crate. For example, to build a single smart contract, one can simply copy the full `contracts/minimal-log` elsewhere, then use it as a starting point to build a standalone smart contract. All the make tasks introduced above, except for `run` which is meant at workspace level, will continue to work.

However, there are still intricacies one needs to pay attention to:

* All the makefiles expect that the current top-level path is put in `TOP` variable. And this `TOP` variable, must always point to the top level of your repository. For workspace-style repositories, `TOP` must always point to the top of the workspace(even if you are directly working on a crate in the workspace), for single-crate-style repositories, `TOP` must always point to the top of the crate directory.
* Unless `TOP` is already set, all makefiles will use current running directory as the `TOP` value. This explains when we chdir to `contracts/stack-reorder` to run a make task, we need to manually set the `TOP` value: in a workspace setup, `TOP` must always point to the top of the workspace.
* All submodules, are expected to be put in `TOP`/deps. It might work if you put them in other folders, but we strongly recommend that this convention to be respected, so as to be nice to other makefiles.
* When workspace-level make process needs to call into contract-level make process for different tasks, variables such as `TOP` (and others, see the top-level makefile for details, search for `export` to locate the exact location) will be passed from the parent make process, to child make process, so as to properly initialize the contract-level make process to respect the workspace layout. This is why `make run` does not need individual setting on `TOP`, despite we want to run the make task on a specific contract.

## Blocker

This sample is pending on the following PRs to land in a proper release:

* <https://github.com/nervosnetwork/capsule/pull/137>

For now, we have to use git dependencies for certain crates.
