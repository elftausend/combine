
#[derive(Debug, Clone, Copy)]
pub struct UES 
{
    pub number: i8, 
    pub power: i8,  // -128-127

}

impl UES {
    pub fn new(n: f32) -> UES {
        to_ues_format(n)
    }
    pub fn to_decimal(&self) -> f32 {
        self.number as f32*10f32.powi(self.power as i32)
    }
    pub fn pow(&self, pow: f32) -> UES {
        to_ues_format(self.to_decimal().powf(pow))
    }
    pub fn sqrt(&self) -> UES {
        to_ues_format(self.to_decimal().sqrt())
    }
    pub fn exp(&self) -> UES {
        to_ues_format(self.to_decimal().exp())
    }
}

impl std::ops::Neg for UES {
    type Output = UES;
    fn neg(self) -> UES {
        UES {number: -self.number, power: self.power}
    }
}

impl std::ops::Add for UES {
    type Output = UES;
    fn add(self, other: UES) -> UES {
        to_ues_format(self.to_decimal()+other.to_decimal())
    }
}

impl std::ops::Sub for UES {
    type Output = UES;
    fn sub(self, other: UES) -> UES {
        to_ues_format(self.to_decimal()-other.to_decimal())
    }
}
impl std::ops::Div for UES {
    type Output = UES;
    fn div(self, other: UES) -> UES {
        to_ues_format(self.to_decimal()/other.to_decimal())
    }
}

impl std::ops::Mul for UES {
    type Output = UES;
    fn mul(self, other: UES) -> UES {
        to_ues_format(self.to_decimal()*other.to_decimal())
    }
}

impl std::cmp::PartialOrd for UES {
    fn partial_cmp(&self, other: &UES) -> Option<std::cmp::Ordering> {
        self.to_decimal().partial_cmp(&other.to_decimal())
    }
}



impl Default for UES {
    fn default() -> UES {
        UES {number: 0, power: 0}
    }
}

impl std::cmp::PartialEq for UES {
    fn eq(&self, other: &UES) -> bool {
        self.to_decimal() == other.to_decimal()
    }
}

pub fn to_ues_format(mut i: f32) -> UES {
    let c = i;
    i = i.abs();
    if i == 0. {
        return UES {number: 0, power: 0};
    }
    let mut power = 0;
    if i < 10. {
        while i < 10. {
            i *= 10.;
       //     println!("{:?}", i);
            power -= 1;
        }
    } else {
        while i > 100. {
            i /= 10.;
        //    println!("{:?}", i);
            power += 1;
        }
    }
   // println!("{:?}", i);
   if c > 0. {
        UES {number: i.round() as i8, power}
   } else {
        UES {number: -i.round() as i8, power}
   }

}