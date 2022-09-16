/*
 *   Copyright (c) 2022
 *   All rights reserved.
 */
use crate::{
    enums::{AnsiSequence, Output},
    parsers::parse_escape,
    traits::AnsiParser,
};

use std::fmt::Write;

macro_rules! test_parser {
    ($name:ident, $string:expr) => {
        #[test]
        fn $name() {
            let mut buff = String::new();
            let ret = parse_escape($string);

            assert!(ret.is_ok());
            let ret = ret.unwrap().1;

            write!(&mut buff, "{}", ret).unwrap();

            assert_eq!(buff, $string);
        }
    };
}

macro_rules! test_def_val_parser {
    ($name:ident, $string:expr) => {
        #[test]
        fn $name() {
            let mut buff = String::new();
            let ret = parse_escape($string);

            assert!(ret.is_ok());
            let ret = ret.unwrap().1;

            write!(&mut buff, "{}", ret).unwrap();

            let ret2 = parse_escape(&buff);
            assert!(ret2.is_ok());

            let ret2 = ret2.unwrap().1;
            assert_eq!(ret, ret2);
        }
    };
}

test_def_val_parser!(cursor_pos_default, "\u{1b}[H");
test_def_val_parser!(cursor_pos, "\u{1b}[10;5H");
test_def_val_parser!(cursor_up_default, "\u{1b}[A");
test_def_val_parser!(cursor_up, "\u{1b}[5A");
test_def_val_parser!(cursor_down, "\u{1b}[5B");
test_def_val_parser!(cursor_forward, "\u{1b}[5C");
test_def_val_parser!(cursor_backward, "\u{1b}[5D");
test_parser!(cursor_save, "\u{1b}[s");
test_parser!(cursor_restore, "\u{1b}[u");

test_parser!(erase_display, "\u{1b}[2J");
test_parser!(erase_line, "\u{1b}[K");

test_parser!(set_video_mode_a, "\u{1b}[4m");
test_parser!(set_video_mode_b, "\u{1b}[4;42m");
test_parser!(set_video_mode_c, "\u{1b}[4;31;42m");
test_parser!(set_video_mode_d, "\u{1b}[4;31;42;42;42m");

test_parser!(reset_mode, "\u{1b}[=13l");
test_parser!(set_mode, "\u{1b}[=7h");

test_parser!(show_cursor, "\u{1b}[?25h");
test_parser!(hide_cursor, "\u{1b}[?25l");
test_parser!(cursor_to_app, "\u{1b}[?1h");

test_parser!(set_newline_mode, "\u{1b}[20h");
test_parser!(set_column_132, "\u{1b}[?3h");
test_parser!(set_smooth_scroll, "\u{1b}[?4h");
test_parser!(set_reverse_video, "\u{1b}[?5h");
test_parser!(set_origin_rel, "\u{1b}[?6h");
test_parser!(set_auto_wrap, "\u{1b}[?7h");
test_parser!(set_auto_repeat, "\u{1b}[?8h");
test_parser!(set_interlacing, "\u{1b}[?9h");

test_parser!(set_cursor_key_to_cursor, "\u{1b}[?1l");

test_parser!(set_linefeed, "\u{1b}[20l");
test_parser!(set_vt52, "\u{1b}[?2l");
test_parser!(set_col80, "\u{1b}[?3l");
test_parser!(set_jump_scroll, "\u{1b}[?4l");
test_parser!(set_normal_video, "\u{1b}[?5l");
test_parser!(set_origin_abs, "\u{1b}[?6l");
test_parser!(reset_auto_wrap, "\u{1b}[?7l");
test_parser!(reset_auto_repeat, "\u{1b}[?8l");
test_parser!(reset_interlacing, "\u{1b}[?9l");

