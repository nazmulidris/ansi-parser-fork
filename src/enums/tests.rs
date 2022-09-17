/*
 *   Copyright (c) 2022
 *   All rights reserved.
 */
use super::*;

use std::fmt::Write;

#[test]
fn test_cursor_pos() {
    let pos = AnsiSequence::CursorPos(5, 20);
    let mut buff = String::new();

    write!(&mut buff, "{}", pos).expect("failed to write");

    assert_eq!(dbg!(buff), "\x1b[5;20H");
}

#[test]
fn test_write_ansi_seq_to_string() {
    let out = vec![
        Output::Escape(AnsiSequence::CursorPos(5, 20)),
        Output::TextBlock("Hello World!"),
    ];

    let mut buff = String::new();
    // iterate out and write to buff
    for o in out {
        write!(&mut buff, "{}", o).expect("failed to write");
    }

    assert_eq!(dbg!(buff), "\x1b[5;20HHello World!");
}
