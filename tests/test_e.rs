const TARGET_EXE_PATH: &str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

#[macro_use]
mod helper;

#[macro_use]
mod helper_e;

mod test_0_e {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-H"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_help_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version() {
        let oup = exec_target(TARGET_EXE_PATH, ["-V"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--version"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_invalid_opt() {
        let oup = exec_target(TARGET_EXE_PATH, ["-z"]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Invalid option: z\n",
                "Missing option: h or t\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    #[test]
    fn test_head_not_a_number() {
        let oup = exec_target(TARGET_EXE_PATH, ["-h", "abc"]);
        assert!(oup.stderr.contains("invalid digit"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    #[test]
    fn test_tail_not_a_number() {
        let oup = exec_target(TARGET_EXE_PATH, ["-t", "xyz"]);
        assert!(oup.stderr.contains("invalid digit"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    #[test]
    fn test_head_negative_number() {
        let oup = exec_target(TARGET_EXE_PATH, ["-h", "-5"]);
        assert!(oup.stderr.contains("invalid digit"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
}

mod test_0_x_options_e {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_x_option_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, x_help_msg!());
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_x_option_rust_version_info() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        assert!(oup.stdout.contains("rustc"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_multiple_x_options() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help", "-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        // The first one should be executed and the program should exit.
        assert!(oup.stdout.contains("Options:"));
        assert!(!oup.stdout.contains("rustc"));
        assert!(oup.status.success());
    }
}

mod test_1_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_non_option() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, [""], b"");
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Missing option: h or t\n",
                "Unexpected argument: \n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_invalid_utf8() {
        let v = std::fs::read(fixture_invalid_utf8!()).unwrap();
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-h", "10"], &v);
        assert_eq!(
            oup.stderr,
            concat!(program_name!(), ": stream did not contain valid UTF-8\n",)
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
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

mod test_1_more_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-h", "3"], super::IN_DAT_1.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "You could not possibly have come at a better time, my dear Watson,\n",
                "he said cordially.\n",
                "I was afraid that you were engaged.\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t2() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-t", "3"], super::IN_DAT_1.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "that for strange effects and extraordinary combinations we must go to life itself,\n",
                "which is always far more daring than any effort of the imagination.\n",
                "A proposition which I took the liberty of doubting.\n",
            )
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t3() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-h", "3", "-t", "3"],
            super::IN_DAT_1.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t4() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-h", "9", "-t", "14", "-i"],
            super::IN_DAT_1.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Try the settee, said Holmes,\n",
                "relapsing into his armchair and putting his finger-tips together,\n",
                "as was his custom when in judicial moods.\n",
            )
        );
        assert!(oup.status.success());
    }
}

/*
mod test_2_e {
    use exec_target::exec_target_with_in;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_multi_line() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "a", "-f", "1"],
            b"abcabca\noooooo\nabcabca\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "1bc1bc1\noooooo\n1bc1bc1\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_multi_line_opt_n() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "a", "-f", "1", "-n"],
            b"abcabca\noooooo\nabcabca\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "1bc1bc1\n1bc1bc1\n");
        assert!(oup.status.success());
    }
} // mod test_2
*/

mod test_3_e {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_output_broken_pipe() {
        let cmdstr = format!(
            "cat \"{}\" | \"{}\" -h 10 | head -n 2",
            "fixtures/sherlock.txt", TARGET_EXE_PATH
        );
        let oup = exec_target("sh", ["-c", &cmdstr]);
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "You could not possibly have come at a better time, my dear Watson,\n",
                "he said cordially.\n"
            )
        );
        assert!(oup.status.success());
    }
}

mod test_4_edge_cases_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    const IN_DAT_EMPTY: &str = "";
    const IN_DAT_NO_NEWLINE: &str = "single line without newline";
    const IN_DAT_SHORT: &str = "line1\nline2\nline3";
    //
    #[test]
    fn test_head_zero() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-h", "0"], IN_DAT_SHORT.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_tail_zero() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-t", "0"], IN_DAT_SHORT.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_head_more_than_lines() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-h", "10"], IN_DAT_SHORT.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, IN_DAT_SHORT.to_string() + "\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_tail_more_than_lines() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-t", "10"], IN_DAT_SHORT.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, IN_DAT_SHORT.to_string() + "\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_empty_input() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-h", "10"], IN_DAT_EMPTY.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_no_newline_input_head() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-h", "1"], IN_DAT_NO_NEWLINE.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, IN_DAT_NO_NEWLINE.to_string() + "\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_no_newline_input_tail() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-t", "1"], IN_DAT_NO_NEWLINE.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, IN_DAT_NO_NEWLINE.to_string() + "\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_head_tail_overlapping() {
        let input = "line1\nline2\nline3\nline4\nline5";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-h", "3", "-t", "3"], input.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "line1\nline2\nline3\nline3\nline4\nline5\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_head_tail_inverse_overlapping() {
        let input = "line1\nline2\nline3\nline4\nline5";
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-h", "3", "-t", "3", "-i"],
            input.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_head_tail_inverse_no_overlap() {
        let input = "line1\nline2\nline3\nline4\nline5";
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-h", "1", "-t", "1", "-i"],
            input.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "line2\nline3\nline4\n");
        assert!(oup.status.success());
    }
}

