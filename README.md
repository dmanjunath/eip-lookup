## eip-lookup

[![Publish EIP Lookup](https://github.com/dmanjunath/eip-lookup/actions/workflows/publish-eip-lookup-release.yml/badge.svg)](https://github.com/dmanjunath/eip-lookup/actions/workflows/publish-eip-lookup-release.yml)


Do you always forget what an EIP does? Confused with all the numbers that get thrown around? 

Here's a CLI allows you to quickly query and interact with EIPs in the Terminal!

### Usage

```bash
eip-lookup

Usage: eip-lookup [OPTIONS] <EIP_ID>

Arguments:
  <EIP_ID>  

Options:
  -b, --browser  
  -h, --help     Print help
  -V, --version  Print version

...
$ eip-lookup 20             # print body in terminal
$ eip-lookup 1559 --browser # open EIP in browser
```

### Installation
The Github Releases have the Ubuntu binary attached. For all other operating systems, build from source using the `build-and-install.sh` file (requires rust and cargo).

```bash
cd eip-lookup/
sh build-and-install.sh
# optional shorthand command
alias eipl=eip-lookup
eipl 20

```