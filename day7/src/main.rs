use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct GameData {
    cards: Vec<u32>,
    bid: u32,
    camel_rank: u32,
}

fn find_max_key<K, V>(a_hash_map: &HashMap<K, V>) -> Option<&K>
where
    V: Ord,
{
    a_hash_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
}

fn get_camel_rank(cards: &Vec<u32>) -> u32 {
    let mut card_rarity: HashMap<u32, u32> = HashMap::new();

    for &card in cards {
        *card_rarity.entry(card).or_insert(0) += 1;
    }
    if card_rarity.len() == 1 {
        return 7;
    }

    let num_of_jokers = card_rarity.remove(&1).unwrap_or(0);
    let max_value_key = find_max_key(&card_rarity).unwrap();
    let curr_value = card_rarity.get(&max_value_key).unwrap();
    card_rarity.insert(*max_value_key, *curr_value + num_of_jokers);

    if card_rarity.values().any(|&x| x == 5) {
        return 7;
    }

    if card_rarity.values().any(|&x| x == 4) {
        return 6;
    }

    if card_rarity.values().any(|&x| x == 3) {
        if card_rarity.values().any(|&x| x == 2) {
            return 5;
        }
        return 4;
    }

    match card_rarity.len() {
        3 => 3,
        4 => 2,
        5 => 1,
        _ => panic!("Should be something else"),
    }
}

impl std::str::FromStr for GameData {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.len() != 2 {
            return Err("Invalid input format");
        }

        let card_str = parts[0];
        let bid_str = parts[1];

        let cards: Vec<u32> = card_str.chars().map(|c| convert_char_to_value(c)).collect();
        let bid: u32 = bid_str.parse().map_err(|_| "Failed to parse bid")?;
        let camel_rank = get_camel_rank(&cards);

        Ok(GameData {
            cards,
            bid,
            camel_rank,
        })
    }
}

impl PartialEq for GameData {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for GameData {}

impl PartialOrd for GameData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GameData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.camel_rank != other.camel_rank {
            self.camel_rank.cmp(&other.camel_rank)
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}

fn convert_char_to_value(c: char) -> u32 {
    match c {
        '2'..='9' => c.to_digit(10).unwrap(),
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid character in card string"),
    }
}

fn star_one(input: &str) -> u32 {
    let mut cards = Vec::new();
    for line in input.lines() {
        match line.parse::<GameData>() {
            Ok(data) => cards.push(data),
            Err(err) => eprintln!("Error parsing line: {}", err),
        }
    }
    cards.sort();
    let mut i = 1;
    let mut sum = 0;
    for card in &cards {
        sum += card.bid * i;
        i += 1;
    }
    return sum;
}

fn main() {
    println!("{:?}", star_one(INPUT));
}
