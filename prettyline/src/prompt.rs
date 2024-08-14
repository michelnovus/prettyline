// [MIT License] Copyright (c) 2024 Michel Novus
//! Defines printable structures.

use anstyle::{Color, Style};
use std::fmt::Display;

/// Main element that allows to encapsulate each piece of the prompt.
///
/// It contains three parts, the left character, the text in the middle
/// and the right character; the left and right characters are extremes
/// of the segment and should be one of those defined in the
/// `constants::symbols` module.
#[derive(Debug)]
pub struct Segment<'a> {
    pub left: Option<Chunk<'a>>,
    pub center: Chunk<'a>,
    pub right: Option<Chunk<'a>>,
}
impl<'a> Display for Segment<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let left: String = match self.left.as_ref() {
            Some(chunk) => chunk.to_string(),
            None => String::new(),
        };
        let center: String = self.center.to_string();
        let right: String = match self.right.as_ref() {
            Some(chunk) => chunk.to_string(),
            None => String::new(),
        };
        write!(f, "{}{}{}", left, center, right)
    }
}

/// Defines whether the text should be thick (bold) or thin (dimm).
#[derive(Debug)]
pub enum TextWeight {
    Bold,
    Dimm,
}

/// Defines how the text should appear.
#[derive(Debug, Default)]
pub struct Chunk<'a> {
    /// The text.
    pub value: &'a str,
    /// The text has a bold, dimm or normal (when it is `None`).
    pub weight: Option<TextWeight>,
    /// The text color itself.
    pub fg_color: Option<Color>,
    /// The background color of text.
    pub bg_color: Option<Color>,
    /// Adds a spaces arround text.
    pad: bool,
}
impl<'a> Chunk<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            value: text,
            weight: None,
            fg_color: None,
            bg_color: None,
            pad: false,
        }
    }
    /// Sets the text color.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg_color = Some(color);
        self
    }
    /// Sets the background text color.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }
    /// Sets text weight (bold, dimm or normal).
    pub fn weight(mut self, weight: TextWeight) -> Self {
        self.weight = Some(weight);
        self
    }
    /// Turn a spaces around text on or off.
    pub fn pad(mut self) -> Self {
        self.pad = !self.pad;
        self
    }
}
impl<'a> Display for Chunk<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut style =
            Style::new().fg_color(self.fg_color).bg_color(self.bg_color);
        if let Some(weight) = &self.weight {
            style = match weight {
                TextWeight::Bold => style.bold(),
                TextWeight::Dimm => style.dimmed(),
            }
        }
        if self.pad {
            write!(f, "{style} {} {style:#}", self.value)
        } else {
            write!(f, "{style}{}{style:#}", self.value)
        }
    }
}
