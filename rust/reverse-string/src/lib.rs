#[cfg(feature = "grapheme")]
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    #[cfg(feature = "grapheme")]
    {
        input.graphemes(true).rev().collect()
    }
    #[cfg(not(feature = "grapheme"))]
    {
        input.chars().rev().collect()
    }
}
