//! Rich text support with formatting runs
//!
//! Allows applying different text styles to different regions of text.

use super::{TextStyle, FontWeight};
use serde::{Deserialize, Serialize};

/// A run of text with consistent formatting
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextRun {
    pub text: String,
    pub style: TextStyle,
}

/// Rich text composed of multiple runs with different styles
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichText {
    pub runs: Vec<TextRun>,
}

impl RichText {
    /// Create rich text from a plain string with a base style
    pub fn from_plain(text: &str, style: TextStyle) -> Self {
        Self {
            runs: vec![TextRun {
                text: text.to_string(),
                style,
            }],
        }
    }

    /// Get the complete text without formatting
    pub fn get_plain_text(&self) -> String {
        self.runs.iter().map(|run| run.text.as_str()).collect::<Vec<_>>().join("")
    }

    /// Get the total text length in characters
    pub fn text_length(&self) -> usize {
        self.runs.iter().map(|run| run.text.len()).sum()
    }

    /// Validate that indices are within bounds
    fn validate_range(&self, start: usize, end: usize) -> bool {
        let len = self.text_length();
        start <= len && end <= len
    }

    /// Normalize range (swap if start > end)
    fn normalize_range(&self, start: usize, end: usize) -> (usize, usize) {
        if start <= end {
            (start, end)
        } else {
            (end, start)
        }
    }

    /// Apply bold to a character range
    pub fn apply_bold(&mut self, start: usize, end: usize) {
        self.apply_style_to_range(start, end, |style| {
            style.weight = FontWeight::Bold;
        });
    }

    /// Remove bold from a character range
    pub fn remove_bold(&mut self, start: usize, end: usize) {
        self.apply_style_to_range(start, end, |style| {
            style.weight = FontWeight::Regular;
        });
    }

    /// Apply italic to a character range
    pub fn apply_italic(&mut self, start: usize, end: usize) {
        self.apply_style_to_range(start, end, |style| {
            style.italic = true;
        });
    }

    /// Remove italic from a character range
    pub fn remove_italic(&mut self, start: usize, end: usize) {
        self.apply_style_to_range(start, end, |style| {
            style.italic = false;
        });
    }

    /// Apply underline to a character range
    pub fn apply_underline(&mut self, start: usize, end: usize) {
        self.apply_style_to_range(start, end, |style| {
            style.underline = true;
        });
    }

    /// Remove underline from a character range
    pub fn remove_underline(&mut self, start: usize, end: usize) {
        self.apply_style_to_range(start, end, |style| {
            style.underline = false;
        });
    }

    /// Apply strikethrough to a character range
    pub fn apply_strikethrough(&mut self, start: usize, end: usize) {
        self.apply_style_to_range(start, end, |style| {
            style.strikethrough = true;
        });
    }

    /// Remove strikethrough from a character range
    pub fn remove_strikethrough(&mut self, start: usize, end: usize) {
        self.apply_style_to_range(start, end, |style| {
            style.strikethrough = false;
        });
    }

    /// Apply a style modification function to a character range
    fn apply_style_to_range<F>(&mut self, start: usize, end: usize, modifier: F)
    where
        F: Fn(&mut TextStyle),
    {
        // Normalize range (swap if needed)
        let (start, end) = self.normalize_range(start, end);

        // Validate range bounds
        if !self.validate_range(start, end) {
            // Range is out of bounds, return without applying changes
            return;
        }

        // If start == end, nothing to do
        if start == end {
            return;
        }

        let mut char_index = 0;
        let mut new_runs = Vec::new();

        for run in &self.runs {
            let run_start = char_index;
            let run_end = char_index + run.text.len();

            if run_end <= start || run_start >= end {
                // No overlap - keep run as is
                new_runs.push(run.clone());
            } else {
                // There's overlap - need to split the run
                let overlap_start = start.saturating_sub(run_start);
                let overlap_end = (end - run_start).min(run.text.len());

                // Part before overlap
                if overlap_start > 0 {
                    new_runs.push(TextRun {
                        text: run.text[..overlap_start].to_string(),
                        style: run.style.clone(),
                    });
                }

                // Overlapped part - apply modification
                let mut modified_style = run.style.clone();
                modifier(&mut modified_style);
                new_runs.push(TextRun {
                    text: run.text[overlap_start..overlap_end].to_string(),
                    style: modified_style,
                });

                // Part after overlap
                if overlap_end < run.text.len() {
                    new_runs.push(TextRun {
                        text: run.text[overlap_end..].to_string(),
                        style: run.style.clone(),
                    });
                }
            }

            char_index = run_end;
        }

        // Merge consecutive runs with the same style
        self.runs = self.merge_runs(new_runs);
    }

