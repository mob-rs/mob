# Mob

[![CircleCI](https://circleci.com/gh/mob-rs/mob/tree/master.svg?style=svg)](https://circleci.com/gh/mob-rs/mob/tree/master)
[![crates.io](http://meritbadge.herokuapp.com/mob)](https://crates.io/crates/mob)

A CLI for mobbing from the comfort of your console.

## Installation

Homebrew:

```bash
brew tap mob-rs/formulae
brew install mob
```

If you are a Rust developer you can simply do:

```bash
cargo install mob
```

## Usage

#### Start

Simple start your mob by running:

```bash
mob start <members>
```

Where members is a comma separated list of users in the mob.

You can specify the amount of time for driving:

```bash
mob start Mike,Zack,Ed --minutes 3
```

#### Status

You can query the status of the mob by running:

```bash
mob status
#=> Current Driver: Mike
```

That command can also take an optional parameter of `--interval` which will
cause the command to poll for a status.

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/mob-rs/mob.
This project is intended to be a safe, welcoming space for collaboration, and
contributors are expected to adhere to the
[Contributor Covenant](contributor-covenant.org) code of conduct.
