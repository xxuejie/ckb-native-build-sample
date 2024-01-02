# ckb-native-build-sample

A collection of CKB contract samples that can and should be compiled natively using latest stable Rust and LLVM.

Notice the include contracts are for illustration only, they are NOT secure enough to guard CKB cells. Don't ever use them directly in production.

## Blocker

This sample is pending on the following PRs to land in a proper release:

* <https://github.com/nervosnetwork/capsule/pull/137>
* <https://github.com/nervosnetwork/ckb-std/pull/71>
* <https://github.com/nervosnetwork/ckb-std/pull/73>

For now, we have to use git dependencies for certain crates.
