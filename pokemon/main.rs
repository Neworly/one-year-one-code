use core::panic;
use std::{collections::HashMap, ops::Add};

fn ascii_to_number(chart: char) -> u8 {
    let byte = chart as u8;

    if byte < b'0' || byte > b'9' {
        panic!("{chart}' cannot be represented as a decimal.");
    }

    return (chart as u8) - b'0'
}

fn slice_to_number(slice: &str) -> usize {
    let mut n: usize = 0;
    let offset = 10;

    slice.chars().for_each(|x: char| {
        if n == 0 {
            n += ascii_to_number(x) as usize;
        } else {
            n = offset * n + (ascii_to_number(x) as usize);
        }
    });

    return n
}

fn explicit_items() -> HashMap<&'static str, &'static str> {
    return vec![
        ("Potion", "Heal: 20"),
        ("Super Potion", "Heal: 60"),
        ("Hyper Potion", "Heal: 120"),
        ("Full Recover", "Heal: 999, Status: None"),
    ].into_iter().collect()
}

struct Item<'a> {
    name: &'a str,
    ability: &'a str,
    quantity: usize
}

struct PokeStats {
    level: usize,

    attack: usize,
    defense: usize,
    
    special_attack: usize,
    special_defense: usize,
    
    speed: usize,
    evade: usize,

    health: isize,
    max_health: isize,

    status: PokemonStatus
}


#[derive(Clone)] struct Move<'a> {
    name: &'a str,
    attack_type: &'a str,
    damage: usize,

    current_use: usize,
    max_use: usize
}


fn explicit_moves<'a>() -> Vec<Move<'a>> {
    let moves = vec![
        Move { name: "Ember", attack_type: "Fire",  damage: 10, current_use: 20, max_use: 20 },
        Move { name: "Water Gun", attack_type: "Water",  damage: 30, current_use: 20, max_use: 20 },
        Move { name: "Tackle", attack_type: "Normal", damage: 20, current_use: 50, max_use: 50 },
        Move { name: "Burba Blast", attack_type: "Psyche", damage: 120, current_use: 1, max_use: 1 }
    ];

    const VALID_TYPES: [&str; 5] = ["fire", "water", "psyche", "normal", "darkness"];
    
    for attack in moves.iter() {
        let mut success = false;
        
        for poke_type in VALID_TYPES.iter() {
            if attack.attack_type.to_lowercase() == poke_type.to_lowercase() {
                success = true;
                break;
            }
        }
        
        if success == false {
            panic!("{}' unable to compile because of implicit attack-type.", attack.attack_type)
        }

    }

    return moves
}

struct PokeMoves<'a> {
    slot1: Option<Move<'a>>,
    slot2: Option<Move<'a>>,
    slot3: Option<Move<'a>>,
    slot4: Option<Move<'a>>
}

struct Pokemon<'a> {
    name: String,
    moves: PokeMoves<'a>,
    stats: PokeStats
}

struct PlayerCard<'a> {
    name: &'a str,

    badges: Vec<&'a str>,
    battles: usize,

    pokemon_encountered: usize
}

struct Backpack<'a> { 
    data: Vec<Item<'a>>
}

struct Player<'a> {
    trainer_card: PlayerCard<'a>,

    // where each item will be stored
    backpack: Backpack<'a>,

    // each pokemon encountered will be stored as a string.
    pokedex: String,

    // pokemons member
    party: Vec<Pokemon<'a>>,
}

// Create new items
impl<'a> Item<'a> {

