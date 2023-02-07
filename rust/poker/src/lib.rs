use std::cmp::Ordering;

#[derive(Debug)]
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

#[derive(Debug, PartialOrd, PartialEq, Eq)]
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

#[derive(Debug)]
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


#[derive(Debug, Eq, PartialEq)]
enum Hand {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPairs,
    Pair,
    HighCard,
}

impl Hand {
    fn new(hand: &str) -> Option<Self> {
        None
    }
}


pub struct PokerHand {
    cards: Vec<Card>
}

impl PokerHand {
    fn new(hand: &str) -> Self {
        Self {
            cards: hand.split_whitespace().filter_map(Card::new).collect()
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
