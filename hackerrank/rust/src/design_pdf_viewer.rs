const LOW: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

#[allow(dead_code)]
/// Calculate the area of highlighting on strings
///
/// * `h`: Array of heights of low-case letters
/// * `word`: a string to be highlightened
///
/// Challenge:
/// https://www.hackerrank.com/challenges/designer-pdf-viewer/problem

fn design_pdf_viewer(h: &[i32], word: &str) -> i32 {
    let max_height = word
        .chars()
        .map(|w| {
            if let Some(position) = LOW.iter().position(|&c| c == w) {
                h[position]
            } else {
                panic!("Error")
            }
        })
        .max();

    if let Some(height) = max_height {
        height * (word.len() as i32)
    } else {
        panic!("Error")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let heights = [
            1, 3, 1, 3, 1, 4, 1, 3, 2, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        ];
        assert_eq!(design_pdf_viewer(&heights, "abc"), 9);
    }
}