    fn ability_from_name(name: &'a str) -> Option<&'a str> {
        return explicit_items().get(name).copied()
    }

    /// Panics if any implicit 'name' was passed
    fn check_item(name: &'a str) {
        let item = Self::ability_from_name(name);
        if item.is_none() {
            panic!("{name}' it's not a supported item");
        }

        println!("compiler: {name}' has accepted '{}' as abilities", item.unwrap());
        return ()
    }

    // Before creating the item, make sure it exists.
    fn new(name: &'a str) -> Self {
        Self::check_item(name);
        return Item { name, ability: Self::ability_from_name(&name).unwrap(), quantity: 1 }
    }
}

enum PokemonStatus {
    None, Poison, Confused
}

// Setup backpack
impl<'a> Backpack<'a> {
    fn new() -> Self {
        return Backpack { data: Vec::new() }
    }

    /// Calls a clousure on every single item, when the closure
    /// return 'true' every remaning iteration is cancelled.
    fn walk_through_items<F>(&mut self, mut call: F) where
        F: FnMut(usize, &mut Item) -> bool
    {
        for (idx, item) in self.data.iter_mut().enumerate() {
            if call(idx, item) != false {
                break;
            }
        }
    }

    /// Before counting 'item' as new, we'll gonna check If we already own one. 
    ///
    /// In case we already own one, increment quantity.
    fn acquire_item(&mut self, name: &'a str) {
        self.walk_through_items(|_, item| {
            if item.name == name {
                item.quantity += 1;
                return true;
            }
            return false
        });

        self.data.push(Item::new(name));
        println!("You've obtained: {name}!");
    }

    fn use_item(&mut self, name: &'a str, party: &mut Vec<Pokemon>) {

        fn tuple_name_value_format(base: &str) -> (String, String) {
            let mut template = ("empty".into(), "value".into());
            let mut turn = 0;

            let mut iterator = base.split(":");
            let mut target = iterator.next();

            while let Some(v) = target {
                if turn == 0 {
                    template.0 = v.into();
                    turn = 1;
                } else {
                    template.1 = v.into();
                    turn = 0;
                }
                target = iterator.next();
            }
            
            if turn == 1 {
                panic!("{}' is missing a value.", template.0);
            }

            return template

        }

        fn actions(item: &Item) -> Vec<(String, String)> {
            let mut effects = Vec::new();

            let base = Item::ability_from_name(item.name).unwrap().trim();

            let temp = String::from(base).add(",");

            let separator = temp.split(",").collect::<Vec<&str>>();
            for str in separator {
                if str.len() <= 2 { continue; }
                effects.push(tuple_name_value_format(str));
            }
                       
            return effects
        }

        fn apply_effects(item: &Item, members: &mut Vec<Pokemon>) {
            // We are gonna hard-code this now, this was not meant to be an intended project.
            //
            // intended to be specified by the user with some catchy handling.
            let pokemon_target_name = "John";
            if Player::look_for_pokemon_in_party(&members, &pokemon_target_name) == false {
                println!("{pokemon_target_name}?' is not in your party.");
                return ()
            }
        
            actions(item).into_iter().for_each(|(name, value)| {
                // it looks pretty amigue.. but fine?
                for pokemon in members.iter_mut() {
                    if pokemon.name != pokemon_target_name { continue; }

                    if name == "Heal" {
                        if pokemon.stats.health < pokemon.stats.max_health {
                            let actual = slice_to_number(&value) as isize;
                            let predict = pokemon.stats.health + actual;
                            if predict > pokemon.stats.max_health {
                                pokemon.stats.health = pokemon.stats.max_health
                            } else {
                                pokemon.stats.health += actual;
                            }
                        } else {
                            println!("{pokemon_target_name}' the programmer forget to implemenet a drawback, anyways you've lost a {name}");
                            continue;
                        }
                        println!("{pokemon_target_name}' healed by {value} thanks by {name}")
                    } else if name == "Status" {
                        pokemon.stats.status = match pokemon.stats.status {
                            PokemonStatus::None => {
                                println!("yeah, {pokemon_target_name} wasted your potion");
                                PokemonStatus::None
                            }, _ => {
                                println!("{pokemon_target_name} has been recovered by {name}");
                                PokemonStatus::None
                            },
                        }
                    }

                }

            }) 

        }

        let mut used_item = false;
        let mut last_item = false;
        let mut in_index: usize = 0;

        self.walk_through_items(|idx, item| {
            if item.name == name {
                used_item = true;
                in_index = idx;

                if item.quantity > 1 {
                    item.quantity -= 1;
                } else {
                    last_item = true;
                }

                apply_effects(item, party);
                return true
            }

            return false
        });

        if used_item == true {
            println!("You've used {name}!");
        } else {
            println!("{name}' is not in your inventory.");
        }

        if last_item == true {
            self.data.remove(in_index);
        }
    }
    
