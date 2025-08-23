use crate::conf::CmdOptConf;
use crate::util::err::BrokenPipeError;
use runnel::RunnelIoe;

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
    let mut tail_buffer = Vec::with_capacity(max_tail);
    //
    // input
    for (curr_line_count, line) in sioe.pg_in().lines().enumerate() {
        let line_s = line?;
        //
        if conf.opt_head.is_some() && curr_line_count < max_head {
            sioe.pg_out().write_line(line_s.clone())?;
        }
        tail_buffer.push(line_s);
        if tail_buffer.len() > max_tail {
            let _ = tail_buffer.remove(0);
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
    let mut body_buffer = Vec::new();
    //
    // input
    for (curr_line_count, line) in sioe.pg_in().lines().enumerate() {
        let line_s = line?;
        //
        if conf.opt_head.is_some() && curr_line_count < max_head {
            // nothing todo
        } else {
            body_buffer.push(line_s);
        }
    }
    //
    // output
    // cut tail lines
    if max_tail > 0 {
        let buffer_len = body_buffer.len();
        let len = buffer_len.min(max_tail);
        let _ = body_buffer.split_off(buffer_len - len);
    }
    for line_s in body_buffer {
        sioe.pg_out().write_line(line_s)?;
    }
    //
    sioe.pg_out().flush_line()?;
    //
    Ok(())
}
