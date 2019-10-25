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
    let mut vs = ViterbiSimu::new("soft".to_string(), start_db, tick_db, end_db, bits_len, iteration);
    vs.simu();
    vs.bit_per_error();
    dbg!(vs.oks);
    dbg!(vs.ngs);

    let mut fg = Figure::new();
    fg.axes2d()
      .set_title("Viterbi", &[])
      .set_legend(Graph(0.5), Graph(0.9), &[], &[])
      .set_x_label("SN", &[])
      .set_y_label("log10(BER)", &[])
      .points(
          vs.ber.iter().map(|(i, _)| i),
          vs.ber.iter().map(|(_, i)| i),
          &[Caption("Parabola")],
      );
    // dbg!(bers);
    // dbg!(vs);
    fg.show().unwrap();
}
