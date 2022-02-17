use structopt::StructOpt;
mod util;
pub use util::*;

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
    /// Lookup Information about a Nature!
    #[structopt(name = "nature")]
    Nature {
        /// Name of the Nature to lookup
        name: String,
    }
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
        Opt::Nature {
            name,
        } => {
            let n = get_nature(&name)?;
            let stat_strings = get_nature_details(&n);
            println!("-{}, +{}", stat_strings[0], stat_strings[1]);
        }
    }
    Ok(())
}

