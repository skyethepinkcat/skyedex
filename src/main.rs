use pokerust::{Type, FromName, Pokemon};
use structopt::StructOpt;

const NO_DAMAGE: f64 = 0.0;
const HALF_DAMAGE: f64 = 0.5;
const NORMAL_DAMAGE: f64 = 1.0;
const DOUBLE_DAMAGE: f64 = 2.0;

#[derive(Debug, StructOpt)]
#[structopt(name = "Skyedex", about = "Find info about Pokemon easily!")]
enum Opt {
    /// Lookup Information about a Pokemon!
    #[structopt(name = "pokemon")]
    Pokemon {
        /// Name of the Pokemon to lookup
        name: String,

        /// Show base stats?
        #[structopt(short, long)]
        stat: bool,

        /// Show ability?
        #[structopt(short = "b", long)]
        ability: bool,

        /// Don't want basic info?
        #[structopt(long)]
        no_basic: bool,

        /// Show all details?
        #[structopt(short, long)]
        all: bool,
    },

    /// Lookup Information about a Type!
    #[structopt(name = "type")]
    Type {
        /// Name of the Type to lookup
        name: String,

        /// Show only defensive type matchups
        #[structopt(short, long)]
        defense: bool,

        /// Show only offensive type matchups
        #[structopt(short, long)]
        offense: bool,

        /// Find Details about a type matchup
        #[structopt(short, long)]
        against: bool,

        /// Name of the opposing type
        #[structopt(required_if("against", "true"), default_value = "null")]
        primary: String,

        /// Name of the opposing secondary type
        #[structopt(required_if("against", "true"), default_value = "null")]
        secondary: String,
    },
}

fn main() -> Result<(), String> {
    match Opt::from_args() {
        // If using subcommand "pokemon," this code will run.
        Opt::Pokemon {
            name,
            mut stat,
            mut ability,
            no_basic,
            all,
        } => {
            let mon = get_pokemon(name)?;
            // If all is selected, change the other flags to true.
            if all {
                stat = true;
                ability = true;
            }

            // Skip showing name + type if the no_basic flag is true.
            if !no_basic {
                let mon_type = get_pokemon_type_names(&mon)?;
                let type_text: String;
                // If the Pokemon has two types, change the format slightly.
                if mon_type.len() == 1 {
                    type_text = format!("({})", mon_type[0])
                } else {
                    type_text = format!("({}, {})", mon_type[0], mon_type[1])
                }

                println!("{}\t{}", make_ascii_titlecase(&mon.name), type_text);
            }
            // If ability flag is true, print each ability.
            if ability {
                let mut ability_text: String = String::new();
                for i in get_ability(&mon)? {
                    ability_text += &i;
                    ability_text += ", ";
                }
                // Remove the last two characters (", ")
                ability_text.pop();
                ability_text.pop();
                println!("Abilities: {}", ability_text);
            }
            // If the stat flag is true, print all base stats.
            if stat {
                let stats = get_stats(&mon)?;
                println!(" HP: {:>3}", stats[0]);
                println!("Atk: {:>3}\tDef: {:>3}", stats[1], stats[2]);
                println!("SpA: {:>3}\tSpD: {:>3}", stats[3], stats[4]);
                println!("Spe: {:>3}", stats[5]);
            }
        }
        // If using subcommand "type," this code will run
        Opt::Type {
            name,
            against,
            primary,
            secondary,
            defense,
            offense,
        } => {
            let t = get_type(&name)?;
            // If the user is trying to test a type matchup.
            if against {
                // Get the primary defensive type, if it does not exist, send error and stop.
                let primary_against = match get_type(&primary) {
                    Ok(i) => i,
                    Err(_) => return Err("You forgot a primary type!".to_string()),
                };
                // Get the total type effectiveness.
                let primary_eff = get_effectiveness(&primary_against, &t);
                let secondary_eff: f64;
                if secondary != "null" {
                    let secondary_against = get_type(&secondary)?;
                    secondary_eff = get_effectiveness(&secondary_against, &t);
                } else {
                    secondary_eff = 1.0;
                }
                println!(
                    "A {}-type move will do {}x damage.",
                    name,
                    primary_eff * secondary_eff
                );
            } else {
                // Simply print details about the type, without any matchup comparison.
                println!("{}", make_ascii_titlecase(&t.name));

                let mut damage_types = vec![];
                let damage_strings = vec![
                    "Immune To:\n",
                    "Resistant To:\n",
                    "Weakness To:\n",
                    "Ineffective Against:\n",
                    "Not Very Effective Against:\n",
                    "Very Effective Against:\n",
                ];
                // If the user specified to show defense and not offense.
                if defense || !offense {
                    damage_types.push(&t.damage_relations.no_damage_from);
                    damage_types.push(&t.damage_relations.half_damage_from);
                    damage_types.push(&t.damage_relations.double_damage_from);
                }
                // If the user specified to show offense and not defense.
                if offense || !defense {
                    damage_types.push(&t.damage_relations.no_damage_to);
                    damage_types.push(&t.damage_relations.half_damage_to);
                    damage_types.push(&t.damage_relations.double_damage_to);
                }
                for i in 0..damage_types.len() {
                    let mut out: String;
                    out = if offense && !defense {
                        damage_strings[i + 3].to_string() // 3 needs to be added to i if only offensive is being used
                    } else {
                        damage_strings[i].to_string()
                    };
                    for x in damage_types[i] {
                        out += &make_ascii_titlecase(&x.name);
                        out += " ";
                    }
                    out.pop();
                    out += "\n";
                    println!("{}", out);
                }
            }
        }
    }
    Ok(())
}

