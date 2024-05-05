use minesweeper::annotate;

pub fn main() {
    run_test(&["1*22*1", "12*322", " 123*2", "112*4*", "1*22*2", "111111"]);
}

fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}

fn run_test(test_case: &[&str]) {
    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    println!(
        "cleaned: {:?}\ncleaned_strs: {:?}\nexpected: {:?}",
        cleaned, cleaned_strs, expected
    );
    assert_eq!(expected, annotate(&cleaned_strs));
}
