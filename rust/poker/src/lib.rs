use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl From<&str> for Suit {
    fn from(s: &str) -> Self {
        match s {
            "H" => Suit::Hearts,
            "C" => Suit::Clubs,
            "S" => Suit::Spades,
            "D" => Suit::Diamonds,
            _ => panic!("Invalid Suit: {}", s),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Ord, PartialOrd, PartialEq, Eq)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<&str> for Rank {
    fn from(s: &str) -> Self {
        match s {
            "2" => Rank::Two,
            "3" => Rank::Three,
            "4" => Rank::Four,
            "5" => Rank::Five,
            "6" => Rank::Six,
            "7" => Rank::Seven,
            "8" => Rank::Eight,
            "9" => Rank::Nine,
            "10" => Rank::Ten,
            "J" => Rank::Jack,
            "Q" => Rank::Queen,
            "K" => Rank::King,
            "A" => Rank::Ace,
            _ => panic!("Invalid Rank: {}", s),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(card: &str) -> Option<Self> {
        match (card.get(..1), card.get(1..)) {
            (None, _) | (_, None) => None,
            (Some(rank), Some(suit)) => {
                    Some(Self {
                        rank: Rank::from(rank),
                        suit: Suit::from(suit),
                    })
            }
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Hand {
    HighCard(Card),
    Pair(Card),
    TwoPairs(Card, Card),
    ThreeOfAKind(Card),
    Straight(Card),
    Flush(Card),
    FullHouse(Card, Card),
    FourOfAKind(Card),
    StraightFlush(Card),
}

fn get_hands_from_cards(hand: &mut [Card]) -> Vec<Hand> {
    let mut v: Vec<Hand> = Vec::new();
    hand.sort_unstable();
    hand.reverse();
    let mut map_rank_to_freq: HashMap<Rank, u32> = HashMap::new();
    let mut map_freq_to_rank: HashMap<u32, Rank> = HashMap::new();
    let mut is_flush = false;
    if hand.iter().all(|c| c.suit == hand[0].suit) {
        is_flush = true;
    }
    for h in hand {
        *map_rank_to_freq.entry(h.rank).or_insert(0) += 1;
    }
    v
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Hand::HighCard(card), Hand::HighCard(other_card)) => card.cmp(other_card),
            (Hand::HighCard(_), _) => Ordering::Less,

            (Hand::Pair(card), Hand::Pair(other_card)) => card.cmp(other_card),
            (Hand::Pair(_), Hand::HighCard(_)) => Ordering::Greater,
            (Hand::Pair(_), _) => Ordering::Less,

            (Hand::TwoPairs(greater_pair, lesser_pair), Hand::TwoPairs(other_greater_pair, other_lesser_pair)) => {
                let res = greater_pair.cmp(other_greater_pair);
                if res == Ordering::Equal {
                    return lesser_pair.cmp(other_lesser_pair)
                }
                res
            },
            (Hand::TwoPairs(_, _), Hand::Pair(_)|Hand::HighCard(_)) => Ordering::Greater,
            (Hand::TwoPairs(_, _), _) => Ordering::Less,

            (Hand::ThreeOfAKind(card), Hand::ThreeOfAKind(other_card)) => card.cmp(other_card),
            (Hand::ThreeOfAKind(_), Hand::HighCard(_)|Hand::Pair(_)|Hand::TwoPairs(_, _)) => Ordering::Greater,
            (Hand::ThreeOfAKind(_), _) => Ordering::Less,

            (Hand::Straight(highest_card), Hand::Straight(other_highest_card)) => highest_card.cmp(other_highest_card),
            (Hand::Straight(_), Hand::HighCard(_)|Hand::Pair(_)|Hand::TwoPairs(_, _)|Hand::ThreeOfAKind(_)) => Ordering::Greater,
            (Hand::Straight(_), _) => Ordering::Less,

            (Hand::Flush(highest_card), Hand::Flush(other_highest_card)) => highest_card.cmp(other_highest_card),
            (Hand::Flush(_), Hand::FullHouse(_, _)|Hand::FourOfAKind(_)|Hand::StraightFlush(_)) => Ordering::Less,
            (Hand::Flush(_), _) => Ordering::Greater,

            (Hand::FullHouse(triple, pair), Hand::FullHouse(other_triple, other_pair)) => {
                let res = triple.cmp(other_triple);
                if res == Ordering::Equal {
                    return pair.cmp(other_pair)
                }
                res
            },
            (Hand::FullHouse(_, _), Hand::FourOfAKind(_)|Hand::StraightFlush(_)) => Ordering::Less,
            (Hand::FullHouse(_, _), _) => Ordering::Greater,

            (Hand::FourOfAKind(card), Hand::FourOfAKind(other_card)) => card.cmp(other_card),
            (Hand::FourOfAKind(_), Hand::StraightFlush(_)) => Ordering::Less,
            (Hand::FourOfAKind(_), _) => Ordering::Greater,

            (Hand::StraightFlush(highest_card), Hand::StraightFlush(other_highest_card)) => highest_card.cmp(other_highest_card),
            (Hand::StraightFlush(_), _) => Ordering::Greater,
        }
    }
}



pub struct PokerHand {
    cards: Vec<Card>,
    hands: Vec<Hand>
}

impl PokerHand {
    fn new(hand: &str) -> Self {
        let mut cards: Vec<Card> = hand.split_whitespace().filter_map(Card::new).collect();
        let hands = get_hands_from_cards(&mut cards);
        Self {
            cards,
            hands,
        }
    }
}

// impl<'a> FromIterator<&'a str> for PokerHand {
//     fn from_iter<I: IntoIterator<Item=&'a str>>(iter: I) -> Self {

//     }
// }

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        None
    }
}

impl PartialEq for PokerHand {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

// impl<'a> FromIterator<PokerHand> for Vec<&'a str> {
//     fn from_iter<I: IntoIterator<Item=PokerHand>>(iter: I) -> Self {
//         for i in iter {
//             i.cards()
//         }
//         Vec::new()
//     }
// }

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // let vec = hands.iter().map(|&c| PokerHand::new(c)).collect();
    // vec
    hands.to_vec()
}
