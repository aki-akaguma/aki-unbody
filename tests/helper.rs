#[allow(unused_macros)]
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

#[allow(unused_macros)]
macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

#[allow(unused_macros)]
macro_rules! program_name {
    () => {
        "aki-unbody"
    };
}

#[allow(unused_macros)]
macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}

#[allow(unused_macros)]
macro_rules! fixture_invalid_utf8 {
    () => {
        "fixtures/invalid_utf8.txt"
    };
}
