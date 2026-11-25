//! How the command bar matches what was typed against its commands.

/// How well `title` (or its `group`) matches `query`: a prefix beats a
/// word start, which beats a substring, which beats the letters in order.
/// `None`: no match.
pub fn score(query: &str, title: &str, group: &str) -> Option<u32> {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return Some(0);
    }
    let title = title.to_lowercase();
    if title.starts_with(&query) {
        return Some(400);
    }
    if title
        .split(|c: char| !c.is_alphanumeric())
        .any(|word| word.starts_with(&query))
    {
        return Some(300);
    }
    if title.contains(&query) {
        return Some(200);
    }
    if group.to_lowercase().starts_with(&query) {
        return Some(150);
    }
    let mut letters = title.chars();
    query
        .chars()
        .filter(|c| !c.is_whitespace())
        .all(|q| letters.any(|t| t == q))
        .then_some(100)
}

/// The indices of `titles` matching `query`, best first, ties in list
/// order.
pub fn rank(query: &str, entries: &[(String, String)]) -> Vec<usize> {
    let mut hits: Vec<(u32, usize)> = entries
        .iter()
        .enumerate()
        .filter_map(|(at, (title, group))| score(query, title, group).map(|s| (s, at)))
        .collect();
    hits.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    hits.into_iter().map(|(_, at)| at).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entries(titles: &[(&str, &str)]) -> Vec<(String, String)> {
        titles
            .iter()
            .map(|(t, g)| ((*t).to_owned(), (*g).to_owned()))
            .collect()
    }

    #[test]
    fn a_prefix_ranks_above_a_word_start_and_letters_in_order() {
        let list = entries(&[
            ("Query devtools", "Go to"),
            ("Diagnostics", "Go to"),
            ("Data", "Go to"),
            ("Device", "Go to"),
        ]);
        assert_eq!(rank("d", &list), vec![1, 2, 3, 0]);
        assert_eq!(rank("dev", &list), vec![3, 0]);
        assert_eq!(rank("qdt", &list), vec![0]);
        assert_eq!(rank("zz", &list), Vec::<usize>::new());
    }

    #[test]
    fn an_empty_query_keeps_the_list_order() {
        let list = entries(&[("Home", "Go to"), ("Undo", "Action")]);
        assert_eq!(rank("  ", &list), vec![0, 1]);
        assert_eq!(rank("act", &list), vec![1]);
    }
}
