use crate::trellis;

mod hard;
mod hard_dp;
mod soft;

pub trait Viterbi {
    fn new(len: usize, sigma: f64) -> Self;
    fn get_raw_request_data(&self) -> &Vec<trellis::Bit>;
    fn get_raw_answer_data(&self) -> &Vec<trellis::Bit>;
    fn decode(&mut self, trellis: &trellis::Trellis);
}


#[derive(Debug)]
pub struct ViterbiSimu {
    pub way: String,
    pub len: usize,
    pub start_db: f64,
    pub tick_db: f64,
    pub end_db: f64,
    pub bits_len: usize,
    pub iteration: usize,
    pub ber: Vec<(f64, f64)>,
    pub oks: Vec<usize>,
    pub ngs: Vec<usize>,
}

impl ViterbiSimu {
    pub fn new(way: String,
               start_db: f64,
               tick_db: f64,
               end_db: f64,
               bits_len: usize,
               iteration: usize,
    ) -> ViterbiSimu {
        let len = ((end_db - start_db) / tick_db) as usize + 1;
        let ber = (0..len)
            .map(|a| start_db + a as f64 * tick_db)
            .map(|a| (a, 0.)).collect();
        return ViterbiSimu {
            way,
            len,
            start_db,
            tick_db,
            end_db,
            bits_len,
            iteration,
            ber,
            oks: vec![0; len],
            ngs: vec![0; len],
        };
    }

    pub fn simu(&mut self) {
        let trellis = trellis::Trellis::new();
        let lines: Vec<f64> = (0..self.len as usize)
            .map(|a| self.start_db + a as f64 * self.tick_db).collect();
        for (i, sn) in lines.iter().enumerate() {
            for _ in 0..self.iteration {
                let sigma = 1.0 / ((10.0 as f64).powf(sn / 10.0) * 2.0).sqrt();
                if &self.way == "hard" {
                    let mut viterbi = hard::ViterbiHard::new(self.bits_len, sigma);
                    viterbi.decode(&trellis);
                    for (r, a) in viterbi.get_raw_request_data().iter().zip(viterbi.get_raw_answer_data()) {
                        if r == a {
                            self.oks[i] += 1;
                        } else {
                            self.ngs[i] += 1;
                        }
                    }
                } else if &self.way == "hard-dp" {
                    let mut viterbi = hard_dp::ViterbiHardDP::new(self.bits_len, sigma);
                    viterbi.decode(&trellis);
                    for (r, a) in viterbi.get_raw_request_data().iter().zip(viterbi.get_raw_answer_data()) {
                        if r == a {
                            self.oks[i] += 1;
                        } else {
                            self.ngs[i] += 1;
                        }
                    }
                } else if &self.way == "soft" {
                    let mut viterbi = soft::ViterbiSoft::new(self.bits_len, sigma);
                    viterbi.decode();
                    for (r, a) in viterbi.raw_request_data.iter().zip(&viterbi.raw_answer_data) {
                        if r == a {
                            self.oks[i] += 1;
                        } else {
                            self.ngs[i] += 1;
                        }
                    }
                } else {
                    panic!("i don't know");
                }
            }
        }
    }

    pub fn bit_per_error(&mut self) {
        for i in 0..self.len {
            // self.ber[i].1 = self.ngs[i] as f64 / (self.oks[i] + self.ngs[i]) as f64;
            self.ber[i].1 = (self.ngs[i] as f64 / (self.oks[i] + self.ngs[i]) as f64).log10();
        }
    }
}