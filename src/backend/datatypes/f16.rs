
#[derive(Debug, Clone, Copy)]
pub struct F16(u16);

impl F16 {

    pub fn to_f32(self) -> f32 {
        if self.0 & 0x7FFFu16 > 0x7F80u16 {
            f32::from_bits((self.0 as u32 | 0x0040u32) << 16)
        } else {
            f32::from_bits((self.0 as u32) << 16)
        }
    }

    pub fn from_f32(value: f32) -> F16 {
        let x = value.to_bits();
        let round = 0x0000_8000u32;

        if (x & round) != 0 && (x & (3 * round - 1)) != 0 {
            F16((x >> 16) as u16 + 1)
        } else {
            F16((x >> 16) as u16)
        }

    }
}

impl Default for F16 {
    fn default() -> F16 {
        F16::from_f32(0.0)
    }
}

impl std::cmp::PartialOrd for F16 {
    fn partial_cmp(&self, other: &F16) -> Option<std::cmp::Ordering> {
        self.to_f32().partial_cmp(&other.to_f32())
    }
}


impl std::cmp::PartialEq for F16 {
    fn eq(&self, other: &F16) -> bool {
        self.to_f32() == other.to_f32()
    }
}

impl std::ops::Neg for F16 {
    type Output = F16;
    fn neg(self) -> F16 {
        F16::from_f32(-self.to_f32())
    }
}


impl std::ops::Add for F16 {
    type Output = F16;
    fn add(self, other: F16) -> F16 {
        F16::from_f32(self.to_f32()+other.to_f32())
    }
}

impl std::ops::Sub for F16 {
    type Output = F16;
    fn sub(self, other: F16) -> F16 {
        F16::from_f32(self.to_f32()-other.to_f32())
    }
}
impl std::ops::Div for F16 {
    type Output = F16;
    fn div(self, other: F16) -> F16 {
        F16::from_f32(self.to_f32()/other.to_f32())
    }
}

impl std::ops::Mul for F16 {
    type Output = F16;
    fn mul(self, other: F16) -> F16 {
        F16::from_f32(self.to_f32()*other.to_f32())
    }
}