#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICACHECNF {
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
#[doc = "Possible values of the field `CACHEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEENR {
    #[doc = "Disable cache. Invalidates all cache entries."]
    DISABLED,
    #[doc = "Enable cache"]
    ENABLED,
}
impl CACHEENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CACHEENR::DISABLED => false,
            CACHEENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CACHEENR {
        match value {
            false => CACHEENR::DISABLED,
            true => CACHEENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CACHEENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CACHEENR::ENABLED
    }
}
#[doc = "Possible values of the field `CACHEPROFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEPROFENR {
    #[doc = "Disable cache profiling"]
    DISABLED,
    #[doc = "Enable cache profiling"]
    ENABLED,
}
impl CACHEPROFENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CACHEPROFENR::DISABLED => false,
            CACHEPROFENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CACHEPROFENR {
        match value {
            false => CACHEPROFENR::DISABLED,
            true => CACHEPROFENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CACHEPROFENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CACHEPROFENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `CACHEEN`"]
pub enum CACHEENW {
    #[doc = "Disable cache. Invalidates all cache entries."]
    DISABLED,
    #[doc = "Enable cache"]
    ENABLED,
}
impl CACHEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CACHEENW::DISABLED => false,
            CACHEENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CACHEENW<'a> {
    w: &'a mut W,
}
impl<'a> _CACHEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CACHEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable cache. Invalidates all cache entries."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CACHEENW::DISABLED)
    }
    #[doc = "Enable cache"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CACHEENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CACHEPROFEN`"]
pub enum CACHEPROFENW {
    #[doc = "Disable cache profiling"]
    DISABLED,
    #[doc = "Enable cache profiling"]
    ENABLED,
}
impl CACHEPROFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CACHEPROFENW::DISABLED => false,
            CACHEPROFENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CACHEPROFENW<'a> {
    w: &'a mut W,
}
impl<'a> _CACHEPROFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CACHEPROFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable cache profiling"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CACHEPROFENW::DISABLED)
    }
    #[doc = "Enable cache profiling"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CACHEPROFENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Cache enable"]
    #[inline]
    pub fn cacheen(&self) -> CACHEENR {
        CACHEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Cache profiling enable"]
    #[inline]
    pub fn cacheprofen(&self) -> CACHEPROFENR {
        CACHEPROFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Cache enable"]
    #[inline]
    pub fn cacheen(&mut self) -> _CACHEENW {
        _CACHEENW { w: self }
    }
    #[doc = "Bit 8 - Cache profiling enable"]
    #[inline]
    pub fn cacheprofen(&mut self) -> _CACHEPROFENW {
        _CACHEPROFENW { w: self }
    }
}
