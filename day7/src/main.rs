const SAMPLE: &str = r#"
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483
"#;
fn main() {
    println!("part 1:");
    part1::run(SAMPLE);
    part1::run(include_str!("input.txt"));

    println!("part2:");
    part2::run(SAMPLE);
    part2::run(include_str!("input.txt"));
}

mod part1 {
    use crate::shared;

    pub fn run(cards: &str) {
        let f = |cards: &str| {
            let mut map = vec![];
            for c in cards.chars() {
                if let Some((_, entry)) = map.iter_mut().find(|(a, _)| a == &c) {
                    *entry += 1;
                } else {
                    map.push((c, 1));
                }
            }
            map
        };
        shared::run(
            cards,
            &[
                'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
            ],
            f,
        );
    }
}

mod part2 {
    use crate::shared;

    pub fn run(cards: &str) {
        let f = |cards: &str| {
            let mut map = vec![];
            let mut count_j = 0;
            for c in cards.chars() {
                if c == 'J' {
                    count_j += 1;
                } else if let Some((_, entry)) = map.iter_mut().find(|(a, _)| a == &c) {
                    *entry += 1;
                } else {
                    map.push((c, 1));
                }
            }

            map.sort_by(|a, b| a.1.cmp(&b.1));
            if let Some(v) = map.last_mut() {
                v.1 += count_j;
            }
            map
        };

        shared::run(
            cards,
            &[
                'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
            ],
            f,
        );
    }
}

mod shared {

    use std::cmp::Ordering;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Hand<'a> {
        pub kind: HandKind,
        pub cards: &'a str,
        pub bid: u128,
    }
    #[derive(Debug, PartialEq, Eq)]
    pub enum HandKind {
        FiveOfAKind,
        FourOfAKind,
        FullHouse,
        ThreeOfAKind,
        TwoPair,
        OnePair,
        HighCard,
    }

    pub fn cmp_hands(lhs: &Hand, rhs: &Hand, cards_order: &[char]) -> Option<std::cmp::Ordering> {
        let kmp = |a: &str, b: &str| {
            a.chars()
                .zip(b.chars())
                .map(|(p0, p1)| cmp(p1, p0, cards_order))
                .find(|ord| !matches!(ord, Ordering::Equal))
                .or(Some(Ordering::Equal))
        };

        match (&lhs.kind, &rhs.kind) {
            (HandKind::FiveOfAKind, HandKind::FiveOfAKind) => kmp(lhs.cards, rhs.cards),
            (HandKind::FiveOfAKind, _) => Some(Ordering::Greater),
            (HandKind::FourOfAKind, HandKind::FiveOfAKind) => Some(std::cmp::Ordering::Less),
            (HandKind::FourOfAKind, HandKind::FourOfAKind) => kmp(lhs.cards, rhs.cards),
            (HandKind::FourOfAKind, _) => Some(Ordering::Greater),
            (HandKind::FullHouse, HandKind::FiveOfAKind | HandKind::FourOfAKind) => {
                Some(std::cmp::Ordering::Less)
            }
            (HandKind::FullHouse, HandKind::FullHouse) => kmp(lhs.cards, rhs.cards),
            (HandKind::FullHouse, _) => Some(Ordering::Greater),
            (
                HandKind::ThreeOfAKind,
                HandKind::FiveOfAKind | HandKind::FourOfAKind | HandKind::FullHouse,
            ) => Some(std::cmp::Ordering::Less),
            (HandKind::ThreeOfAKind, HandKind::ThreeOfAKind) => kmp(lhs.cards, rhs.cards),
            (HandKind::ThreeOfAKind, _) => Some(Ordering::Greater),
            (
                HandKind::TwoPair,
                HandKind::FiveOfAKind
                | HandKind::FourOfAKind
                | HandKind::FullHouse
                | HandKind::ThreeOfAKind,
            ) => Some(std::cmp::Ordering::Less),
            (HandKind::TwoPair, HandKind::TwoPair) => kmp(lhs.cards, rhs.cards),

            (HandKind::TwoPair, _) => Some(Ordering::Greater),
            (HandKind::OnePair, HandKind::OnePair) => kmp(lhs.cards, rhs.cards),
            (HandKind::OnePair, HandKind::HighCard) => Some(Ordering::Greater),
            (HandKind::OnePair, _) => Some(Ordering::Less),
            (HandKind::HighCard, HandKind::HighCard) => kmp(lhs.cards, rhs.cards),

            (HandKind::HighCard, _) => Some(Ordering::Less),
        }
    }

    pub fn cmp(a: char, b: char, cards_order: &[char]) -> Ordering {
        let pos_b = cards_order.iter().position(|c| c == &b).unwrap();
        let pos_a = cards_order.iter().position(|c| c == &a).unwrap();
        pos_a.cmp(&pos_b)
    }

    pub fn run<F>(input: &str, cards_order: &[char], part: F)
    where
        F: Fn(&str) -> Vec<(char, i32)>,
    {
        let mut hands = input
            .trim()
            .lines()
            .filter_map(|line| line.trim().split_once(' '))
            .filter_map(|(cards, bid)| {
                bid.parse::<u128>().ok().map(|bid| Hand {
                    cards,
                    kind: get_kind(cards, &part),
                    bid,
                })
            })
            .collect::<Vec<_>>();
        hands.sort_by(|a, b| cmp_hands(a, b, cards_order).unwrap());

        let mut tot = 0;
        for (i, hand) in hands.iter().enumerate() {
            tot += hand.bid * (i + 1) as u128;
        }
        println!("{tot}");
    }

    fn get_kind<F>(cards: &str, part: &F) -> HandKind
    where
        F: Fn(&str) -> Vec<(char, i32)>,
    {
        let map = part(cards);

        let max = map.iter().map(|(_, c)| c).max();

        match map.len() {
            0 | 1 => HandKind::FiveOfAKind,
            2 if max == Some(&4) => HandKind::FourOfAKind,
            2 if max == Some(&3) => HandKind::FullHouse,
            3 if max == Some(&3) => HandKind::ThreeOfAKind,
            3 if max == Some(&2) => HandKind::TwoPair,
            4 => HandKind::OnePair,
            5 => HandKind::HighCard,
            _ => panic!("nope"),
        }
    }
}
