# Introduction

To support volumetric queries for Mercator, a new domain-specific language (DSL) was created.

ANTLR was used to write and test the SDL, to check it stays simple 
to parse and and fast to execute. The actual [parser](https://epfl-dias.github.io/mercator_parser/) and interpreter is 
defined in rust, using [LALRPOP](https://docs.rs/lalrpop/0.18.1/lalrpop/).
