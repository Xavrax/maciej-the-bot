# :warning: Work in progress! :warning:
# Maciej-The-Bot

![Rust](https://github.com/Xavrax/maciej-the-bot/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## What is it?

![Maciej - created by: twitter.com/artOphelia](maciej.png)

**Maciej-the-bot** is simple discord bot written in Rust that uses [serenity-rs](https://github.com/serenity-rs/serenity)
as a backend.

## Requirements

All what is needed to play with this repository:

- [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) - to download repository
- [curl](https://curl.se/download.html) [linux only] - for installing rust toolchain
- [rust toolchain](https://rustup.rs/) - project is written in rust lang
- [docker](https://docs.docker.com/engine/install/) [optional] - it is possible to use dockerized version of binary

## Building

Maciej can be build by typing command below:

```shell
cargo build
```

## Bot's commands

Is it possible to check all supported commands by running binary with calling `help` command
from discord chat or just take a look at [help file](messages/help.txt).

## Old bot

The old Maciej can be found at separated branch [old-maciej](https://github.com/Xavrax/maciej-the-bot/tree/old-maciej)
and won't be supported anymore.