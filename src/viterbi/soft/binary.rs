use crate::box_muller;

// 0 or 1
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Bit(pub usize);

impl Into<usize> for Bit {
    fn into(self) -> usize {
        if self.0 == 0 {
            0
        } else if self.0 == 1 {
            1
        } else {
            panic!("cant convert {:#?} to usize", self);
        }
    }
}

pub fn into_2bits(num: usize) -> (Bit, Bit) {
    if num == 0 {
        (Bit(0), Bit(0))
    } else if num == 1 {
        (Bit(0), Bit(1))
    } else if num == 2 {
        (Bit(1), Bit(0))
    } else if num == 3 {
        (Bit(1), Bit(1))
    } else {
        panic!("cant convert {:#?} to (Bit, Bit)", num);
    }
}

// (0, 1) -> (-1, 1)
pub fn bpsk(bits: Signal) -> (isize, isize) {
    match bits {
        Signal(Bit(0), Bit(0)) => (-1, -1),
        Signal(Bit(0), Bit(1)) => (-1, 1),
        Signal(Bit(1), Bit(0)) => (1, -1),
        Signal(Bit(1), Bit(1)) => (1, 1),
        _ => {
            panic!("cant converto {:?} to (bpsk, bpsk)", bits);
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Signal(pub Bit, pub Bit);

impl Signal {
    pub fn add_noise(self, sigma: f64) -> NoisedSignal {
        let ex1 = box_muller::box_muller();
        let ex2 = box_muller::box_muller();
        let (u1, u2) = {
            let Signal(Bit(b1), Bit(b2)) = self;
            let mut r1 = b1 as isize;
            let mut r2 = b2 as isize;
            if b1 == 0 {
                r1 = -1;
            }
            if b2 == 0 {
                r2 = -1;
            }
            (r1, r2)
        };
        NoisedSignal(u1 as f64 + ex1 * sigma, u2 as f64 + ex2 * sigma)
    }
}

// 0が-1にマッピングされて、ガウス通信路を通る
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NoisedSignal(pub f64, pub f64);

#[derive(Debug, Copy, Clone)]
pub struct StateMachine(pub Bit, pub Bit);

impl Into<usize> for StateMachine {
    fn into(self) -> usize {
        match self {
            StateMachine(Bit(0), Bit(0)) => 0,
            StateMachine(Bit(0), Bit(1)) => 1,
            StateMachine(Bit(1), Bit(0)) => 2,
            StateMachine(Bit(1), Bit(1)) => 3,
            _ => {
                panic!("cant convert {:#?} to usize", self);
            }
        }
    }
}

impl From<(Bit, Bit)> for StateMachine {
    fn from(bits: (Bit, Bit)) -> Self {
        StateMachine(bits.0, bits.1)
    }
}

impl StateMachine {
    pub fn new(sms: (Bit, Bit)) -> Self {
        StateMachine(sms.0, sms.1)
    }
    pub fn set(&mut self, bit: Bit) -> Signal {
        match (self.0, self.1, bit) {
            (Bit(0), Bit(0), Bit(0)) => {
                self.0 = Bit(0);
                self.1 = Bit(0);
                Signal(Bit(0), Bit(0))
            }
            (Bit(0), Bit(0), Bit(1)) => {
                self.0 = Bit(1);
                self.1 = Bit(0);
                Signal(Bit(1), Bit(1))
            }
            (Bit(0), Bit(1), Bit(0)) => {
                self.0 = Bit(0);
                self.1 = Bit(0);
                Signal(Bit(1), Bit(1))
            }
            (Bit(0), Bit(1), Bit(1)) => {
                self.0 = Bit(1);
                self.1 = Bit(0);
                Signal(Bit(0), Bit(0))
            }
            (Bit(1), Bit(0), Bit(0)) => {
                self.0 = Bit(0);
                self.1 = Bit(1);
                Signal(Bit(1), Bit(0))
            }
            (Bit(1), Bit(0), Bit(1)) => {
                self.0 = Bit(1);
                self.1 = Bit(1);
                Signal(Bit(0), Bit(1))
            }
            (Bit(1), Bit(1), Bit(0)) => {
                self.0 = Bit(0);
                self.1 = Bit(1);
                Signal(Bit(0), Bit(1))
            }
            (Bit(1), Bit(1), Bit(1)) => {
                self.0 = Bit(1);
                self.1 = Bit(1);
                Signal(Bit(1), Bit(0))
            }
            _ => {
                panic!("i don't know this state: {:#?}", self);
            }
        }
    }
}