    fn look_up(&mut self, name: &'a str) -> bool {
        let mut found = false;
        self.walk_through_items(|_, item| {
            if item.name == name {
                found = true;
                return true
            }
            return false
        });

        return found
    }

    fn show_items(&mut self) {
        println!("<=Backpack=>");
        self.walk_through_items(|_, item| {
            println!("{}: {}", item.name, Item::ability_from_name(&item.name).unwrap());
            return false
        } );
        println!("<=End=>");
    }
}

// Setup pokemons
impl PokeStats {
    /// temporary method?
    fn default() -> Self {
        return PokeStats { 
            level: 1, 
            attack: 10, 
            defense: 20, 
            special_attack: 12,
            special_defense: 20, 
            speed: 4, 
            evade: 5, 
            health: 100, 
            max_health: 100,
            status: PokemonStatus::None
        } 
    }
}

impl<'a> Pokemon<'a> {
    fn new(name: &'a str) -> Self {
        // every pokemon will have the same movset and stats for now .
        let sets = explicit_moves();
        return Pokemon { name: name.to_string(), moves: PokeMoves {
            slot1: Some(sets[0].clone()),
            slot2: None,
            slot3: Some(sets[3].clone()),
            slot4: None
        }, stats: PokeStats::default() }
    }
}

// Setup player
impl<'a> Player<'a> {
    const MAX_POKEMONS_IN_PARTY: usize = 6;
    fn new(name: &'a str) -> Self {
        return Player {
            trainer_card: PlayerCard { name, badges: Vec::new(), battles: 0, pokemon_encountered: 0 },

            backpack: Backpack::new(),

            // testing: pokedex should be always aware about which pokemons are members of your party.
            pokedex: String::new(),

            party: Vec::with_capacity(Self::MAX_POKEMONS_IN_PARTY)

        }
    }

    fn add_party_member(&mut self, member: Pokemon<'a>) {
        let len = self.party.len();

        if len == Self::MAX_POKEMONS_IN_PARTY {
            println!("Your party is full '{}/{}' cannot add one more member: {}' got rejected", len, Self::MAX_POKEMONS_IN_PARTY, member.name);
            return ()
        }

        println!("member number {}: {} was added into our party", len + 1, member.name);

        self.party.push(member);
    }

