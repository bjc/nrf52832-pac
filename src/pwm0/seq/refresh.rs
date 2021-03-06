#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REFRESH {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `CNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTR {
    #[doc = "Update every PWM period"]
    CONTINUOUS,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            CNTR::CONTINUOUS => 0,
            CNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> CNTR {
        match value {
            0 => CNTR::CONTINUOUS,
            i => CNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == CNTR::CONTINUOUS
    }
}
#[doc = "Values that can be written to the field `CNT`"]
pub enum CNTW {
    #[doc = "Update every PWM period"]
    CONTINUOUS,
}
impl CNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            CNTW::CONTINUOUS => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Update every PWM period"]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CNTW::CONTINUOUS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:23 - Amount of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)"]
    #[inline]
    pub fn cnt(&self) -> CNTR {
        CNTR::_from({
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:23 - Amount of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)"]
    #[inline]
    pub fn cnt(&mut self) -> _CNTW {
        _CNTW { w: self }
    }
}
