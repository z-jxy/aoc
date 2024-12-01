#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
enum HandStrength {
    FiveOfAKind, // AAAAA
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandStrength {
    fn from_cards(cards: &[Card]) -> HandStrength {
        let mut counts = [0; 15];

        cards.iter().for_each(|card| {
            counts[*card as usize] += 1;
        });

        let mut counts = counts.iter().enumerate().collect::<Vec<_>>();
        println!("{:?}", counts);
        counts.sort_by(|a, b| b.1.cmp(a.1));

        let counts = counts.iter().map(|(_, v)| **v).collect::<Vec<_>>();

        match counts[0] {
            5 => HandStrength::FiveOfAKind,
            4 => HandStrength::FourOfAKind,
            3 => {
                if counts[1] == 2 {
                    HandStrength::FullHouse
                } else {
                    HandStrength::ThreeOfAKind
                }
            }
            2 => {
                if counts[1] == 2 {
                    HandStrength::TwoPair
                } else {
                    HandStrength::OnePair
                }
            }
            _ => HandStrength::HighCard,
        }
    }

    fn from_cards2(cards: &[Card]) -> HandStrength {
        let mut counts = [0; 15];
        let jokers = cards.iter().filter(|c| **c == Card::Joker).count();
        println!("hand: {:?}\njokers: {}\n----", cards, jokers);

        for card in cards {
            if *card != Card::Joker {
                counts[*card as usize] += 1;
            }
        }

        let mut sorted_counts = counts
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != Card::Joker as usize)
            .collect::<Vec<_>>();

        sorted_counts.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(&b.0)));

        match sorted_counts[0].1 + jokers {
            5 => HandStrength::FiveOfAKind,
            4 => HandStrength::FourOfAKind,
            3 => {
                if *sorted_counts[1].1 >= 2 || jokers >= 2 {
                    HandStrength::FullHouse
                } else {
                    HandStrength::ThreeOfAKind
                }
            }
            2 => {
                if *sorted_counts[1].1 == 2 || (*sorted_counts[1].1 == 1 && jokers >= 1) {
                    HandStrength::TwoPair
                } else {
                    HandStrength::OnePair
                }
            }
            _ => HandStrength::HighCard,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
enum Card {
    Ace,   // 14
    King,  // 13
    Queen, // 12
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker, // wildcard
}

impl Card {
    fn from_char(c: char) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Joker,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
struct Hand {
    strength: HandStrength,
    cards: [Card; 5],
    bid: usize,
}

pub fn hand_type(cards: &[Card]) -> HandStrength {
    let mut card_counts = [0; 14];
    let mut max_count = 0;
    let mut second_max_count = 0;
    for &card in cards {
        let count = &mut card_counts[card as usize];
        *count += 1;
        if card == Card::Joker {
            continue;
        }
        if *count > max_count {
            max_count = *count;
        } else if *count > second_max_count {
            second_max_count = *count;
        }
    }
    max_count += card_counts[Card::Joker as usize];
    match (max_count, second_max_count) {
        (5, _) => HandStrength::FiveOfAKind,
        (4, _) => HandStrength::FourOfAKind,
        (3, 2) | (3, 1) if card_counts[Card::Joker as usize] > 0 => HandStrength::FullHouse,
        (3, _) => HandStrength::ThreeOfAKind,
        (2, 2) => HandStrength::TwoPair,
        (2, 1) if card_counts[Card::Joker as usize] > 0 => HandStrength::ThreeOfAKind,
        (2, _) => HandStrength::OnePair,
        _ => HandStrength::HighCard,
    }
}

impl Hand {}

fn rank_hands(hands: &[Hand]) -> Vec<Hand> {
    let mut sorted = hands.iter().copied().collect::<Vec<_>>();
    // if the strengths are equal, check each card in the hand until the first non-equal card is found
    // the hand the contains the higher card wins
    sorted.sort_by(|a, b| {
        if a.strength == b.strength {
            for i in 0..5 {
                if a.cards[i] != b.cards[i] {
                    return b.cards[i].cmp(&a.cards[i]);
                }
            }
        }
        b.strength.cmp(&a.strength)
    });

    sorted
}

pub fn part_two(hands: &mut [Hand]) -> usize {
    // Modify the part_two function to accommodate Joker's wildcard functionality
    hands.sort_unstable();
    hands
        .iter()
        .zip(1..=hands.len() as u32)
        .map(|(hand, rank)| hand.bid * rank as usize)
        .sum()
}

#[aoc::main(07)]
fn main(input: &str) -> (usize, usize) {
    //let input = std::fs::read_to_string("../inputs/07.test").unwrap();

    let hands = input
        .lines()
        .map(|line| {
            let mut line = line.split_whitespace();

            let mut cards = [Card::Two; 5];

            line.next()
                .unwrap()
                .chars()
                .enumerate()
                .for_each(|(i, c)| cards[i] = Card::from_char(c));

            let bid = line.next().unwrap().parse().unwrap();
            Hand {
                cards,
                strength: hand_type(&cards),
                bid,
            }
        })
        .collect::<Vec<_>>();

    let ranked = rank_hands(&hands);

    // let total_winnings = ranked
    //     .iter()
    //     .enumerate()
    //     .map(|(rank, hand)| (rank + 1) * hand.bid)
    //     .sum::<usize>();

    let total_winnings = part_two(&mut hands.to_vec());

    println!("{:?}", ranked);
    println!("winnings: {}", total_winnings);

    assert!(
        total_winnings != 251617933 || total_winnings != 251068434 || total_winnings > 251617933
    );
    (0, 0)
}
