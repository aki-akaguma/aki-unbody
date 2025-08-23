macro_rules! help_msg {
    () => {
        concat!(
            version_msg!(),
            "\n",
            indoc::indoc!(
                r#"
            Usage:
              aki-unbody [options]

            output first or last n lines, like a head and tail of linux command.

            Options:
              -h, --head <num>      output the first <num> lines.
              -t, --tail <num>      output the last <num> lines.
              -i, --inverse         output the body, except for head and tail.

              -H, --help        display this help and exit
              -V, --version     display version information and exit
              -X <x-options>    x options. try -X help

            Examples:
              Outputs first 2 lines:
                cat file1.txt | aki-unbody --head 2
              Outputs last 2 lines:
                cat file1.txt | aki-unbody --tail 2
              Outputs body, except for first 2 lines and last 2 lines:
                cat file1.txt | aki-unbody --head 2 --tail 2 --inverse
            "#
            ),
            "\n",
        )
    };
}

macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

macro_rules! program_name {
    () => {
        "aki-unbody"
    };
}

macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}

macro_rules! do_execute {
    ($args:expr) => {
        do_execute!($args, "")
    };
    ($args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            #[allow(clippy::box_default)]
            Box::new(StringOut::default()),
            #[allow(clippy::box_default)]
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute(&sioe, &program, $args);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe
                    .pg_err()
                    .lock()
                    .write_fmt(format_args!("{}: {}\n", program, err));
            }
        };
        (r, sioe)
    }};
}

macro_rules! buff {
    ($sioe:expr, serr) => {
        $sioe.pg_err().lock().buffer_to_string()
    };
    ($sioe:expr, sout) => {
        $sioe.pg_out().lock().buffer_to_string()
    };
}

mod test_s0 {
    use libaki_unbody::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_help() {
        let (r, sioe) = do_execute!(&["-H"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(&["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(&["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(&["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_non_option() {
        let (r, sioe) = do_execute!(&[""]);
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

mod test_s1 {
    use libaki_unbody::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t1() {
        let (r, sioe) = do_execute!(&["-h", "3"], super::IN_DAT_1);
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
        let (r, sioe) = do_execute!(&["-t", "3"], super::IN_DAT_1);
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
        let (r, sioe) = do_execute!(&["-h", "3", "-t", "3"], super::IN_DAT_1);
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
        let (r, sioe) = do_execute!(&["-h", "9", "-t", "14", "-i"], super::IN_DAT_1);
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
mod test_s2 {
    use libaki_unbody::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_multi_line() {
        let (r, sioe) = do_execute!(&["-e", "a", "-f", "1"], "abcabca\noooooo\nabcabca\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "1bc1bc1\noooooo\n1bc1bc1\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multi_line_opt_n() {
        let (r, sioe) = do_execute!(&["-e", "a", "-f", "1", "-n"], "abcabca\noooooo\nabcabca\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "1bc1bc1\n1bc1bc1\n");
        assert!(r.is_ok());
    }
}
*/

mod test_s3 {
    /*
    use libaki_gsub::*;
    use runnel::RunnelIoe;
    use runnel::medium::stringio::{StringIn, StringOut, StringErr};
    use std::io::Write;
    //
     * can NOT test
    #[test]
    fn test_output_broken_pipe() {
    }
    */
}
