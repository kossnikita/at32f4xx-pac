#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Field `RLDF` reader - Reload counter interrupt flag"]
pub type RLDF_R = crate::BitReader<RLDFR_A>;
#[doc = "Reload counter interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RLDFR_A {
    #[doc = "0: Downcounter doesn't reached 0x40"]
    NotReached = 0,
    #[doc = "1: Downcounter reached 0x40"]
    Reached = 1,
}
impl From<RLDFR_A> for bool {
    #[inline(always)]
    fn from(variant: RLDFR_A) -> Self {
        variant as u8 != 0
    }
}
impl RLDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLDFR_A {
        match self.bits {
            false => RLDFR_A::NotReached,
            true => RLDFR_A::Reached,
        }
    }
    #[doc = "Downcounter doesn't reached 0x40"]
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == RLDFR_A::NotReached
    }
    #[doc = "Downcounter reached 0x40"]
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == RLDFR_A::Reached
    }
}
#[doc = "Reload counter interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RLDFW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<RLDFW_AW> for bool {
    #[inline(always)]
    fn from(variant: RLDFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLDF` writer - Reload counter interrupt flag"]
pub type RLDF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, RLDFW_AW>;
impl<'a, REG, const O: u8> RLDF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RLDFW_AW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Reload counter interrupt flag"]
    #[inline(always)]
    pub fn rldf(&self) -> RLDF_R {
        RLDF_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("rldf", &format_args!("{}", self.rldf().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Reload counter interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn rldf(&mut self) -> RLDF_W<STS_SPEC, 0> {
        RLDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
