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
