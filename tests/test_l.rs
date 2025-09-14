#[macro_use]
mod helper;

#[macro_use]
mod helper_l;

mod test_0_l {
    use libaki_unbody::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    #[test]
    fn test_help() {
        let (r, sioe) = do_execute!(["-H"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_invalid_opt() {
        let (r, sioe) = do_execute!(["-z"]);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": ",
                "Invalid option: z\n",
                "Missing option: h or t\n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    #[test]
    fn test_head_not_a_number() {
        let (r, sioe) = do_execute!(["-h", "abc"]);
        assert!(buff!(sioe, serr).contains("invalid digit"));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    #[test]
    fn test_tail_not_a_number() {
        let (r, sioe) = do_execute!(["-t", "xyz"]);
        assert!(buff!(sioe, serr).contains("invalid digit"));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    #[test]
    fn test_head_negative_number() {
        let (r, sioe) = do_execute!(["-h", "-5"]);
        assert!(buff!(sioe, serr).contains("invalid digit"));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
}

mod test_0_x_options_l {
    use libaki_unbody::*;
    use runnel::medium::stringio::*;
    use runnel::*;
    //
    #[test]
    fn test_x_option_help() {
        let (r, sioe) = do_execute!(["-X", "help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), x_help_msg!());
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_x_option_rust_version_info() {
        let (r, sioe) = do_execute!(["-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(buff!(sioe, sout).contains("rustc"));
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multiple_x_options() {
        let (r, sioe) = do_execute!(["-X", "help", "-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        // The first one should be executed and the program should exit.
        assert!(buff!(sioe, sout).contains("Options:"));
        assert!(!buff!(sioe, sout).contains("rustc"));
        assert!(r.is_ok());
    }
}

mod test_1_l {
    use libaki_unbody::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    #[test]
    fn test_non_option() {
        let (r, sioe) = do_execute!([""], "");
        #[rustfmt::skip]
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(), ": ",
                "Missing option: h or t\n",
                "Unexpected argument: \n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    #[test]
    fn test_invalid_utf8() {
        let v = std::fs::read(fixture_invalid_utf8!()).unwrap();
        let s = unsafe { String::from_utf8_unchecked(v) };
        let (r, sioe) = do_execute!(["-h", "10"], &s);
        assert_eq!(
            buff!(sioe, serr),
            concat!(program_name!(), ": stream did not contain valid UTF-8\n",)
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
}

const IN_DAT_1: &str = "\
You could not possibly have come at a better time, my dear Watson,
he said cordially.
I was afraid that you were engaged.
So I am. Very much so.
Then I can wait in the next room.
Not at all.
This gentleman, Mr. Wilson,
has been my partner and helper in many of my most successful cases,
and I have no doubt that he will be of the utmost use to me in yours also.
Try the settee, said Holmes,
relapsing into his armchair and putting his finger-tips together,
as was his custom when in judicial moods.
I know, my dear Watson,
that you share my love of all that is bizarre and outside the conventions
and humdrum routine of everyday life.
You have shown your relish for it by the enthusiasm which has prompted
you to chronicle, and, if you will excuse my saying so,
somewhat to embellish so many of my own little adventures.

\"Your cases have indeed been of the greatest interest to me,\" I observed.

You will remember that I remarked the other day,
just before we went into the very simple problem presented by Miss Mary Sutherland,
that for strange effects and extraordinary combinations we must go to life itself,
which is always far more daring than any effort of the imagination.
A proposition which I took the liberty of doubting.
";

mod test_1_more_l {
    use libaki_unbody::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    #[test]
    fn test_t1() {
        let (r, sioe) = do_execute!(["-h", "3"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "You could not possibly have come at a better time, my dear Watson,\n",
                "he said cordially.\n",
                "I was afraid that you were engaged.\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t2() {
        let (r, sioe) = do_execute!(["-t", "3"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "that for strange effects and extraordinary combinations we must go to life itself,\n",
                "which is always far more daring than any effort of the imagination.\n",
                "A proposition which I took the liberty of doubting.\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t3() {
        let (r, sioe) = do_execute!(["-h", "3", "-t", "3"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "You could not possibly have come at a better time, my dear Watson,\n",
                "he said cordially.\n",
                "I was afraid that you were engaged.\n",
                //
                "that for strange effects and extraordinary combinations we must go to life itself,\n",
                "which is always far more daring than any effort of the imagination.\n",
                "A proposition which I took the liberty of doubting.\n",
            )
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t4() {
        let (r, sioe) = do_execute!(["-h", "9", "-t", "14", "-i"], super::IN_DAT_1);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            concat!(
                "Try the settee, said Holmes,\n",
                "relapsing into his armchair and putting his finger-tips together,\n",
                "as was his custom when in judicial moods.\n",
            )
        );
        assert!(r.is_ok());
    }
}

/*
mod test_2_l {
    use libaki_unbody::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    #[test]
    fn test_multi_line() {
        let (r, sioe) = do_execute!(["-e", "a", "-f", "1"], "abcabca\noooooo\nabcabca\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "1bc1bc1\noooooo\n1bc1bc1\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multi_line_opt_n() {
        let (r, sioe) = do_execute!(["-e", "a", "-f", "1", "-n"], "abcabca\noooooo\nabcabca\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "1bc1bc1\n1bc1bc1\n");
        assert!(r.is_ok());
    }
}
*/

mod test_3_l {
    /*
    use libaki_gsub::*;
    use runnel::RunnelIoe;
    use runnel::medium::stringio::{StringIn, StringOut, StringErr};
    //
     * can NOT test
    #[test]
    fn test_output_broken_pipe() {
    }
    */
}

mod test_4_edge_cases_l {
    use libaki_unbody::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    const IN_DAT_EMPTY: &str = "";
    const IN_DAT_NO_NEWLINE: &str = "single line without newline";
    const IN_DAT_SHORT: &str = "line1\nline2\nline3";
    //
    #[test]
    fn test_head_zero() {
        let (r, sioe) = do_execute!(["-h", "0"], IN_DAT_SHORT);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_tail_zero() {
        let (r, sioe) = do_execute!(["-t", "0"], IN_DAT_SHORT);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_head_more_than_lines() {
        let (r, sioe) = do_execute!(["-h", "10"], IN_DAT_SHORT);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), IN_DAT_SHORT.to_string() + "\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_tail_more_than_lines() {
        let (r, sioe) = do_execute!(["-t", "10"], IN_DAT_SHORT);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), IN_DAT_SHORT.to_string() + "\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_empty_input() {
        let (r, sioe) = do_execute!(["-h", "10"], IN_DAT_EMPTY);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_no_newline_input_head() {
        let (r, sioe) = do_execute!(["-h", "1"], IN_DAT_NO_NEWLINE);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), IN_DAT_NO_NEWLINE.to_string() + "\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_no_newline_input_tail() {
        let (r, sioe) = do_execute!(["-t", "1"], IN_DAT_NO_NEWLINE);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), IN_DAT_NO_NEWLINE.to_string() + "\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_head_tail_overlapping() {
        let input = "line1\nline2\nline3\nline4\nline5";
        let (r, sioe) = do_execute!(["-h", "3", "-t", "3"], input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "line1\nline2\nline3\nline3\nline4\nline5\n"
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_head_tail_inverse_overlapping() {
        let input = "line1\nline2\nline3\nline4\nline5";
        let (r, sioe) = do_execute!(["-h", "3", "-t", "3", "-i"], input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_head_tail_inverse_no_overlap() {
        let input = "line1\nline2\nline3\nline4\nline5";
        let (r, sioe) = do_execute!(["-h", "1", "-t", "1", "-i"], input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "line2\nline3\nline4\n");
        assert!(r.is_ok());
    }
}

mod test_4_formats_l {
    use libaki_unbody::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    const IN_DAT_CRLF: &str = "line1\r\nline2\r\nline3";
    const IN_DAT_BLANK_LINES: &str = "line1\n\nline3\nline4\n\nline6";
    //
    #[test]
    fn test_crlf_line_endings_head() {
        let (r, sioe) = do_execute!(["-h", "2"], IN_DAT_CRLF);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "line1\nline2\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_crlf_line_endings_tail() {
        let (r, sioe) = do_execute!(["-t", "2"], IN_DAT_CRLF);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "line2\nline3\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_with_blank_lines_head() {
        let (r, sioe) = do_execute!(["-h", "3"], IN_DAT_BLANK_LINES);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "line1\n\nline3\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_with_blank_lines_tail() {
        let (r, sioe) = do_execute!(["-t", "3"], IN_DAT_BLANK_LINES);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "line4\n\nline6\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_with_blank_lines_inverse() {
        let (r, sioe) = do_execute!(["-h", "1", "-t", "1", "-i"], IN_DAT_BLANK_LINES);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "\nline3\nline4\n\n");
        assert!(r.is_ok());
    }
}

mod test_4_long_options_l {
    use libaki_unbody::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    const IN_DAT_MIXED: &str = "line1\n\nline3\nline4\n\nline6\nline7";
    //
    #[test]
    fn test_long_head_option() {
        let (r, sioe) = do_execute!(["--head", "2"], IN_DAT_MIXED);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "line1\n\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_long_tail_option() {
        let (r, sioe) = do_execute!(["--tail", "2"], IN_DAT_MIXED);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "line6\nline7\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_long_inverse_option() {
        let (r, sioe) = do_execute!(["--head", "1", "--tail", "1", "--inverse"], IN_DAT_MIXED);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "\nline3\nline4\n\nline6\n");
        assert!(r.is_ok());
    }
}

mod test_4_inverse_scenarios_l {
    use libaki_unbody::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    #[test]
    fn test_inverse_head_only() {
        let input = "1\n2\n3\n4\n5";
        let (r, sioe) = do_execute!(["-h", "2", "-i"], input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "3\n4\n5\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_inverse_tail_only() {
        let input = "1\n2\n3\n4\n5";
        let (r, sioe) = do_execute!(["-t", "2", "-i"], input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "1\n2\n3\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_inverse_head_equals_lines() {
        let input = "1\n2\n3";
        let (r, sioe) = do_execute!(["-h", "3", "-i"], input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_inverse_tail_equals_lines() {
        let input = "1\n2\n3";
        let (r, sioe) = do_execute!(["-t", "3", "-i"], input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_ok());
    }
}

mod test_4_unusual_input_l {
    use libaki_unbody::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    const IN_DAT_LONG_LINE: &str = "This is a very long line that does not contain any newlines and is meant to test how the program handles a single long string of text without any line breaks at all.";
    //
    #[test]
    fn test_long_line_no_newline_head() {
        let (r, sioe) = do_execute!(["-h", "1"], IN_DAT_LONG_LINE);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), IN_DAT_LONG_LINE.to_string() + "\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_long_line_no_newline_tail() {
        let (r, sioe) = do_execute!(["-t", "1"], IN_DAT_LONG_LINE);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), IN_DAT_LONG_LINE.to_string() + "\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_long_line_no_newline_inverse() {
        let (r, sioe) = do_execute!(["-h", "1", "-t", "1", "-i"], IN_DAT_LONG_LINE);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_ok());
    }
}

mod test_4_large_numbers_l {
    use libaki_unbody::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    #[test]
    fn test_head_u64_max() {
        let input = "line1\nline2";
        // Using a very large number, should just output the whole file.
        let (r, sioe) = do_execute!(["-h", &u64::MAX.to_string()], input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "line1\nline2\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_tail_u64_max() {
        let input = "line1\nline2";
        let (r, sioe) = do_execute!(["-t", &u64::MAX.to_string()], input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "line1\nline2\n");
        assert!(r.is_ok());
    }
}

mod test_4_complex_overlaps_l {
    use libaki_unbody::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    #[test]
    fn test_head_contains_tail() {
        let input = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10";
        let (r, sioe) = do_execute!(["-h", "8", "-t", "3"], input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "1\n2\n3\n4\n5\n6\n7\n8\n8\n9\n10\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_tail_contains_head() {
        let input = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10";
        let (r, sioe) = do_execute!(["-h", "3", "-t", "8"], input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "1\n2\n3\n3\n4\n5\n6\n7\n8\n9\n10\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_inverse_head_contains_tail() {
        let input = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10";
        let (r, sioe) = do_execute!(["-h", "8", "-t", "3", "-i"], input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_inverse_tail_contains_head() {
        let input = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10";
        let (r, sioe) = do_execute!(["-h", "3", "-t", "8", "-i"], input);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), ""); // Head is completely contained in tail, so inverse is empty
        assert!(r.is_ok());
    }
}

mod test_4_encoding_l {
    use libaki_unbody::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    const IN_DAT_UTF8: &str = "こんにちは\n世界\n\n你好\n世界\n";
    //
    #[test]
    fn test_utf8_head() {
        let (r, sioe) = do_execute!(["-h", "2"], IN_DAT_UTF8);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "こんにちは\n世界\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_utf8_tail() {
        let (r, sioe) = do_execute!(["-t", "3"], IN_DAT_UTF8);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "\n你好\n世界\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_utf8_inverse() {
        let (r, sioe) = do_execute!(["-h", "1", "-t", "2", "-i"], IN_DAT_UTF8);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "世界\n\n");
        assert!(r.is_ok());
    }
}
