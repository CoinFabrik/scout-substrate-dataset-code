# Scout Substrate Dataset Code

![https://img.shields.io/badge/license-MIT-green](https://img.shields.io/badge/license-MIT-green)

This repository contains all the code for the dataset in the https://github.com/CoinFabrik/scout-substrate-dataset repository.

It contains the whole story for all the repositories involved, so you can just checkout the proper revision using its commit hash.

Each relevant revision is also tagged. Tags have the form `audited-A-B` or `remediated-A-B` where A is the `audited_project_id` and B is the index in the `findings` array. See https://github.com/CoinFabrik/scout-substrate-dataset-code/tags

## How to get the code

### Bundles

We leverage the github API to make the code referred in the dataset available as either .zip or .tar.gz files. You can fetch them by tag or commit hash. Any commit hash referred in the dataset should be in this repository.

#### By tag

Bundles can be fetched as .zip or .tar.gz files.

In order to fetch a .zip file do:
```
$ wget https://github.com/CoinFabrik/scout-substrate-dataset-code/archive/refs/tags/[tag-name].zip
```

In order to fetch a .tar.gz file do:
```
$ wget https://github.com/CoinFabrik/scout-substrate-dataset-code/archive/refs/tags/[tag-name].tar.gz
```

#### By commit hash

Bundles can also be fetched as .zip or .tar.gz files by its commit hash.

In order to fetch a .zip file do:

```
$ wget https://github.com/CoinFabrik/scout-substrate-dataset-code/archive/[commit-hash].zip
```

In order to fetch a .tar.gz file do:

```
$ wget https://github.com/CoinFabrik/scout-substrate-dataset-code/archive/[commit-hash].tar.gz
```

Both the short and long form of the commit hash can be used.

### Full history

The full history of each commit can be obtained as a git repository aswell.

#### By tag

To fetch all the history of a revision by tag do:

```
$ git clone https://github.com/CoinFabrik/scout-substrate-dataset-code [target-directory] -b [tag-name] --single-branch
```
#### By commit hash

Fetching all the history by commit hash is a little more involved, given that the git command does not support clonning an arbitrary commit hash. These are the commands required:
```
$ mkdir [target-directory]
$ cd [target-directory]
$ git init
$ git fetch https://github.com/CoinFabrik/scout-substrate-dataset-code [commit-hash]
$ git checkout [commit-hash]
```

Both the short and long form of the commit hash can be used.

## About CoinFabrik

We - [CoinFabrik](https://www.coinfabrik.com/) - are a research and development company specialized in Web3, with a strong background in cybersecurity. Founded in 2014, we have worked on over 500 blockchain-related projects, EVM-based and also for Solana, Algorand, and Polkadot. Beyond development, we offer security audits through a dedicated in-house team of senior cybersecurity professionals, currently working on code in Substrate, Solidity, Clarity, Rust, and TEAL.

Our team has an academic background in computer science and mathematics, with work experience focused on cybersecurity and software development, including academic publications, patents turned into products, and conference presentations. Furthermore, we have an ongoing collaboration on knowledge transfer and open-source projects with the University of Buenos Aires.
