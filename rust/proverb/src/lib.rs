pub fn build_proverb(list: Vec<&str>) -> String {
    // Early termination
    if list.len() == 0 {
        return String::new();
    }
    let mut output = Vec::new();
    let mut iter = list.iter();
    let mut prev = iter.next();
    let mut new = iter.next();
    while new != None {
        let sentence = format!(
            "For want of a {} the {} was lost.",
            prev.unwrap(),
            new.unwrap()
        );
        output.push(sentence);
        prev = new;
        new = iter.next();
    }
    output.push(format!("And all for the want of a {}.", list[0]));
    return output.join("\n");
}
