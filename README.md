# 🦀 rust proj init setup & ci

[![codecov](https://codecov.io/gh/ben1009/init-conf/branch/master/graph/badge.svg)](https://codecov.io/gh/ben1009/init-conf)
[![Build Status](https://github.com/ben1009/init-conf/workflows/check/badge.svg)](https://github.com/ben1009/init-conf/actions?query=workflow:check)

## provide the basic setup for rust project

- github actions
  - checks (fmt, clippy, test, coverage, typos with nightly and beta versions, mutil-os)
  - coverage (codecov)
- rustfmt config
- toolchain
- cargo makefile with dev as entrypoint
- `main.rs` is just for test all the workflows, could be removed when import to the other project