mod test_4_formats_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    const IN_DAT_CRLF: &str = "line1\r\nline2\r\nline3";
    const IN_DAT_BLANK_LINES: &str = "line1\n\nline3\nline4\n\nline6";
    //
    #[test]
    fn test_crlf_line_endings_head() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-h", "2"], IN_DAT_CRLF.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "line1\nline2\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_crlf_line_endings_tail() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-t", "2"], IN_DAT_CRLF.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "line2\nline3\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_with_blank_lines_head() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-h", "3"], IN_DAT_BLANK_LINES.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "line1\n\nline3\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_with_blank_lines_tail() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-t", "3"], IN_DAT_BLANK_LINES.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "line4\n\nline6\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_with_blank_lines_inverse() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-h", "1", "-t", "1", "-i"],
            IN_DAT_BLANK_LINES.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "\nline3\nline4\n\n");
        assert!(oup.status.success());
    }
}

mod test_4_long_options_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    const IN_DAT_MIXED: &str = "line1\n\nline3\nline4\n\nline6\nline7";
    //
    #[test]
    fn test_long_head_option() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["--head", "2"], IN_DAT_MIXED.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "line1\n\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_long_tail_option() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["--tail", "2"], IN_DAT_MIXED.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "line6\nline7\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_long_inverse_option() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["--head", "1", "--tail", "1", "--inverse"],
            IN_DAT_MIXED.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "\nline3\nline4\n\nline6\n");
        assert!(oup.status.success());
    }
}

mod test_4_inverse_scenarios_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_inverse_head_only() {
        let input = "1\n2\n3\n4\n5";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-h", "2", "-i"], input.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "3\n4\n5\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_inverse_tail_only() {
        let input = "1\n2\n3\n4\n5";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-t", "2", "-i"], input.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "1\n2\n3\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_inverse_head_equals_lines() {
        let input = "1\n2\n3";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-h", "3", "-i"], input.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_inverse_tail_equals_lines() {
        let input = "1\n2\n3";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-t", "3", "-i"], input.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
}

mod test_4_unusual_input_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    const IN_DAT_LONG_LINE: &str = "This is a very long line that does not contain any newlines and is meant to test how the program handles a single long string of text without any line breaks at all.";
    //
    #[test]
    fn test_long_line_no_newline_head() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-h", "1"], IN_DAT_LONG_LINE.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, IN_DAT_LONG_LINE.to_string() + "\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_long_line_no_newline_tail() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-t", "1"], IN_DAT_LONG_LINE.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, IN_DAT_LONG_LINE.to_string() + "\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_long_line_no_newline_inverse() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-h", "1", "-t", "1", "-i"],
            IN_DAT_LONG_LINE.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
}

mod test_4_large_numbers_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_head_u64_max() {
        let input = "line1\nline2";
        // Using a very large number, should just output the whole file.
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-h", &u64::MAX.to_string()],
            input.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "line1\nline2\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_tail_u64_max() {
        let input = "line1\nline2";
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-t", &u64::MAX.to_string()],
            input.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "line1\nline2\n");
        assert!(oup.status.success());
    }
}

mod test_4_complex_overlaps_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_head_contains_tail() {
        let input = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-h", "8", "-t", "3"], input.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "1\n2\n3\n4\n5\n6\n7\n8\n8\n9\n10\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_tail_contains_head() {
        let input = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10";
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-h", "3", "-t", "8"], input.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "1\n2\n3\n3\n4\n5\n6\n7\n8\n9\n10\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_inverse_head_contains_tail() {
        let input = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10";
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-h", "8", "-t", "3", "-i"],
            input.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_inverse_tail_contains_head() {
        let input = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10";
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-h", "3", "-t", "8", "-i"],
            input.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, ""); // Head is completely contained in tail, so inverse is empty
        assert!(oup.status.success());
    }
}

mod test_4_encoding_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    const IN_DAT_UTF8: &str = "こんにちは\n世界\n\n你好\n世界\n";
    //
    #[test]
    fn test_utf8_head() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-h", "2"], IN_DAT_UTF8.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "こんにちは\n世界\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_utf8_tail() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-t", "3"], IN_DAT_UTF8.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "\n你好\n世界\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_utf8_inverse() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-h", "1", "-t", "2", "-i"],
            IN_DAT_UTF8.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "世界\n\n");
        assert!(oup.status.success());
    }
}