    fn look_for_pokemon_in_party(party: &Vec<Pokemon>, name: &str) -> bool {
        for pokemon in party.iter() {
            if pokemon.name == name {
                return true
            }
        }
        return false
    }

}


fn main() {

    fn default_player<'a>() -> Player<'a> {
        let mut trainer = Player::new("John");
        
        trainer.backpack.acquire_item("Potion");

        trainer.add_party_member(Pokemon::new("John"));
        trainer.add_party_member(Pokemon::new("John2"));
        trainer.add_party_member(Pokemon::new("John3"));
        trainer.add_party_member(Pokemon::new("John4"));
        trainer.add_party_member(Pokemon::new("John5"));
        trainer.add_party_member(Pokemon::new("John6"));
 
        return trainer
    }

    fn rival_npc<'a>() -> Player<'a> {
        let mut humanoid = Player::new("Mew");

        // Backpack items
        for _ in 0 .. 10 {
            humanoid.backpack.acquire_item("Full Recover");
        }

        // Party members
        humanoid.add_party_member(Pokemon::new("Smith"));
        humanoid.add_party_member(Pokemon::new("Smith2"));
        humanoid.add_party_member(Pokemon::new("Smith3"));
        humanoid.add_party_member(Pokemon::new("Smith4"));
        humanoid.add_party_member(Pokemon::new("Smith5"));
        humanoid.add_party_member(Pokemon::new("Smith6"));

        return humanoid
    }

    fn battle_match(p1: &mut Player, p2: &mut Player) {
        println!("= = = =");
        fn calculate_damage(base_damage: usize, stats: &PokeStats, attack_type: &str) -> usize {

            // doesn't really matter...
            return base_damage;
        }

        fn alive_pokemons(p: &mut Vec<Pokemon>) -> usize {
            let mut alive = 0;
            
            for pokemon in p.iter() {
                
                if pokemon.stats.health > 0 {
                    alive += 1;
                
                }
            }
    
            return alive;
        }
        
        let mut user = (alive_pokemons(&mut p1.party), p1);
        let mut enemy = (alive_pokemons(&mut p2.party), p2);


        // since we are hardcoding this, should we make it grow linearly?
        let mut user_party_member_idx = 3;
        let mut enemy_party_member_idx = 0;
    
        while user.0 > 0 && enemy.0 > 0 {

            let mut user_attacked = false;
            let mut enemy_attacked = false;

            let user_pokemon = &mut user.1.party[user_party_member_idx];
            let enemy_pokemon = &mut enemy.1.party[enemy_party_member_idx];

            // hard coding player skills, attack uses will not be handled...
            let user_skill = user_pokemon.moves.slot1.clone().unwrap();
            let enemy_skill = user_pokemon.moves.slot1.clone().unwrap();

            println!("{} uses {} agaisnt {}", user_pokemon.name, user_skill.name, enemy_pokemon.name); 

            while enemy_attacked == false || user_attacked == false {
                if enemy_attacked == true || user_pokemon.stats.speed > enemy_pokemon.stats.speed {
                    enemy_pokemon.stats.health -= calculate_damage(user_skill.damage, &user_pokemon.stats, user_skill.attack_type) as isize;
                    if enemy_pokemon.stats.health <= 0 {
                        println!("{} was defeated by {}", enemy_pokemon.name, user_pokemon.name);
                        enemy_party_member_idx += 1;
                        break;
                    }
                    user_attacked = true;
                } else if enemy_attacked == false {
                    user_pokemon.stats.health -= calculate_damage(enemy_skill.damage, &enemy_pokemon.stats, enemy_skill.attack_type) as isize;
                    if user_pokemon.stats.health <= 0 {
                        println!("{} was defeated by {}", user_pokemon.name, enemy_pokemon.name);
                        user_party_member_idx = (user_party_member_idx + 1) % 6;
                        break;
                    }
                    enemy_attacked = true
                }
            }

            user.0 = alive_pokemons(&mut user.1.party);
            enemy.0 = alive_pokemons(&mut enemy.1.party);

        }
    }

    fn tutorial(p1: &mut Player, p2: &mut Player) {
        // we are gonna hard code this, input will not be handled
        println!("\nHey, there welcome into PBCL. Here we'll introduce you the gameplay.\nPlease choose an item within your backpack");
        p1.backpack.show_items();

        let mut choosed_item = "Potion";
        while p1.backpack.look_up(choosed_item) != true {
            println!("{choosed_item}' is not into your backpack, please choose an item in within!");
            choosed_item = "Super Potion"
        }
    
        p1.backpack.use_item(choosed_item, &mut p1.party);
        println!("Good joob, you've used a potion.. technically?");
        println!("Now, let's move on to battle!");
        battle_match(p1, p2);
        println!("nothing more.. this is really it..\nI guess this counts as 2023 end-year 'project' well.. not fullfithed though..");
    }

    let mut trainer = default_player();
    let mut rival = rival_npc();
    
    tutorial(&mut trainer, &mut rival);
}

