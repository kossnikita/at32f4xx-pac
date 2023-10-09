#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Field `CALRDY` reader - Internal high-speed clock calibration ready"]
pub type CALRDY_R = crate::BitReader<CALRDY_A>;
#[doc = "Internal high-speed clock calibration ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALRDY_A {
    #[doc = "0: Interal 8MHz oscillator calibration is not ready"]
    NotReady = 0,
    #[doc = "1: Interal 8MHz oscillator calibration is ready"]
    Ready = 1,
}
impl From<CALRDY_A> for bool {
    #[inline(always)]
    fn from(variant: CALRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl CALRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALRDY_A {
        match self.bits {
            false => CALRDY_A::NotReady,
            true => CALRDY_A::Ready,
        }
    }
    #[doc = "Interal 8MHz oscillator calibration is not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == CALRDY_A::NotReady
    }
    #[doc = "Interal 8MHz oscillator calibration is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == CALRDY_A::Ready
    }
}
#[doc = "Field `RSLOST` reader - Reference Signal Lost"]
pub type RSLOST_R = crate::BitReader<RSLOST_A>;
#[doc = "Reference Signal Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSLOST_A {
    #[doc = "0: Reference Signal is not lost"]
    NotLost = 0,
    #[doc = "1: Reference Signal is lost"]
    Lost = 1,
}
impl From<RSLOST_A> for bool {
    #[inline(always)]
    fn from(variant: RSLOST_A) -> Self {
        variant as u8 != 0
    }
}
impl RSLOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSLOST_A {
        match self.bits {
            false => RSLOST_A::NotLost,
            true => RSLOST_A::Lost,
        }
    }
    #[doc = "Reference Signal is not lost"]
    #[inline(always)]
    pub fn is_not_lost(&self) -> bool {
        *self == RSLOST_A::NotLost
    }
    #[doc = "Reference Signal is lost"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == RSLOST_A::Lost
    }
}
impl R {
    #[doc = "Bit 0 - Internal high-speed clock calibration ready"]
    #[inline(always)]
    pub fn calrdy(&self) -> CALRDY_R {
        CALRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reference Signal Lost"]
    #[inline(always)]
    pub fn rslost(&self) -> RSLOST_R {
        RSLOST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("rslost", &format_args!("{}", self.rslost().bit()))
            .field("calrdy", &format_args!("{}", self.calrdy().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
