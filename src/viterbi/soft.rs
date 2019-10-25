use crate::trellis;

mod binary;

use binary::{Bit, NoisedSignal, Signal, StateMachine};

#[derive(Debug)]
pub struct ViterbiSoft {
    pub raw_request_data: Vec<Bit>,
    pub signal_request_data: Vec<Signal>,
    pub noised_request_data: Vec<NoisedSignal>,
    pub raw_answer_data: Vec<Bit>,
}

impl ViterbiSoft {
    pub fn new(len: usize, sigma: f64) -> Self {
        let raw_request_data: Vec<Bit> =
            (0..len).map(|i| {
                if i < len - 2 {
                    Bit((rand::random::<bool>() as usize).into())
                } else {
                    Bit(0)
                }
            }).collect();

        let mut sm: StateMachine = StateMachine::new((Bit(0), Bit(0)));

        let signal_request_data: Vec<Signal> =
            raw_request_data.iter()
                            .map(|r| sm.set(*r))
                            .collect();

        let noised_request_data =
            signal_request_data.iter()
                               .map(|s| s.add_noise(sigma))
                               .collect();

        let raw_answer_data = Vec::with_capacity(len);

        ViterbiSoft {
            raw_request_data,
            signal_request_data,
            noised_request_data,
            raw_answer_data,
        }
    }

    pub fn decode(&mut self) {
        let len = self.raw_request_data.len();
        let mut memo: Vec<Vec<Option<(Option<(StateMachine, Bit)>, f64)>>> = vec![vec![None; len + 1]; 4];
        memo[0][0] = Some((None, 0.));
        for i in 0..len {
            for j in 0..4 {
                if let Some(cell) = memo[j][i].clone() {
                    let now_bits = binary::into_2bits(j);
                    { // 0をセットしたときの処理
                        let mut sm0 = StateMachine::from(now_bits);
                        let signal0 = binary::bpsk(sm0.set(Bit(0)));
                        let euc_dis0 = cell.1
                            + (signal0.0 as f64 - self.noised_request_data[i].0).powi(2)
                            + (signal0.1 as f64 - self.noised_request_data[i].1).powi(2);
                        if let Some(next_cell) = memo[Into::<usize>::into(sm0)][i + 1].clone() {
                            if next_cell.1 > euc_dis0 {
                                memo[Into::<usize>::into(sm0)][i + 1] =
                                    Some((Some((StateMachine::from(now_bits), Bit(0))), euc_dis0));
                            }
                        } else {
                            memo[Into::<usize>::into(sm0)][i + 1] =
                                Some((Some((StateMachine::from(now_bits), Bit(0))), euc_dis0));
                        }
                    }
                    { // 1をセットしたときの処理
                        let mut sm1 = StateMachine::from(now_bits);
                        let signal1 = binary::bpsk(sm1.set(Bit(1)));
                        let euc_dis1 = cell.1
                            + (signal1.0 as f64 - self.noised_request_data[i].0).powi(2)
                            + (signal1.1 as f64 - self.noised_request_data[i].1).powi(2);
                        if let Some(next_cell) = memo[Into::<usize>::into(sm1)][i + 1].clone() {
                            if next_cell.1 > euc_dis1 {
                                memo[Into::<usize>::into(sm1)][i + 1] =
                                    Some((Some((StateMachine::from(now_bits), Bit(1))), euc_dis1));
                            }
                        } else {
                            memo[Into::<usize>::into(sm1)][i + 1] =
                                Some((Some((StateMachine::from(now_bits), Bit(1))), euc_dis1));
                        }
                    }
                }
            }
        }

        let bit = memo[0][len].unwrap().0.unwrap().1;
        let mut tmp_answer = vec![bit];
        let mut parent = memo[0][len].unwrap().0.unwrap().0;
        for i in 0..len - 1 {
            let pre = memo[Into::<usize>::into(parent)][len - i - 1].unwrap();
            tmp_answer.push(pre.0.unwrap().1);
            parent = pre.0.unwrap().0;
        }
        tmp_answer.reverse();
        self.raw_answer_data = tmp_answer;
    }
}
