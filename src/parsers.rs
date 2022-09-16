/*
 *   Copyright (c) 2022 
 *   All rights reserved.
 */
#[cfg(test)]
mod tests;

use crate::AnsiSequence;

use core::convert::TryInto;
use heapless::Vec;
use nom::*;

macro_rules! tag_parser {
    ($sig:ident, $tag:expr, $ret:expr) => {
        named!(
            $sig<&str, AnsiSequence>,
            do_parse!(
                tag!($tag) >>
                ($ret)
            )
        );
    }
}

named!(
    parse_int<&str, u32>,
    map_res!(
        nom::digit,
        |s: &str| s.parse::<u32>()
    )
);

// TODO kind of ugly, would prefer to pass in the default so we could use it for
// all escapes with defaults (not just those that default to 1).
named!(
    parse_def_cursor_int<&str, u32>,
    map!(
        nom::digit0,
        |s: &str| s.parse::<u32>().unwrap_or(1)
    )
);

named!(
    cursor_pos<&str, AnsiSequence>,
    do_parse!(
        tag!("[")               >>
        x: parse_def_cursor_int >>
        opt!(tag!(";"))         >>
        y: parse_def_cursor_int >>
        alt!(
            tag!("H") |
            tag!("f")
        )               >>
        (AnsiSequence::CursorPos(x, y))
    )
);

named!(
    escape<&str, AnsiSequence>,
    do_parse!(
        tag!("\u{1b}") >>
        (AnsiSequence::Escape)
    )
);

named!(
    cursor_up<&str, AnsiSequence>,
    do_parse!(
        tag!("[")                >>
        am: parse_def_cursor_int >>
        tag!("A")                >>
        (AnsiSequence::CursorUp(am))
    )
);

named!(
    cursor_down<&str, AnsiSequence>,
    do_parse!(
        tag!("[")                >>
        am: parse_def_cursor_int >>
        tag!("B")                >>
        (AnsiSequence::CursorDown(am))
    )
);

named!(
    cursor_forward<&str, AnsiSequence>,
    do_parse!(
        tag!("[")                >>
        am: parse_def_cursor_int >>
        tag!("C")                >>
        (AnsiSequence::CursorForward(am))
    )
);

named!(
    cursor_backward<&str, AnsiSequence>,
    do_parse!(
        tag!("[")                >>
        am: parse_def_cursor_int >>
        tag!("D")                >>
        (AnsiSequence::CursorBackward(am))
    )
);

named!(
    graphics_mode1<&str, AnsiSequence>,
    do_parse!(
        tag!("[")       >>
        val: parse_int >>
        tag!("m")      >>
        val: expr_res!(val.try_into()) >>
        conv: expr_res!(Vec::from_slice(&[val])) >>
        (AnsiSequence::SetGraphicsMode(conv))
    )
);

named!(
    graphics_mode2<&str, AnsiSequence>,
    do_parse!(
        tag!("[")       >>
        val1: parse_int >>
        tag!(";")       >>
        val2: parse_int >>
        tag!("m")       >>
        val1: expr_res!(val1.try_into()) >>
        val2: expr_res!(val2.try_into()) >>
        conv: expr_res!(Vec::from_slice(&[
            val1,
            val2,
        ])) >>
        (AnsiSequence::SetGraphicsMode(conv))
    )
);

named!(
    graphics_mode3<&str, AnsiSequence>,
    do_parse!(
        tag!("[")       >>
        val1: parse_int >>
        tag!(";")       >>
        val2: parse_int >>
        tag!(";")       >>
        val3: parse_int >>
        tag!("m")       >>
        val1: expr_res!(val1.try_into()) >>
        val2: expr_res!(val2.try_into()) >>
        val3: expr_res!(val3.try_into()) >>
        conv: expr_res!(Vec::from_slice(&[
            val1,
            val2,
            val3,
        ])) >>
        (AnsiSequence::SetGraphicsMode(conv))
    )
);

named!(
    graphics_mode4<&str, AnsiSequence>,
    do_parse!(
        tag!("[m") >>
        (AnsiSequence::SetGraphicsMode(Vec::new()))
    )
);

named!(
    graphics_mode5<&str, AnsiSequence>,
    do_parse!(
        tag!("[")       >>
        val1: parse_int >>
        tag!(";")       >>
        val2: parse_int >>
        tag!(";")       >>
        val3: parse_int >>
        tag!(";")       >>
        val4: parse_int >>
        tag!(";")       >>
        val5: parse_int >>
        tag!("m")       >>
        val1: expr_res!(val1.try_into()) >>
        val2: expr_res!(val2.try_into()) >>
        val3: expr_res!(val3.try_into()) >>
        val4: expr_res!(val4.try_into()) >>
        val5: expr_res!(val5.try_into()) >>
        conv: expr_res!(Vec::from_slice(&[
            val1,
            val2,
            val3,
            val4,
            val5,
        ])) >>
        (AnsiSequence::SetGraphicsMode(conv))
    )
);

named!(
    graphics_mode<&str, AnsiSequence>,
    alt!(
          graphics_mode1
        | graphics_mode2
        | graphics_mode3
        | graphics_mode4
        | graphics_mode5
    )
);

named!(
    set_mode<&str, AnsiSequence>,
    do_parse!(
        tag!("[=")                       >>
        mode: parse_int                  >>
        conv: expr_res!(mode.try_into()) >>
        tag!("h")                        >>
        (AnsiSequence::SetMode(conv))
    )
);

