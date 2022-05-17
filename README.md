# gk-yaml 
[![license](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/r1cm3d/gk-yaml/blob/master/LICENSE)

**TL;DR:**

![](assets/demo.gif)

## Prerequisites
[![rust](https://img.shields.io/badge/rust-2021-orange?style=flat-square)](https://github.com/rust-lang/rust)
``` console
brew install rustup && rustup-init
```

## Table of Contents
* [TL;DR](#ppsch)
* [Prerequisites](#prerequisites)
* [About](#about-the-project)
* [Building](#building)
* [Installing](#installing)
* [Tips](#tips)
* [Testing](#testing)
* [Getting Help](#getting-help)

## About
A CLI application that helps to work with goalkeeper generating yaml files automatically.

## Building 
```
make build
```
It will call `cargo build` in order to download and build all dependencies.

## Installing
```
make install
```
It requires `GK_BIN` environment variable that must be on `PATH` environment variable. The user must have permissions to
write in this directory.

## Tips 
It is worth to keep most used commands in aliases as follows:
``` console
# Reprocess Clearing
alias repcleit='gk-yaml --envs itau --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations" sqs reprocess  --channel C01QQ11RECT --src-queue "NetworkAuthorizationsClearing-dead-letter" --dst-queue NetworkAuthorizationsClearing'
alias repclemt='gk-yaml --envs multitenant --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations" sqs reprocess  --channel C01QQ11RECT --src-queue "NetworkAuthorizationsClearing-dead-letter" --dst-queue NetworkAuthorizationsClearing'
alias repcleboth='gk-yaml --envs multitenant itau --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations" sqs reprocess  --channel C01QQ11RECT --src-queue "NetworkAuthorizationsClearing-dead-letter" --dst-queue NetworkAuthorizationsClearing'

# Download clearing
alias dowcleit='gk-yaml --envs itau --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations" sqs download --channel C01QQ11RECT --queue "NetworkAuthorizationsClearing-dead-letter"'
alias dowclemt='gk-yaml --envs multitenant --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations" sqs download --channel C01QQ11RECT --queue "NetworkAuthorizationsClearing-dead-letter"'
alias dowcleboth='gk-yaml --envs multitenant itau --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations" sqs download --channel C01QQ11RECT --queue "NetworkAuthorizationsClearing-dead-letter"'

# Reprocess Ledger
alias repledt='gk-yaml --envs itau --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations" sqs reprocess  --channel C01QQ11RECT --src-queue "NetworkAuthorizationsLedgerIntegrity-dead-letter" --dst-queue NetworkAuthorizationsLedgerIntegrity'
alias repledmt='gk-yaml --envs multitenant --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations" sqs reprocess  --channel C01QQ11RECT --src-queue "NetworkAuthorizationsLedgerIntegrity-dead-letter" --dst-queue NetworkAuthorizationsLedgerIntegrity'
alias repledboth='gk-yaml --envs multitenant itau --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations" sqs reprocess  --channel C01QQ11RECT --src-queue "NetworkAuthorizationsLedgerIntegrity-dead-letter" --dst-queue NetworkAuthorizationsLedgerIntegrity'

# Download Ledger
alias dowledit='gk-yaml --envs itau --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations" sqs download --channel C01QQ11RECT --queue "NetworkAuthorizationsLedgerIntegrity-dead-letter"'
alias dowledmt='gk-yaml --envs multitenant --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations" sqs download --channel C01QQ11RECT --queue "NetworkAuthorizationsLedgerIntegrity-dead-letter"'
alias dowledboth='gk-yaml --envs multitenant itau --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations" sqs download --channel C01QQ11RECT --queue "NetworkAuthorizationsLedgerIntegrity-dead-letter"'
 
# Download Settlement Files 
alias dowsetit='gk-yaml --envs itau --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations" sqs download --channel C01QQ11RECT --queue "ClearingBatchSettlementFiles-dead-letter"'
alias dowsetmt='gk-yaml --envs multitenant --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations" sqs download --channel C01QQ11RECT --queue "ClearingBatchSettlementFiles-dead-letter"'
alias dowsetboth='gk-yaml --envs multitenant itau --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations" sqs download --channel C01QQ11RECT --queue "ClearingBatchSettlementFiles-dead-letter"'

# Purge Settlement Files 
alias pursetit='gk-yaml --envs itau --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations"  sqs purge --queue "ClearingBatchSettlementFiles-dead-letter"'
alias pursetmt='gk-yaml --envs multitenant --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations"  sqs purge --queue "ClearingBatchSettlementFiles-dead-letter"'
alias pursetboth='gk-yaml --envs multitenant itau --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations"  sqs purge --queue "ClearingBatchSettlementFiles-dead-letter"'

# Purge Ledger 
alias purledit='gk-yaml --envs itau --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations"  sqs purge --queue "NetworkAuthorizationsLedgerIntegrity-dead-letter"'
alias purledmt='gk-yaml --envs multitenant --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations"  sqs purge --queue "NetworkAuthorizationsLedgerIntegrity-dead-letter"'
alias purledboth='gk-yaml --envs multitenant itau --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations"  sqs purge --queue "NetworkAuthorizationsLedgerIntegrity-dead-letter"'

# Purge Clearing 
alias purcleit='gk-yaml --envs itau --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations"  sqs purge --queue "NetworkAuthorizationsClearing-dead-letter"'
alias purclemt='gk-yaml --envs multitenant --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations"  sqs purge --queue "NetworkAuthorizationsClearing-dead-letter"'
alias purcleboth='gk-yaml --envs multitenant itau --root-directory "$PROJECTS/goalkeeper" --squad "pci-authorizations"  sqs purge --queue "NetworkAuthorizationsClearing-dead-letter"'
```

If one want to reprocess Clearing DLQ for both environments I only have to type:
```
repcleboth
```
and open the PR with [gh cli](https://github.com/cli/cli).

## Testing 
```
make test 
```
It will call `cargo test` aiming to run the basic unit tests.

## Getting Help

```console
./gk-yaml --help
```

Help information will be displayed:

```console
gk-yaml - goalkeeper yaml generator 0.1.0
A CLI application that helps to work with goalkeeper generating yaml files automatically.

USAGE:
    gk-yaml --channel <CHANNEL> --squad <SQUAD> --root-directory <ROOT_DIRECTORY> --envs <ENVS>... <SUBCOMMAND>

OPTIONS:
    -c, --channel <CHANNEL>
            The slack channel to send tasks notifications

    -e, --envs <ENVS>...
            Environment related to the task. It accepts a list: ext,itau,multitenant [possible
            values: itau, multitenant, india, ext]

    -h, --help
            Print help information

    -r, --root-directory <ROOT_DIRECTORY>
            Directory that [goalkeeper](https://github.com/pismo/goalkeeper) is located

    -s, --squad <SQUAD>
            Squad related to the task. It is used to create file path

    -V, --version
            Print version information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    sqs   
```
