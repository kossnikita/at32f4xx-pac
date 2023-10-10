#[doc = "Register `MUXGCLR` reader"]
pub type R = crate::R<MUXGCLR_SPEC>;
#[doc = "Register `MUXGCLR` writer"]
pub type W = crate::W<MUXGCLR_SPEC>;
#[doc = "Field `TRGOVFC1` reader - Clear trigger overrun interrupt flag"]
pub type TRGOVFC1_R = crate::BitReader;
#[doc = "Field `TRGOVFC1` writer - Clear trigger overrun interrupt flag"]
pub type TRGOVFC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRGOVFC2` reader - Clear trigger overrun interrupt flag"]
pub type TRGOVFC2_R = crate::BitReader;
#[doc = "Field `TRGOVFC2` writer - Clear trigger overrun interrupt flag"]
pub type TRGOVFC2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRGOVFC3` reader - Clear trigger overrun interrupt flag"]
pub type TRGOVFC3_R = crate::BitReader;
#[doc = "Field `TRGOVFC3` writer - Clear trigger overrun interrupt flag"]
pub type TRGOVFC3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRGOVFC4` reader - Clear trigger overrun interrupt flag"]
pub type TRGOVFC4_R = crate::BitReader;
#[doc = "Field `TRGOVFC4` writer - Clear trigger overrun interrupt flag"]
pub type TRGOVFC4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovfc1(&self) -> TRGOVFC1_R {
        TRGOVFC1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovfc2(&self) -> TRGOVFC2_R {
        TRGOVFC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovfc3(&self) -> TRGOVFC3_R {
        TRGOVFC3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovfc4(&self) -> TRGOVFC4_R {
        TRGOVFC4_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXGCLR")
            .field("trgovfc1", &format_args!("{}", self.trgovfc1().bit()))
            .field("trgovfc2", &format_args!("{}", self.trgovfc2().bit()))
            .field("trgovfc3", &format_args!("{}", self.trgovfc3().bit()))
            .field("trgovfc4", &format_args!("{}", self.trgovfc4().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MUXGCLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovfc1(&mut self) -> TRGOVFC1_W<MUXGCLR_SPEC, 0> {
        TRGOVFC1_W::new(self)
    }
    #[doc = "Bit 1 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovfc2(&mut self) -> TRGOVFC2_W<MUXGCLR_SPEC, 1> {
        TRGOVFC2_W::new(self)
    }
    #[doc = "Bit 2 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovfc3(&mut self) -> TRGOVFC3_W<MUXGCLR_SPEC, 2> {
        TRGOVFC3_W::new(self)
    }
    #[doc = "Bit 3 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovfc4(&mut self) -> TRGOVFC4_W<MUXGCLR_SPEC, 3> {
        TRGOVFC4_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Generator Interrupt Clear Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxgclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxgclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXGCLR_SPEC;
impl crate::RegisterSpec for MUXGCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxgclr::R`](R) reader structure"]
impl crate::Readable for MUXGCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxgclr::W`](W) writer structure"]
impl crate::Writable for MUXGCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXGCLR to value 0"]
impl crate::Resettable for MUXGCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