    /// Merge consecutive runs that have the same style
    fn merge_runs(&self, runs: Vec<TextRun>) -> Vec<TextRun> {
        if runs.is_empty() {
            return runs;
        }

        let mut merged = Vec::new();
        let mut current_run = runs[0].clone();

        for run in &runs[1..] {
            if styles_equal(&current_run.style, &run.style) {
                // Merge with current run
                current_run.text.push_str(&run.text);
            } else {
                // Start a new run
                merged.push(current_run);
                current_run = run.clone();
            }
        }
        merged.push(current_run);

        merged
    }

    /// Check if text has bold formatting in the range
    pub fn has_bold(&self, start: usize, end: usize) -> bool {
        self.check_style(start, end, |style| style.weight == FontWeight::Bold)
    }

    /// Check if text has italic formatting in the range
    pub fn has_italic(&self, start: usize, end: usize) -> bool {
        self.check_style(start, end, |style| style.italic)
    }

    /// Check if text has underline formatting in the range
    pub fn has_underline(&self, start: usize, end: usize) -> bool {
        self.check_style(start, end, |style| style.underline)
    }

    /// Check if a style property matches across the range
    fn check_style<F>(&self, start: usize, end: usize, checker: F) -> bool
    where
        F: Fn(&TextStyle) -> bool,
    {
        let start = start.min(end);
        let end = start.max(end);

        let mut char_index = 0;
        for run in &self.runs {
            let run_start = char_index;
            let run_end = char_index + run.text.len();

            if run_end > start && run_start < end {
                if !checker(&run.style) {
                    return false;
                }
            }

            char_index = run_end;
        }

        true
    }
}

/// Check if two TextStyles are equal
fn styles_equal(a: &TextStyle, b: &TextStyle) -> bool {
    a.font_family == b.font_family
        && a.font_size == b.font_size
        && a.weight == b.weight
        && a.alignment == b.alignment
        && a.color == b.color
        && a.italic == b.italic
        && a.underline == b.underline
        && a.strikethrough == b.strikethrough
        && a.background_color == b.background_color
        && a.line_height == b.line_height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rich_text_from_plain() {
        let style = TextStyle::default();
        let text = RichText::from_plain("Hello World", style.clone());

        assert_eq!(text.get_plain_text(), "Hello World");
        assert_eq!(text.runs.len(), 1);
    }

    #[test]
    fn test_apply_bold() {
        let style = TextStyle::default();
        let mut text = RichText::from_plain("Hello World", style);

        text.apply_bold(0, 5);

        assert!(text.has_bold(0, 5));
        assert!(!text.has_bold(6, 11));
    }

    #[test]
    fn test_apply_italic() {
        let style = TextStyle::default();
        let mut text = RichText::from_plain("Hello World", style);

        text.apply_italic(6, 11);

        assert!(!text.has_italic(0, 5));
        assert!(text.has_italic(6, 11));
    }

    #[test]
    fn test_multiple_formatting() {
        let style = TextStyle::default();
        let mut text = RichText::from_plain("Hello World", style);

        text.apply_bold(0, 5);
        text.apply_italic(6, 11);
        text.apply_underline(0, 11);

        assert!(text.has_bold(0, 5));
        assert!(text.has_italic(6, 11));
        assert!(text.has_underline(0, 11));
    }

    #[test]
    fn test_run_merging() {
        let style = TextStyle::default();
        let mut text = RichText::from_plain("Hello", style);

        text.apply_bold(0, 5);
        text.remove_bold(0, 5);

        // Should merge back to single run
        assert_eq!(text.runs.len(), 1);
    }
}
