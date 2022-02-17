use pokerust::{Type, FromName, Pokemon, Nature};
const NO_DAMAGE: f64 = 0.0;
const HALF_DAMAGE: f64 = 0.5;
const NORMAL_DAMAGE: f64 = 1.0;
const DOUBLE_DAMAGE: f64 = 2.0;

// Obtain a Pokemon using an api request, given it's name.
pub fn get_pokemon(name: String) -> Result<pokerust::Pokemon, String> {
    let out = Pokemon::from_name(&name.to_lowercase());
    match out {
        Ok(p) => Ok(p),
        Err(_) => Err("Couldn't find the Pokemon!".to_string()),
    }
}

// Obtain a type using an api request, given it's name.
pub fn get_type(name: &String) -> Result<pokerust::Type, String> {
    let t = Type::from_name(&name.to_lowercase());
    match t {
        Ok(t) => Ok(t),
        Err(_) => Err("Couldn't find the type!".to_string()),
    }
}

// Obtain a type using an api request, given it's name.
pub fn get_nature(name: &String) -> Result<pokerust::Nature, String> {
    let t = Nature::from_name(&name.to_lowercase());
    match t {
        Ok(t) => Ok(t),
        Err(_) => Err("Couldn't find the Nature!".to_string()),
    }
}

pub fn get_nature_details(nature :&Nature) -> [String; 2] {
    let mut stat_names :[String;2] = [
        String::from(""),
        String::from(""),
    ];
    stat_names[0] = match &nature.decreased_stat {
        None =>  "None".to_string(),
        Some(n) => n.get().unwrap().name

    };

    stat_names[1] = match &nature.increased_stat {
        None =>  "None".to_string(),
        Some(n) => n.get().unwrap().name

    };
    return stat_names;
}
// Obtain the names of a pokemon's types in a vector.
pub fn get_pokemon_type_names(p: &Pokemon) -> Result<Vec<String>, String> {
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
pub fn get_stats(p: &Pokemon) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = vec![];
    for s in &p.stats {
        out.push(make_ascii_titlecase(&s.base_stat.to_string()));
    }
    Ok(out)
}

// Get the effectiveness of attacking a defending type with an offensive type.
pub fn get_effectiveness(defending: &pokerust::Type, attacking: &pokerust::Type) -> f64 {
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
pub fn get_ability(p: &Pokemon) -> Result<Vec<String>, String> {
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
pub fn make_ascii_titlecase(st: &str) -> String {
    let mut s: String = st.to_string();

    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
    return s.to_string();
}