test_parser!(set_alternate_keypad, "\u{1b}=");
test_parser!(set_numeric_keypad, "\u{1b}>");
test_parser!(set_uk_g0, "\u{1b}(A");
test_parser!(set_uk_g1, "\u{1b})A");
test_parser!(set_us_g0, "\u{1b}(B");
test_parser!(set_us_g1, "\u{1b})B");
test_parser!(set_g0_special, "\u{1b}(0");
test_parser!(set_g1_special, "\u{1b})0");
test_parser!(set_g0_alternate, "\u{1b}(1");
test_parser!(set_g1_alternate, "\u{1b})1");
test_parser!(set_g0_graph, "\u{1b}(2");
test_parser!(set_g1_graph, "\u{1b})2");
test_parser!(set_single_shift2, "\u{1b}N");
test_parser!(set_single_shift3, "\u{1b}O");

#[test]
fn test_parser_iterator() {
    let count = "\x1b[=25l\x1b[=7l\x1b[0m\x1b[36m\x1b[1m-`"
        .ansi_parse()
        .count();

    assert_eq!(count, 6);
}

#[test]
fn test_parser_iterator_failure() {
    let count = "\x1b[=25l\x1b[=7l\x1b[0m\x1b[36;1;15;2m\x1b[1m-`"
        .ansi_parse()
        .count();

    assert_eq!(count, 6);
}

#[test]
fn test_default_value() {
    let mut strings: Vec<Output> = "\x1b[H\x1b[123456H\x1b[;123456H\x1b[7asd;1234H\x1b[a;sd7H"
        .ansi_parse()
        .collect();

    strings = dbg!(strings);

    assert_eq!(strings.len(), 5);
    assert_eq!(strings[0], Output::Escape(AnsiSequence::CursorPos(1, 1)));
    assert_eq!(
        strings[1],
        Output::Escape(AnsiSequence::CursorPos(123456, 1))
    );
    assert_eq!(
        strings[2],
        Output::Escape(AnsiSequence::CursorPos(1, 123456))
    );
    assert_eq!(strings[3], Output::TextBlock("\x1b[7asd;1234H"));
    assert_eq!(strings[4], Output::TextBlock("\x1b[a;sd7H"));
}

#[test]
fn test_escape() {
    let parts: Vec<_> = "\x1b\x1b[33mFoobar".ansi_parse().collect();
    assert_eq!(
        parts,
        vec![
            Output::Escape(AnsiSequence::Escape),
            Output::TextBlock("[33mFoobar")
        ]
    );
}

#[test]
fn test_lolcat() {
    let mut parts: Vec<Output> = "\u{1b}[38;2;51;254;77mS\u{1b}[39m\u{1b}[38;2;52;254;77mt\u{1b}[39m\u{1b}[38;2;52;254;77ma\u{1b}[39m\u{1b}[38;2;52;254;76mt\u{1b}[39m\u{1b}[38;2;53;254;76me\u{1b}[39m\u{1b}[38;2;53;254;76m \u{1b}[39m\u{1b}[38;2;53;254;75m{\u{1b}[39m\u{1b}[38;2;54;254;75m \u{1b}[39m\u{1b}[38;2;54;254;74ms\u{1b}[39m\u{1b}[38;2;54;254;74mt\u{1b}[39m\u{1b}[38;2;55;254;74ma\u{1b}[39m\u{1b}[38;2;55;254;73mc\u{1b}[39m\u{1b}[38;2;56;254;73mk\u{1b}[39m\u{1b}[38;2;56;254;72m:\u{1b}[39m\u{1b}[38;2;56;254;72m \u{1b}[39m\u{1b}[38;2;57;254;72m[\u{1b}[39m\u{1b}[38;2;57;254;71m0\u{1b}[39m\u{1b}[38;2;57;254;71m]\u{1b}[39m\u{1b}[38;2;58;254;71m \u{1b}[39m\u{1b}[38;2;58;254;70m}\u{1b}[39m".ansi_parse().collect();

    parts = dbg!(parts);

    dbg!(text_blocks(&parts));
    fn text_blocks<'a>(parts: &'a [Output]) -> Vec<&'a str> {
        parts
            .iter()
            .filter_map(|part| match part {
                Output::TextBlock(text) => Some(*text),
                _ => None,
            })
            .collect()
    }
}

