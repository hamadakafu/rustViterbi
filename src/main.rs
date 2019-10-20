#![allow(non_snake_case)]

use gnuplot::{Figure, Caption, Color, Graph, AxesCommon};

mod trellis;
mod viterbi;
mod box_muller;

use viterbi::Viterbi;
use crate::viterbi::ViterbiSimu;


fn main() {
    let start_db = 1.0;
    let tick_db = 0.5;
    let end_db = 5.0;
    let bits_len = 1024;
    let iteration = 10000;

    // let mut vs = ViterbiSimu::new("hard".to_string(), start_db, tick_db, end_db, bits_len, iteration);
    let mut vs = ViterbiSimu::new("hard-dp".to_string(), start_db, tick_db, end_db, bits_len, iteration);
    vs.simu();
    vs.bit_per_error();
    dbg!(vs.oks.iter().sum::<usize>());
    dbg!(vs.ngs.iter().sum::<usize>());

    let mut fg = Figure::new();
    fg.axes2d()
      .set_title("A plot", &[])
      .set_legend(Graph(0.5), Graph(0.9), &[], &[])
      .set_x_label("sn比", &[])
      .set_y_label("log_10(BER)", &[])
      .points(
          vs.ber.iter().map(|(i, _)| i),
          vs.ber.iter().map(|(_, i)| i),
          &[Caption("Parabola")],
      );
    // dbg!(bers);
    // dbg!(vs);
    fg.show().unwrap();
}
