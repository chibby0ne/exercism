static OF_BEER: &str = "of beer";
static ON_THE_WALL: &str = "on the wall";
static TAKE: &str = "Take";
static DOWN_AND_PASS_IT_AROUND: &str = "down and pass it around, ";
static GO_TO_THE_STORE_AND_BUY_SOME_MORE: &str = "Go to the store and buy some more, ";
static NO_MORE: &str = "No more";
static BOTTLES: &str = "bottles";
const BOTTLE_LENGTH: usize = 6;
static IT: &str = "it";
static ONE: &str = "one";
static NINETY_NINE: &str = "99";

fn bottle_or_bottles(n: i32) -> &'static str {
    if n != 1 {
        BOTTLES
    } else {
        &BOTTLES[..BOTTLE_LENGTH]
    }
}

fn one_or_it(n: i32) -> &'static str {
    if n > 1 {
        ONE
    } else {
        IT
    }
}

fn no_more_or_more(n: i32) -> String {
    if n == 0 {
        NO_MORE.to_lowercase()
    } else {
        n.to_string()
    }
}

pub fn verse(n: i32) -> String {
    let mut verse = String::new();
    if n == 0 {
        verse.push_str(&format!(
            "{0} {1} {2} {3}, {4} {1} {2}.\n",
            NO_MORE,
            BOTTLES,
            OF_BEER,
            ON_THE_WALL,
            NO_MORE.to_lowercase()
        ));
        verse.push_str(GO_TO_THE_STORE_AND_BUY_SOME_MORE);
        verse.push_str(&format!(
            "{0} {1} {2} {3}.\n",
            NINETY_NINE, BOTTLES, OF_BEER, ON_THE_WALL
        ));
    } else {
        verse.push_str(&format!(
            "{0} {1} {2} {3}, {0} {1} {2}.\n",
            n,
            bottle_or_bottles(n),
            OF_BEER,
            ON_THE_WALL
        ));
        verse.push_str(&format!(
            "{0} {1} {2}",
            TAKE,
            one_or_it(n),
            DOWN_AND_PASS_IT_AROUND
        ));
        verse.push_str(&format!(
            "{0} {1} {2} {3}.\n",
            no_more_or_more(n - 1),
            bottle_or_bottles(n - 1),
            OF_BEER,
            ON_THE_WALL
        ));
    }
    return verse;
}

pub fn sing(start: i32, end: i32) -> String {
    let mut output = String::new();
    let mut i = start;
    while i > end {
        output.push_str(verse(i).as_str());
        output.push_str("\n");
        i -= 1;
    }
    output.push_str(verse(end).as_str());
    return output;
}
