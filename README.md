# Open settings and configuration framework

This document should acts as a starting point to a global configuration framework that should remove the countless configuration file formats that exist in the modern software world.

It shall specify a common configuration language that can be used interchangeably between settings apps, configuration services and applications.

## Rationale

The goal of this document is to spark the idea of a common configuration / settings platform that can be used to configure operating systems, programs and services.

Configuring program settings is a difficult task and one that gets done differently from program to program.
This proposal exists to push the idea of a common configuration language that can be used in two directions:

- The program that gets configured

- The program that configures other programs

This eases the development of programs that need a configuration interface, as they can hook into existing configuration platforms.
The same is true for configuring programs such as settings - they can hook into this system and access lots of interfaces and configuration possibilities.

# Specification

This part of the document specifies a proposal for this configuration language and interface.

## Model

The configuration is represented in a tree form using key-value pairs.
This allows for unique identification of configuration keys to avoid duplicates.

The keys can contain any character except `/` (slash).
They are used to separate trees from each other to allow a dot-syntax-like key access.

The following primitives exist:

- Integers
- Floating point numbers
- Booleans
- Strings

And the following containers exist:

- Lists
- Key-Value Pairs

An example of this configuration tree would be the following:
```
- org (tree)
  |
  | - some-org (tree)
  |   |
  |   | - enabled (boolean)
  |   |
  |   | - home (string) 

```

The `org` and `org/some-org` keys are trees that contain nested configurations.
The `org/some-org/enabled` key is a boolean and the `org/some-org/home` key is a string.

## Schema

Each program that gets configured has a schema in which it expects its configuration data.
The schema is built using the primitives and containers to exactly describe the layout of configurations that the program expects.

**Attributes**

Each primitive and container can have some attributes attached to it.
They describe additional information about the data at hand.

Attributes can be optional or mandatory.

### Integers

Attributes:
- `size`: The size in bytes (1/2/4/8)
- `signed`: Whether the integer stores and needs the sign bit or not
- `limits` (optional): A list of tuples that limit the possibilities of the contained value

### Floating point numbers

Attributes:

- `size`: The size in bytes (2/4...)
- `limits` (optional): A list of tuples that limit the possibilities of the contained value

### Booleans

Attributes:

- `variants` (optional): A list of possible values (`true`/`false`) of the contained value

### Strings

Attributes:

- `length` (optional): The maximum length of the string (if some)
- `variants` (optional): A list of strings that are allowed for the value


### Lists

Attributes:

- `length` (optional): The maximum length of the list (if some)

### Key-Value Pairs

The backbone of the configuration tree.
It can contain sub-key-value pairs and thus implement a tree structure.
