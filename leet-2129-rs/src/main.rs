struct Solution;

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        capitalize_titles(title)
    }
}

// pub fn capitalize_one_title(title: String) -> String {
//     let mut chars = title.chars();
//     let head = chars.next().unwrap().to_ascii_uppercase();
//     let tail: String = chars.map(|c| c.to_ascii_lowercase()).collect();
//
//     let mut res = String::new();
//     res.push(head);
//     res.push_str(tail.as_str());
//
//     res
// }

// Idiomatic way of capitalize a string from https://stackoverflow.com/a/38406885
fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => {
            f.to_uppercase().collect::<String>()
                + c.map(|c| c.to_ascii_lowercase())
                    .collect::<String>()
                    .as_str()
        }
    }
}

pub fn capitalize_titles(title: String) -> String {
    title
        .split(" ")
        .map(|s| {
            if s.len() <= 2 {
                s.to_ascii_lowercase()
            } else {
                capitalize_first(s)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let res = Solution::capitalize_title("a quick brown fox jumps over a lazy dog".to_string());
    println!("{}", res);
}
