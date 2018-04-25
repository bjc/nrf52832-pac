#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ACCDBL {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ACCDBLR {
    bits: u8,
}
impl ACCDBLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Register accumulating the number of detected double or illegal transitions. ( SAMPLE = 2 )."]
    #[inline]
    pub fn accdbl(&self) -> ACCDBLR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ACCDBLR { bits }
    }
}
