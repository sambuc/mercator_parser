# Mercator Parser

Query parser for Mercator.

## Mercator: Spatial Index

**Mercator** is a spatial *volumetric* index for the [Human Brain Project](http://www.humanbrainproject.eu). It is a component of the [Knowledge Graph](http://www.humanbrainproject.eu/en/explore-the-brain/search/) service, which  provides the spatial anchoring for the metadata registered as well as processes the volumetric queries.

It is build on top of the Iron Sea database toolkit.

## Iron Sea: Database Toolkit

**Iron Sea** provides a set of database engine bricks, which can be combined and applied on arbitrary data structures.

Unlike a traditional database, it does not assume a specific physical structure for the tables nor the records, but relies on the developper to provide a set of extractor functions which are used by the specific indices provided.

This enables the index implementations to be agnostic from the underlying data structure, and re-used.

## Requirements

### Software

 * Rust: https://www.rust-lang.org

## Quick start

## Building from sources

To build this project, you will need to run the following:

```sh
cargo build --release
```

### Installation

To install the software on the system you can use:

```sh
cargo install --release
```

### Usage

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin vehicula pretium
quam sit amet facilisis. Class aptent taciti sociosqu ad litora torquent per
conubia nostra, per inceptos himenaeos. Curabitur metus sapien, rhoncus vitae
eleifend nec, convallis vel nunc. Nulla metus mauris, porta eu porta eu,
vulputate et est. Suspendisse lacinia leo vel auctor aliquet. Maecenas non arcu
libero. Nulla ut eleifend dui. Cras bibendum pharetra facilisis. Proin mattis
libero non pharetra tristique. Nam massa nulla, ultrices pharetra quam a,
fermentum placerat dolor. Nullam mollis libero et neque lobortis, id dignissim
lectus dignissim. Maecenas ligula enim, congue in ornare vel, volutpat ut ante.

```sh
cargo run --release
```

## Documentation

For more information, please refer to the [documentation](https://epfl-dias.github.io/mercator_parser/).

If you want to build the documentation and access it locally, you can use:

```sh
cargo doc --open
```

## Acknowledgements

This open source software code was developed in part or in whole in the
Human Brain Project, funded from the European Union’s Horizon 2020
Framework Programme for Research and Innovation under the Specific Grant
Agreement No. 785907 (Human Brain Project SGA2).
