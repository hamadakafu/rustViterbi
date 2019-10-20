use crate::box_muller::box_muller;
use std::ops::{Add, Sub};
use std::usize;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SMState {
    OO,
    OI,
    IO,
    II,
}

impl Into<SMState> for usize {
    fn into(self) -> SMState {
        match self {
            0 => SMState::OO,
            1 => SMState::OI,
            2 => SMState::IO,
            3 => SMState::II,
            _ => panic!("cant convert from int to SMState"),
        }
    }
}

impl Into<usize> for SMState {
    fn into(self) -> usize {
        match self {
            SMState::OO => 0,
            SMState::OI => 1,
            SMState::IO => 2,
            SMState::II => 3,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Signal {
    OO,
    OI,
    IO,
    II,
}

impl Signal {
    pub fn add_noise(&self, sigma: f64) -> Signal {
        let first;
        let second;
        match *self {
            Signal::OO => {
                first = -1.0 + sigma * box_muller();
                second = -1.0 + sigma * box_muller();
            }
            Signal::OI => {
                first = -1.0 + sigma * box_muller();
                second = 1.0 + sigma * box_muller();
            }
            Signal::IO => {
                first = 1.0 + sigma * box_muller();
                second = -1.0 + sigma * box_muller();
            }
            Signal::II => {
                first = 1.0 + sigma * box_muller();
                second = 1.0 + sigma * box_muller();
            }
        }
        // dbg!(first, second);
        match (first > 0.0, second > 0.0) {
            (false, false) => Signal::OO,
            (false, true) => Signal::OI,
            (true, false) => Signal::IO,
            (true, true) => Signal::II,
        }
    }
}

impl Into<Signal> for usize {
    fn into(self) -> Signal {
        match self {
            0 => Signal::OO,
            1 => Signal::OI,
            2 => Signal::IO,
            3 => Signal::II,
            _ => panic!("cant convert from int to SMState"),
        }
    }
}

impl Into<usize> for Signal {
    fn into(self) -> usize {
        match self {
            Signal::OO => 0,
            Signal::OI => 1,
            Signal::IO => 2,
            Signal::II => 3,
        }
    }
}

impl Sub for Signal {
    type Output = usize;
    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Signal::OO, Signal::OO) => 0,
            (Signal::OO, Signal::OI) => 1,
            (Signal::OO, Signal::IO) => 1,
            (Signal::OO, Signal::II) => 2,

            (Signal::OI, Signal::OO) => 1,
            (Signal::OI, Signal::OI) => 0,
            (Signal::OI, Signal::IO) => 2,
            (Signal::OI, Signal::II) => 1,

            (Signal::IO, Signal::OO) => 1,
            (Signal::IO, Signal::OI) => 2,
            (Signal::IO, Signal::IO) => 0,
            (Signal::IO, Signal::II) => 1,

            (Signal::II, Signal::OO) => 2,
            (Signal::II, Signal::OI) => 1,
            (Signal::II, Signal::IO) => 1,
            (Signal::II, Signal::II) => 0,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Bit {
    O,
    I,
}

impl Into<Bit> for usize {
    fn into(self) -> Bit {
        if self == 0 {
            Bit::O
        } else if self == 1 {
            Bit::I
        } else {
            panic!("cant convert from into to Bit")
        }
    }
}

impl Into<usize> for Bit {
    fn into(self) -> usize {
        match self {
            Bit::O => 0,
            Bit::I => 1,
        }
    }
}

impl Add for Bit {
    type Output = usize;
    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Bit::O, Bit::O) => 0,
            (Bit::O, Bit::I) | (Bit::I, Bit::O) => 1,
            (Bit::I, Bit::I) => 2,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StateMachine {
    pub state: SMState,
}

impl StateMachine {
    pub fn new(sms: SMState) -> Self {
        StateMachine { state: sms }
    }
    pub fn set(&mut self, bit: Bit) -> Signal {
        match (bit, self.state) {
            (Bit::O, SMState::OO) => {
                self.state = SMState::OO;
                Signal::OO
            }
            (Bit::I, SMState::OO) => {
                self.state = SMState::IO;
                Signal::II
            }
            (Bit::O, SMState::OI) => {
                self.state = SMState::OO;
                Signal::II
            }
            (Bit::I, SMState::OI) => {
                self.state = SMState::IO;
                Signal::OO
            }
            (Bit::O, SMState::IO) => {
                self.state = SMState::OI;
                Signal::IO
            }
            (Bit::I, SMState::IO) => {
                self.state = SMState::II;
                Signal::OI
            }
            (Bit::O, SMState::II) => {
                self.state = SMState::OI;
                Signal::OI
            }
            (Bit::I, SMState::II) => {
                self.state = SMState::II;
                Signal::IO
            }
        }
    }
    // // 今の状態から1つ前の状態とその際の受信信号が2つ返す
    // pub fn rollback(&mut self, bit: Bit) -> Option<((SMState, Signal), (SMState, Signal))> {
    //     match (bit, self.state) {
    //         (Bit::O, SMState::OO) => {
    //             Some(((SMState::OO, Signal::OO), (SMState::OI, Signal::II)))
    //         }
    //         (Bit::I, SMState::OO) => {
    //             None
    //         }
    //         (Bit::O, SMState::OI) => {
    //             Some(((SMState::IO, Signal::IO), (SMState::II, Signal::OI)))
    //         }
    //         (Bit::I, SMState::OI) => {
    //             None
    //         }
    //         (Bit::O, SMState::IO) => {
    //             None
    //         }
    //         (Bit::I, SMState::IO) => {
    //             Some(((SMState::OO, Signal::II), (SMState::OI, Signal::OO)))
    //         }
    //         (Bit::O, SMState::II) => {
    //             None
    //         }
    //         (Bit::I, SMState::II) => {
    //             Some(((SMState::II, Signal::IO), (SMState::IO, Signal::OI)))
    //         }
    //     }
    // }
}

#[derive(Debug)]
pub struct Trellis {
    pub table: [[[[Option<Bit>; 4]; 4]; 4]; 4],
}

impl Trellis {
    pub fn new() -> Self {
        let mut table = [[[[None; 4]; 4]; 4]; 4];
        for sms in 0..4 {
            for first_signal in 0..4 {
                for second_signal in 0..4 {
                    for third_signal in 0..4 {
                        // 最もハミング距離が短いbitのパスを最後のMTStateごとに探す
                        let mut OO_bits = None;
                        let mut OI_bits = None;
                        let mut IO_bits = None;
                        let mut II_bits = None;
                        let mut OO_min_diff = None;
                        let mut OI_min_diff = None;
                        let mut IO_min_diff = None;
                        let mut II_min_diff = None;

                        for first_bit in 0..2 {
                            for second_bit in 0..2 {
                                for third_bit in 0..2 {
                                    // calculate
                                    let mut sm: StateMachine = StateMachine::new(sms.into());

                                    let fs: Signal = first_signal.into();
                                    let ss: Signal = second_signal.into();
                                    let ts: Signal = third_signal.into();

                                    let mut diff = Some(0);
                                    diff = diff.map(|a| a + (fs - sm.set(first_bit.into())));
                                    diff = diff.map(|a| a + (ss - sm.set(second_bit.into())));
                                    diff = diff.map(|a| a + (ts - sm.set(third_bit.into())));

                                    // 距離が同じだった場合にこいつ虫で多数決とかいいかもなにか工夫することができるかもしれない
                                    // 距離が同じだった場合min diffをNoneにしてあとでむしする
                                    match sm.state {
                                        SMState::OO => {
                                            if OO_min_diff > diff || OO_min_diff == None {
                                                OO_min_diff = diff;
                                                OO_bits = Some([first_bit, second_bit, third_bit]);
                                            } else if OO_min_diff == diff {
                                                OO_min_diff = None;
                                                OO_bits = None;
                                                panic!("wow equal diff in OO");
                                            }
                                        }
                                        SMState::OI => {
                                            if OI_min_diff > diff || OI_min_diff == None {
                                                OI_min_diff = diff;
                                                OI_bits = Some([first_bit, second_bit, third_bit]);
                                            } else if OI_min_diff == diff {
                                                OI_min_diff = None;
                                                OI_bits = None;
                                                panic!("wow equal diff in OI");
                                            }
                                        }
                                        SMState::IO => {
                                            if IO_min_diff > diff || IO_min_diff == None {
                                                IO_min_diff = diff;
                                                IO_bits = Some([first_bit, second_bit, third_bit]);
                                            } else if IO_min_diff == diff {
                                                IO_min_diff = None;
                                                IO_bits = None;
                                                panic!("wow equal diff in IO");
                                            }
                                        }
                                        SMState::II => {
                                            if II_min_diff > diff || II_min_diff == None {
                                                II_min_diff = diff;
                                                II_bits = Some([first_bit, second_bit, third_bit]);
                                            } else if II_min_diff == diff {
                                                II_min_diff = None;
                                                II_bits = None;
                                                panic!("wow equal diff in II");
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // 多数決で決める
                        let bits_sum = vec![OO_bits, OI_bits, IO_bits, II_bits]
                            // 最初のbitsを撮ってくる必要がある
                            .into_iter()
                            .filter(|a| *a != None)
                            .map(|a| a.unwrap()[0])
                            .fold(0, |acc, item| if item > 1 {panic!("wow")} else {acc + Into::<usize>::into(item)});
                        // dbg!(bits_sum);
                        if bits_sum < 2 {
                            table[sms][first_signal][second_signal][third_signal] = Some(Bit::O);
                        } else if bits_sum > 2 {
                            table[sms][first_signal][second_signal][third_signal] = Some(Bit::I);
                        } else {
                            panic!("wow not much");
                            table[sms][first_signal][second_signal][third_signal] = None;
                        }

                        /*
                        // 一番ハミング距離が短いパスが決める
                        // 同じ距離だったときどうするか
                        let mut diffs = vec![
                            (Signal::OO, OO_min_diff),
                            (Signal::OI, OI_min_diff),
                            (Signal::IO, IO_min_diff),
                            (Signal::II, II_min_diff),
                        ];
                        diffs.sort_by_key(|e| e.1);
                        if diffs[0].1 == diffs[1].1 {
                            if rand::random::<bool>() {
                                // dbg!("swap");
                                diffs.swap(0, 1);
                            }
                        }
                        match diffs[0].0 {
                            Signal::OO => {
                                table[sms][first_signal][second_signal][third_signal] = Some(OO_bits.unwrap()[0].into());
                            }
                            Signal::OI => {
                                table[sms][first_signal][second_signal][third_signal] = Some(OI_bits.unwrap()[0].into());
                            }
                            Signal::IO => {
                                table[sms][first_signal][second_signal][third_signal] = Some(IO_bits.unwrap()[0].into());
                            }
                            Signal::II => {
                                table[sms][first_signal][second_signal][third_signal] = Some(II_bits.unwrap()[0].into());
                            }
                        }
                        */
                    }
                }
            }
        }
        // dbg!(table);
        return Trellis { table };
    }

    pub fn get_next_bit(&self, sms: SMState, sss: (Signal, Signal, Signal)) -> Bit {
        self.table[Into::<usize>::into(sms)][Into::<usize>::into(sss.0)][Into::<usize>::into(sss.1)]
            [Into::<usize>::into(sss.2)]
            .unwrap()
    }
}
