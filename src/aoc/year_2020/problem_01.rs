type Entry = isize;
type EntryVec = std::vec::Vec<Entry>;

fn get_sorted_entries_vec(input: impl std::io::BufRead) -> EntryVec {
    let mut entries: EntryVec = input
        .lines()
        .map(|line| line.unwrap().parse::<Entry>().unwrap())
        .collect();
    entries.sort();
    entries
}

fn find_two_entries_with_the_sum(entries: &[Entry], sum: Entry) -> Option<(Entry, Entry)> {
    let left = entries.first();
    let right = entries.last();

    if let Some((l, r)) = left.zip(right) {
        use std::cmp::Ordering;
        match sum.cmp(&(l + r)) {
            Ordering::Equal => Some((*l, *r)),
            Ordering::Greater => find_two_entries_with_the_sum(&entries[1..], sum),
            Ordering::Less => find_two_entries_with_the_sum(&entries[..entries.len() - 1], sum),
        }
    } else {
        None
    }
}

fn find_three_entries_with_the_sum(entries: &[Entry], sum: Entry) -> Option<(Entry, Entry, Entry)> {
    for (index, &first_entry) in entries.iter().enumerate() {
        let rest_sum = sum - first_entry;
        if let Some((second_entry, third_entry)) =
            find_two_entries_with_the_sum(&entries[index + 1..], rest_sum)
        {
            return Some((first_entry, second_entry, third_entry));
        }
    }

    None
}

pub fn solve_a(input: impl std::io::BufRead) -> Entry {
    let entries = get_sorted_entries_vec(input);

    let (first_entry, second_entry) =
        find_two_entries_with_the_sum(&entries, 2020).expect("Solution not found");

    first_entry * second_entry
}

pub fn solve_b(input: impl std::io::BufRead) -> Entry {
    let entries = get_sorted_entries_vec(input);

    let (first_entry, second_entry, third_entry) =
        find_three_entries_with_the_sum(&entries, 2020).expect("Solution not found");

    first_entry * second_entry * third_entry
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "1721\n979\n366\n299\n675\n1456";

    #[test]
    fn check_a() {
        assert_eq!(514579, super::solve_a(TEST_INPUT.as_bytes()));
    }

    #[test]
    fn check_b() {
        assert_eq!(241861950, super::solve_b(TEST_INPUT.as_bytes()));
    }
}
