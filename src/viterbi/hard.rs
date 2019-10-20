use self::super::Viterbi;
use crate::trellis;

#[derive(Debug)]
pub struct ViterbiHard {
    pub raw_request_data: Vec<trellis::Bit>,
    pub signal_request_data: Vec<trellis::Signal>,
    pub noised_request_data: Vec<trellis::Signal>,
    pub raw_answer_data: Vec<trellis::Bit>,
    pub state_machine: trellis::StateMachine,
}

impl Viterbi for ViterbiHard {
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

        let state_machine = trellis::StateMachine::new(trellis::SMState::OO);
        let raw_answer_data = Vec::with_capacity(len);
        let viterbi = ViterbiHard {
            raw_request_data,
            signal_request_data,
            noised_request_data,
            raw_answer_data,
            state_machine,
        };
        // dbg!(&viterbi);
        return viterbi;
    }

    fn get_raw_request_data(&self) -> &Vec<trellis::Bit> { return &self.raw_request_data; }
    fn get_raw_answer_data(&self) -> &Vec<trellis::Bit> { return &self.raw_answer_data; }

    fn decode(&mut self, trellis: &trellis::Trellis) {
        for i in 0..self.raw_request_data.len() - 2 {
            let bit = trellis.get_next_bit(
                self.state_machine.state,
                (self.noised_request_data[i], self.noised_request_data[i + 1], self.noised_request_data[i + 2]),
            );
            dbg!(i);
            dbg!(&self.state_machine);
            dbg!(&self.noised_request_data[i]);
            dbg!(&self.noised_request_data[i + 1]);
            dbg!(&self.noised_request_data[i + 2]);
            dbg!(bit);
            self.state_machine.set(bit);
            self.raw_answer_data.push(bit);
            dbg!(&self);
        }
        self.raw_answer_data.push(trellis::Bit::O);
        self.raw_answer_data.push(trellis::Bit::O);
    }
}