#[test]
fn test_lolcat_2() {
    let test_data = "\u{1b}[38;2;51;254;77mS\u{1b}[39m\u{1b}[38;2;52;254;77mt\u{1b}[39m\u{1b}[38;2;52;254;77ma\u{1b}[39m\u{1b}[38;2;52;254;76mt\u{1b}[39m\u{1b}[38;2;53;254;76me\u{1b}[39m\u{1b}[38;2;53;254;76m \u{1b}[39m\u{1b}[38;2;53;254;75m{\u{1b}[39m\u{1b}[38;2;54;254;75m \u{1b}[39m\u{1b}[38;2;54;254;74ms\u{1b}[39m\u{1b}[38;2;54;254;74mt\u{1b}[39m\u{1b}[38;2;55;254;74ma\u{1b}[39m\u{1b}[38;2;55;254;73mc\u{1b}[39m\u{1b}[38;2;56;254;73mk\u{1b}[39m\u{1b}[38;2;56;254;72m:\u{1b}[39m\u{1b}[38;2;56;254;72m \u{1b}[39m\u{1b}[38;2;57;254;72m[\u{1b}[39m\u{1b}[38;2;57;254;71m0\u{1b}[39m\u{1b}[38;2;57;254;71m]\u{1b}[39m\u{1b}[38;2;58;254;71m \u{1b}[39m\u{1b}[38;2;58;254;70m}\u{1b}[39m";
    let unparsed_ansi_text = ANSIText::new(test_data);
    dbg!(unparsed_ansi_text.segments());
    dbg!(unparsed_ansi_text.segments().len());
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ANSITextSegment<'a> {
    pub vec_parts: Vec<&'a Output<'a>>,
    pub unicode_width: usize,
}

impl ANSITextSegment<'_> {
    pub fn new() -> Self {
        Self {
            vec_parts: vec![],
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ANSIText<'a> {
    pub ansi_text: &'a str,
    pub parts: Vec<Output<'a>>,
}

impl<'a> ANSIText<'a> {
    /// Given an unparsed ANSI text &[str], parse it and return an [ANSIText].
    pub fn new(ansi_text: &'a str) -> Self {
        let parts: Vec<Output> = ansi_text.ansi_parse().collect();
        Self { ansi_text, parts }
    }

    /// Returns all the segments that are delimited by an [Output::TextBlock].
    pub fn segments(&'a self) -> Vec<ANSITextSegment<'a>> {
        let mut vec_segments = Vec::new();

        let mut current_segment = ANSITextSegment::new();

        for part in &self.parts {
            match part {
                Output::TextBlock(_text) => {
                    current_segment.vec_parts.push(part);
                    // Start a new segment & save the current one.
                    vec_segments.push(current_segment);
                    current_segment = ANSITextSegment::new();
                }
                Output::Escape(_ansi_sequence) => {
                    current_segment.vec_parts.push(part);
                }
            }
        }

        // Take care of dangling current_segment.
        if !vec_segments.contains(&current_segment) {
            vec_segments.push(current_segment);
        }

        // Calculate the unicode_width of each segment.
        for segment in &mut vec_segments {
            for part in &segment.vec_parts {
                if let Output::TextBlock(text) = part {
                    segment.unicode_width += unicode_width::UnicodeWidthStr::width(*text);
                }
            }
        }

        vec_segments
    }

    /// Returns the maximum number of segments that will fit in the given display column width.
    pub fn segments_fit_in_display_col(max_col: usize) -> Vec<ANSITextSegment<'a>> {
        // TK: impl this
        unimplemented!()
    }
}