// Obtain a Pokemon using an api request, given it's name.
fn get_pokemon(name: String) -> Result<pokerust::Pokemon, String> {
    let out = Pokemon::from_name(&name.to_lowercase());
    match out {
        Ok(p) => Ok(p),
        Err(_) => Err("Couldn't find the Pokemon!".to_string()),
    }
}

// Obtain a type using an api request, given it's name.
fn get_type(name: &String) -> Result<pokerust::Type, String> {
    let t = Type::from_name(&name.to_lowercase());
    match t {
        Ok(t) => Ok(t),
        Err(_) => Err("Couldn't find the type!".to_string()),
    }
}

// Obtain the names of a pokemon's types in a vector.
fn get_pokemon_type_names(p: &Pokemon) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = vec![];
    for i in &p.types {
        match i.type_.get() {
            Ok(t) => out.push(make_ascii_titlecase(&t.name)),
            Err(_) => return Err("Something went wrong!".to_string()),
        };
    }

    Ok(out)
}

// Get a given Pokemon's stats as a vector string.
fn get_stats(p: &Pokemon) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = vec![];
    for s in &p.stats {
        out.push(make_ascii_titlecase(&s.base_stat.to_string()));
    }
    Ok(out)
}

// Get the effectiveness of attacking a defending type with an offensive type.
fn get_effectiveness(defending: &pokerust::Type, attacking: &pokerust::Type) -> f64 {
    let damage_types = vec![
        &defending.damage_relations.no_damage_from,
        &defending.damage_relations.half_damage_from,
        &defending.damage_relations.double_damage_from,
    ];
    for i in 0..3 {
        for x in damage_types[i] {
            if x.name == attacking.name {
                match i {
                    0 => return NO_DAMAGE,
                    1 => return HALF_DAMAGE,
                    2 => return DOUBLE_DAMAGE,
                    _ => {}
                }
            }
        }
    }
    return NORMAL_DAMAGE;
}

// Get all of a given Pokemon's abilities.
fn get_ability(p: &Pokemon) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = Vec::new();
    for i in &p.abilities {
        match i.ability.get() {
            Ok(t) => out.push(make_ascii_titlecase(&t.name)),
            Err(_) => return Err("Something went wrong!".to_string()),
        };
    }
    Ok(out)
}

// Make a given string title case.
fn make_ascii_titlecase(st: &str) -> String {
    let mut s: String = st.to_string();

    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
    return s.to_string();
}
