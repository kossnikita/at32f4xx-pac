#[doc = "Register `MUXGSTS` reader"]
pub type R = crate::R<MUXGSTS_SPEC>;
#[doc = "Register `MUXGSTS` writer"]
pub type W = crate::W<MUXGSTS_SPEC>;
#[doc = "Field `TRGOVF1` reader - Trigger overrun interrupt flag"]
pub type TRGOVF1_R = crate::BitReader;
#[doc = "Field `TRGOVF1` writer - Trigger overrun interrupt flag"]
pub type TRGOVF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRGOVF2` reader - Trigger overrun interrupt flag"]
pub type TRGOVF2_R = crate::BitReader;
#[doc = "Field `TRGOVF2` writer - Trigger overrun interrupt flag"]
pub type TRGOVF2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRGOVF3` reader - Trigger overrun interrupt flag"]
pub type TRGOVF3_R = crate::BitReader;
#[doc = "Field `TRGOVF3` writer - Trigger overrun interrupt flag"]
pub type TRGOVF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRGOVF4` reader - Trigger overrun interrupt flag"]
pub type TRGOVF4_R = crate::BitReader;
#[doc = "Field `TRGOVF4` writer - Trigger overrun interrupt flag"]
pub type TRGOVF4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovf1(&self) -> TRGOVF1_R {
        TRGOVF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovf2(&self) -> TRGOVF2_R {
        TRGOVF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovf3(&self) -> TRGOVF3_R {
        TRGOVF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovf4(&self) -> TRGOVF4_R {
        TRGOVF4_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovf1(&mut self) -> TRGOVF1_W<MUXGSTS_SPEC, 0> {
        TRGOVF1_W::new(self)
    }
    #[doc = "Bit 1 - Trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovf2(&mut self) -> TRGOVF2_W<MUXGSTS_SPEC, 1> {
        TRGOVF2_W::new(self)
    }
    #[doc = "Bit 2 - Trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovf3(&mut self) -> TRGOVF3_W<MUXGSTS_SPEC, 2> {
        TRGOVF3_W::new(self)
    }
    #[doc = "Bit 3 - Trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovf4(&mut self) -> TRGOVF4_W<MUXGSTS_SPEC, 3> {
        TRGOVF4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Generator Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxgsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxgsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXGSTS_SPEC;
impl crate::RegisterSpec for MUXGSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxgsts::R`](R) reader structure"]
impl crate::Readable for MUXGSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxgsts::W`](W) writer structure"]
impl crate::Writable for MUXGSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXGSTS to value 0"]
impl crate::Resettable for MUXGSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
