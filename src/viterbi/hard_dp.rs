use crate::trellis;
use crate::viterbi::Viterbi;
use crate::trellis::{SMState, StateMachine, Bit};

#[derive(Debug)]
pub struct ViterbiHardDP {
    pub raw_request_data: Vec<trellis::Bit>,
    pub signal_request_data: Vec<trellis::Signal>,
    pub noised_request_data: Vec<trellis::Signal>,
    pub raw_answer_data: Vec<trellis::Bit>,
}

impl Viterbi for ViterbiHardDP {
    fn new(len: usize, sigma: f64) -> Self {
        let raw_request_data: Vec<trellis::Bit> =
            (0..len).map(|i| {
                if i < len - 2 {
                    (rand::random::<bool>() as usize).into()
                } else {
                    trellis::Bit::O
                }
            }).collect();
        let mut sm: trellis::StateMachine = trellis::StateMachine::new(trellis::SMState::OO);
        let signal_request_data: Vec<trellis::Signal> = raw_request_data.iter().map(|r| {
            sm.set(*r)
        }).collect();
        let noised_request_data = signal_request_data.iter().map(|s| {
            s.add_noise(sigma)
        }).collect();
        let raw_answer_data = Vec::with_capacity(len);
        ViterbiHardDP {
            raw_request_data,
            signal_request_data,
            noised_request_data,
            raw_answer_data,
        }
    }

    fn get_raw_request_data(&self) -> &Vec<trellis::Bit> { return &self.raw_request_data; }
    fn get_raw_answer_data(&self) -> &Vec<trellis::Bit> { return &self.raw_answer_data; }

    fn decode(&mut self, trellis: &trellis::Trellis) {
        // dp
        // (Option<(parents, target bit), min dis)
        let mut memo: Vec<Vec<Option<(Option<(trellis::SMState, Bit)>, usize)>>> = vec![vec![None; self.noised_request_data.len() + 1]; 4];
        memo[0][0] = Some((None, 0));
        for i in 0..self.noised_request_data.len() {
            for j in 0..4 {
                if let Some(value) = memo[j][i] {
                    let old_state = Into::<SMState>::into(j);

                    let mut sm = StateMachine::new(old_state);
                    let signal = sm.set(Bit::O);
                    let new_state = sm.state;
                    let new_dis = self.noised_request_data[i] - signal + value.1;

                    match memo[Into::<usize>::into(new_state)][i + 1] {
                        Some(already_value) if (already_value.1 < new_dis) => {}
                        Some(already_value) if already_value.1 == new_dis => {
                            // random
                            if rand::random::<bool>() {
                                memo[Into::<usize>::into(new_state)][i + 1] = Some((Some((old_state, Bit::O)), new_dis));
                            }
                        }
                        _ => {
                            memo[Into::<usize>::into(new_state)][i + 1] = Some((Some((old_state, Bit::O)), new_dis));
                        }
                    }

                    let mut sm = StateMachine::new(old_state);
                    let signal = sm.set(Bit::I);
                    let new_state = sm.state;
                    let new_dis = self.noised_request_data[i] - signal + value.1;

                    match memo[Into::<usize>::into(new_state)][i + 1] {
                        Some(already_value) if (already_value.1 < new_dis) => {}
                        Some(already_value) if already_value.1 == new_dis => {
                            // random
                            if rand::random::<bool>() {
                                memo[Into::<usize>::into(new_state)][i + 1] = Some((Some((old_state, Bit::I)), new_dis));
                            }
                        }
                        _ => {
                            memo[Into::<usize>::into(new_state)][i + 1] = Some((Some((old_state, Bit::I)), new_dis));
                        }
                    }
                }
            }
        }
        let bits_len = self.raw_request_data.len();
        let bit = memo[0][bits_len].unwrap().0.unwrap().1;
        let mut tmp_answer = vec![bit];
        let mut parent = memo[0][bits_len].unwrap().0.unwrap().0;
        for i in 0..bits_len-1 {
            let pre = memo[Into::<usize>::into(parent)][bits_len - i - 1].unwrap();
            tmp_answer.push(pre.0.unwrap().1);
            parent = pre.0.unwrap().0;
        }
        tmp_answer.reverse();
        self.raw_answer_data = tmp_answer;
    }
}
