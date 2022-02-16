# Skyedex

This is a rust application that uses the [pokeapi](https://pokeapi.co/) to
quickly access basic information about Pokemon.

## Compilation and Installation

To compile and install, simply run:

```bash
cargo install --path . 
```
This should install all necessary requirements!

## Usage
Currently, there are only two major functions of Skyedex:

### pokemon
You can use the "pokemon" subcommand to look up certain information about pokemon.

```
USAGE:
    skyedex pokemon [FLAGS] <name>

FLAGS:
    -b, --ability     Show ability?
    -a, --all         Show all details?
    -h, --help        Prints help information
        --no-basic    Don't want basic info?
    -s, --stat        Show base stats?
    -V, --version     Prints version information
```

```
<~>-> skyedex pokemon eevee -a
Eevee   (Normal)
Abilities: Run-away, Adaptability, Anticipation
 HP:  55
Atk:  55        Def:  50
SpA:  45        SpD:  65
Spe:  55
```


### type
You can use the "type" command to lookup information on type matchup, including
how effective a type is against certain type combination.

```
USAGE:
    skyedex type [FLAGS] <name> [ARGS]

FLAGS:
    -a, --against    Find Details about a type matchup
    -d, --defense    Show only defensive type matchups
    -h, --help       Prints help information
    -o, --offense    Show only offensive type matchups
    -V, --version    Prints version information

ARGS:
    <name>         Name of the Type to lookup
    <primary>      Name of the opposing type [default: null]
    <secondary>    Name of the opposing secondary type [default: null]
```

```
<~>-> skyedex type poison
Poison
Immune To:

Resistant To:
Fighting Poison Bug Grass Fairy

Weakness To:
Ground Psychic

Ineffective Against:
Steel

Not Very Effective Against:
Poison Ground Rock Ghost

Very Effective Against:
Grass Fairy
```

```
<~>-> skyedex type poison -a ground grass
A poison-type move will do 1x damage.
```
