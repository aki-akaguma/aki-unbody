use crate::conf::CmdOptConf;
use crate::util::err::BrokenPipeError;
use runnel::RunnelIoe;
use std::io::{BufRead, Write};

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    let r = if conf.flg_inverse {
        run_inverse(sioe, conf)
    } else if conf.opt_head.is_some() && conf.opt_tail.is_none() {
        run_only_head(sioe, conf)
    } else {
        run_0(sioe, conf)
    };
    if r.is_broken_pipe() {
        return Ok(());
    }
    r
}

fn run_only_head(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    let max_head = conf.opt_head.unwrap();
    //
    for (curr_line_count, line) in sioe.pin().lock().lines().enumerate() {
        if curr_line_count >= max_head {
            break;
        }
        let line_s = line?;
        let line_ss = line_s.as_str();
        //let line_len: usize = line_ss.len();
        //
        #[rustfmt::skip]
        sioe.pout().lock().write_fmt(format_args!("{}\n", line_ss))?;
    }
    //
    sioe.pout().lock().flush()?;
    //
    Ok(())
}

fn run_0(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    let mut buffer = Vec::new();
    let max_head = conf.opt_head.unwrap_or(0);
    //
    // input
    for (curr_line_count, line) in sioe.pin().lock().lines().enumerate() {
        let line_s = line?;
        let line_ss = line_s.as_str();
        //let line_len: usize = line_ss.len();
        //
        if conf.opt_head.is_some() && curr_line_count < max_head {
            #[rustfmt::skip]
            sioe.pout().lock().write_fmt(format_args!("{}\n", line_ss))?;
        }
        //
        buffer.push(line_s);
    }
    // output
    let tail_n = conf.opt_tail.unwrap_or(0);
    if tail_n > 0 {
        let buffer_len = buffer.len();
        let len = buffer_len.min(tail_n);
        let lines = &buffer[(buffer_len - len)..buffer_len];
        for line in lines {
            let line_ss = line.as_str();
            #[rustfmt::skip]
            sioe.pout().lock().write_fmt(format_args!("{}\n", line_ss))?;
        }
    }
    //
    sioe.pout().lock().flush()?;
    //
    Ok(())
}

fn run_inverse(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    let mut buffer = Vec::new();
    let max_head = conf.opt_head.unwrap_or(0);
    //
    // input
    for (curr_line_count, line) in sioe.pin().lock().lines().enumerate() {
        let line_s = line?;
        //let line_ss = line_s.as_str();
        //let line_len: usize = line_ss.len();
        //
        if conf.opt_head.is_some() && curr_line_count < max_head {
            // nothing todo
        } else {
            buffer.push(line_s);
        }
    }
    // output
    let tail_n = conf.opt_tail.unwrap_or(0);
    {
        let buffer_len = buffer.len();
        let len = buffer_len.min(tail_n);
        let lines = &buffer[0..(buffer_len - len)];
        for line in lines {
            let line_ss = line.as_str();
            #[rustfmt::skip]
            sioe.pout().lock().write_fmt(format_args!("{}\n", line_ss))?;
        }
    }
    //
    sioe.pout().lock().flush()?;
    //
    Ok(())
}
