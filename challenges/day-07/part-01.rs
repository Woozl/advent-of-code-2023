use std::{collections::HashMap, fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-07/input.txt"))?;

    let cards = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    let value_map: HashMap<char, usize> = cards.iter().enumerate().map(|(i, c)| (*c, i + 2)).collect();

    let hands = input.lines().map(| l | {
        let mut line_iter = l.split_ascii_whitespace();
        let hand: &str = line_iter.next().unwrap();
        let bid: usize = line_iter.next().unwrap().parse().unwrap();
        (hand, bid)
    });

    let mut scored_hands = hands.map(| (hand, bid) | {
        let mut freq_map: HashMap<char, usize> = HashMap::new();
        let mut hand_score: usize = 0;

        for (i, c) in hand.chars().enumerate() {
            let card_value = value_map.get(&c).unwrap();

            // im not sure if this works on every possible input, the idea is to multiply
            // the card value by a weight determined by its position in the hand (further left
            // is stronger).
            hand_score += card_value * usize::pow(100, (4 - i) as u32);
            
            freq_map.entry(c).and_modify(| freq | *freq += 1).or_insert(1);
        }

        let mut freq_distrib: Vec<&usize> = freq_map.values().collect();
        freq_distrib.sort();

        let hand_classification: usize = match freq_distrib.as_slice() {
            [5] => 7, // Five of a kind
            [1, 4] => 6, // Four of a kind
            [2, 3] => 5, // Full house
            [1, 1, 3] => 4, // Three of a kind
            [1, 2, 2] => 3, // Two pair
            [1, 1, 1, 2] => 2, // One pair
            [1, 1, 1, 1, 1] => 1, // High card
            d => panic!("Found a card distribution that doesn't fix a classification {d:?}")
        };

        (hand, bid, hand_classification, hand_score)
    }).collect::<Vec<(&str, usize, usize, usize)>>();

    // if the hand classifications are the same, sort by the score
    scored_hands.sort_by(| a, b | {
        if a.2 == b.2 { 
            a.3.cmp(&b.3)
        }
        else {
            a.2.cmp(&b.2)
        }
    });

    let mut sum: usize = 0;
    for (i, (hand, bid, class, score)) in scored_hands.iter().enumerate() {
        println!("{}\t-\t{}\t-\t{}\t-\t{}", hand, bid, class, score);
        sum += (i + 1) * bid;
    }
    
    println!("Sum: {}", sum);

    Ok(())
}
