## eip-lookup

[![Publish EIP Lookup](https://github.com/dmanjunath/eip-lookup/actions/workflows/publish-eip-lookup-release.yml/badge.svg)](https://github.com/dmanjunath/eip-lookup/actions/workflows/publish-eip-lookup-release.yml)


Do you always forget what an EIP does? Confused with all the numbers that get thrown around? 

Here's a CLI allows you to quickly query and view EIPs in the Terminal!

### Usage

```bash
eip-lookup 20
```

### Installation
The Github Releases have the binary attached. Currently works in Ubuntu. For all other operating systems, build from source

```bash
cargo build --release
# make executable available globally
sudo mv ./target/release/eip-lookup /usr/bin/
eip-lookup 20

```