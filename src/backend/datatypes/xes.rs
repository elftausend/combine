
#[derive(Debug, Clone, Copy)]
pub struct XES {
    pub number: i16
}

impl XES {
    pub fn new(n: i16) -> XES {
        XES {number: n}
    }
    #[inline]
    pub fn to_f32(self) -> f32 {
        let mut power = self.number%10;
        if power < 0 {
            power = -power;
        }
        ((self.number/10) as f32)*10f32.powi(-power as i32)
    }
    pub fn package(self, xs: XES) -> (i32, i32){
        let diff = ((self.number%10) - (xs.number%10)).abs() as u32;
  //      println!("diff: {}", diff);
  //      println!("Number: {}", (self.number/10)*10_i16.pow(diff));
        (((self.number/10) as i32)*10_i32.pow(diff), ((xs.number/10) as i32))
    }
    pub fn package2(self, xs: XES) -> (i16, i16) {
        (self.number-(self.number%10), (xs.number-(xs.number%10))/10)
    }
    pub fn exp(&self) -> XES {
        to_xes_format(self.to_f32().exp())
    }
    pub fn pow(&self, pow: f32) -> XES {
        to_xes_format(self.to_f32().powf(pow))
    }
    pub fn sqrt(&self) -> XES {
        to_xes_format(self.to_f32().sqrt())
    }
}

pub fn testxes() {
    
    let xes = XES::new(121); // 0.543
                        // 543 * 10^-3
    let to_dec = xes.to_f32(); 
    let to_dec2 = XES::new(840).to_f32();

    let add = xes*XES::new(840);
    println!("add {:?}", add);

    println!("decimal: {:?}", to_xes_format(to_dec*to_dec2));
}


/*



impl std::ops::Add for XES {
    type Output = XES;
    fn add(self, xs: XES) -> XES {
        return xs;
   //     let package = self.package(xs);
   //     println!("package: {:?}", package);
    //    return XES {number: (((package.0 + package.1) as f32)/10.).round() as i16};

    }
}
*/

impl std::cmp::PartialOrd for XES {
    fn partial_cmp(&self, other: &XES) -> Option<std::cmp::Ordering> {
        self.to_f32().partial_cmp(&other.to_f32())
    }
}

impl Default for XES {
    fn default() -> XES {
        XES {number: 00}
    }
}

impl std::cmp::PartialEq for XES {
    fn eq(&self, other: &XES) -> bool {
        self.to_f32() == other.to_f32()
    }
}

impl std::ops::Neg for XES {
    type Output = XES;
    fn neg(self) -> XES {
        XES {number: -self.number}
    }
}

impl std::ops::Add for XES {
    type Output = XES;
    fn add(self, other: XES) -> XES {
        to_xes_format(self.to_f32()+other.to_f32())
    }
}

impl std::ops::Sub for XES {
    type Output = XES;
    fn sub(self, other: XES) -> XES {
        to_xes_format(self.to_f32()-other.to_f32())
    }
}
impl std::ops::Div for XES {
    type Output = XES;
    fn div(self, other: XES) -> XES {
        to_xes_format(self.to_f32()/other.to_f32())
    }
}

impl std::ops::Mul for XES {
    type Output = XES;
    fn mul(self, other: XES) -> XES {
        to_xes_format(self.to_f32()*other.to_f32())
    }
}
#[inline]
pub fn to_xes_format(mut i: f32) -> XES {
    let c = i;
    if i < 0. {
        i = -i;
    }
   // println!("ii {:?}", i);
    if i == 0. {
        return XES {number: 0,};
    }
    let mut power = 0;
    while i < 100. {
        i *= 10.;
        power += 1;
    }  
 //   println!("i: {:?}", i.round());
 //   println!("p: {}", power-1);

  //  i = i*10.;
    let i = i.round()*10.;
   // println!("power: {}", power);
   if c > 0. {
       if power == 0 {
           return XES {number: i as i16};
       }
        XES {number: i as i16+(power)} //*vllt mit -1 bei power?
   //     return XES {number: i.round() as i8, power: power};
   } else {
       if power == 0 {
            return XES {number: -(i as i16+(power))}; //*
       }
       XES {number: -(i as i16+(power))} //*
 //       return XES {number: -i.round() as i8, power: power};
   }

}
