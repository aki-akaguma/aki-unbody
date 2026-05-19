use crate::conf::CmdOptConf;
use crate::util::err::BrokenPipeError;
use runnel::RunnelIoe;
use std::collections::VecDeque;

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    let r = if conf.flg_inverse {
        run_inverse(sioe, conf)
    } else {
        run_normal(sioe, conf)
    };
    if r.is_broken_pipe() {
        return Ok(());
    }
    r
}

fn run_normal(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    let max_head = conf.opt_head.unwrap_or(0);
    let max_tail = conf.opt_tail.unwrap_or(0);
    let mut tail_buffer = VecDeque::with_capacity(max_tail.min(4 * 1024));
    //
    // input
    for (curr_line_count, line) in sioe.pg_in().lines().enumerate() {
        let line_s = line?;
        //
        if conf.opt_head.is_some() && curr_line_count < max_head {
            sioe.pg_out().write_line(line_s.clone())?;
        }
        if max_tail > 0 {
            tail_buffer.push_back(line_s);
            if tail_buffer.len() > max_tail {
                let _ = tail_buffer.pop_front();
            }
        }
    }
    //
    // output
    for line_s in tail_buffer {
        sioe.pg_out().write_line(line_s)?;
    }
    //
    sioe.pg_out().flush_line()?;
    //
    Ok(())
}

fn run_inverse(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    let max_head = conf.opt_head.unwrap_or(0);
    let max_tail = conf.opt_tail.unwrap_or(0);
    let mut tail_window = VecDeque::with_capacity(max_tail.min(4 * 1024));
    //
    // input
    for (curr_line_count, line) in sioe.pg_in().lines().enumerate() {
        let line_s = line?;
        //
        if conf.opt_head.is_some() && curr_line_count < max_head {
            // nothing todo
        } else if max_tail > 0 {
            tail_window.push_back(line_s);
            if tail_window.len() > max_tail {
                if let Some(out_line) = tail_window.pop_front() {
                    sioe.pg_out().write_line(out_line)?;
                }
            }
        } else {
            sioe.pg_out().write_line(line_s)?;
        }
    }
    //
    sioe.pg_out().flush_line()?;
    //
    Ok(())
}