named!(
    reset_mode<&str, AnsiSequence>,
    do_parse!(
        tag!("[=")                       >>
        mode: parse_int                  >>
        conv: expr_res!(mode.try_into()) >>
        tag!("l")                        >>
        (AnsiSequence::ResetMode(conv))
    )
);

named!(
    set_top_and_bottom<&str, AnsiSequence>,
    do_parse!(
        tag!("[")    >>
        x: parse_int >>
        tag!(";")    >>
        y: parse_int >>
        tag!("r")    >>
        (AnsiSequence::SetTopAndBottom(x, y))
    )
);

tag_parser!(cursor_save, "[s", AnsiSequence::CursorSave);
tag_parser!(cursor_restore, "[u", AnsiSequence::CursorRestore);
tag_parser!(erase_display, "[2J", AnsiSequence::EraseDisplay);
tag_parser!(erase_line, "[K", AnsiSequence::EraseLine);
tag_parser!(hide_cursor, "[?25l", AnsiSequence::HideCursor);
tag_parser!(show_cursor, "[?25h", AnsiSequence::ShowCursor);
tag_parser!(cursor_to_app, "[?1h", AnsiSequence::CursorToApp);
tag_parser!(set_new_line_mode, "[20h", AnsiSequence::SetNewLineMode);
tag_parser!(set_col_132, "[?3h", AnsiSequence::SetCol132);
tag_parser!(set_smooth_scroll, "[?4h", AnsiSequence::SetSmoothScroll);
tag_parser!(set_reverse_video, "[?5h", AnsiSequence::SetReverseVideo);
tag_parser!(set_origin_rel, "[?6h", AnsiSequence::SetOriginRelative);
tag_parser!(set_auto_wrap, "[?7h", AnsiSequence::SetAutoWrap);
tag_parser!(set_auto_repeat, "[?8h", AnsiSequence::SetAutoRepeat);
tag_parser!(set_interlacing, "[?9h", AnsiSequence::SetInterlacing);
tag_parser!(set_linefeed, "[20l", AnsiSequence::SetLineFeedMode);
tag_parser!(set_cursorkey, "[?1l", AnsiSequence::SetCursorKeyToCursor);
tag_parser!(set_vt52, "[?2l", AnsiSequence::SetVT52);
tag_parser!(set_col80, "[?3l", AnsiSequence::SetCol80);
tag_parser!(set_jump_scroll, "[?4l", AnsiSequence::SetJumpScrolling);
tag_parser!(set_normal_video, "[?5l", AnsiSequence::SetNormalVideo);
tag_parser!(set_origin_abs, "[?6l", AnsiSequence::SetOriginAbsolute);
tag_parser!(reset_auto_wrap, "[?7l", AnsiSequence::ResetAutoWrap);
tag_parser!(reset_auto_repeat, "[?8l", AnsiSequence::ResetAutoRepeat);
tag_parser!(reset_interlacing, "[?9l", AnsiSequence::ResetInterlacing);

tag_parser!(set_alternate_keypad, "=", AnsiSequence::SetAlternateKeypad);
tag_parser!(set_numeric_keypad, ">", AnsiSequence::SetNumericKeypad);
tag_parser!(set_uk_g0, "(A", AnsiSequence::SetUKG0);
tag_parser!(set_uk_g1, ")A", AnsiSequence::SetUKG1);
tag_parser!(set_us_g0, "(B", AnsiSequence::SetUSG0);
tag_parser!(set_us_g1, ")B", AnsiSequence::SetUSG1);
tag_parser!(set_g0_special, "(0", AnsiSequence::SetG0SpecialChars);
tag_parser!(set_g1_special, ")0", AnsiSequence::SetG1SpecialChars);
tag_parser!(set_g0_alternate, "(1", AnsiSequence::SetG0AlternateChar);
tag_parser!(set_g1_alternate, ")1", AnsiSequence::SetG1AlternateChar);
tag_parser!(set_g0_graph, "(2", AnsiSequence::SetG0AltAndSpecialGraph);
tag_parser!(set_g1_graph, ")2", AnsiSequence::SetG1AltAndSpecialGraph);
tag_parser!(set_single_shift2, "N", AnsiSequence::SetSingleShift2);
tag_parser!(set_single_shift3, "O", AnsiSequence::SetSingleShift3);

named!(
    combined<&str, AnsiSequence>,
    alt!(
          escape
        | cursor_pos
        | cursor_up
        | cursor_down
        | cursor_forward
        | cursor_backward
        | cursor_save
        | cursor_restore
        | erase_display
        | erase_line
        | graphics_mode
        | set_mode
        | reset_mode
        | hide_cursor
        | show_cursor
        | cursor_to_app
        | set_new_line_mode
        | set_col_132
        | set_smooth_scroll
        | set_reverse_video
        | set_origin_rel
        | set_auto_wrap
        | set_auto_repeat
        | set_interlacing
        | set_linefeed
        | set_cursorkey
        | set_vt52
        | set_col80
        | set_jump_scroll
        | set_normal_video
        | set_origin_abs
        | reset_auto_wrap
        | reset_auto_repeat
        | reset_interlacing
        | set_top_and_bottom
        | set_alternate_keypad
        | set_numeric_keypad
        | set_uk_g0
        | set_uk_g1
        | set_us_g0
        | set_us_g1
        | set_g0_special
        | set_g1_special
        | set_g0_alternate
        | set_g1_alternate
        | set_g0_graph
        | set_g1_graph
        | set_single_shift2
        | set_single_shift3
    )
);

named!(
    pub parse_escape<&str, AnsiSequence>,
    do_parse!(
        tag!("\u{1b}")    >>
        seq: combined     >>
        (seq)
    )
);
