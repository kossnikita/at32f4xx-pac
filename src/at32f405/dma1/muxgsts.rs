#[doc = "Register `MUXGSTS` reader"]
pub type R = crate::R<MUXGSTS_SPEC>;
#[doc = "Register `MUXGSTS` writer"]
pub type W = crate::W<MUXGSTS_SPEC>;
#[doc = "Field `TRGOVF1` reader - Trigger overrun interrupt flag"]
pub type TRGOVF1_R = crate::BitReader;
#[doc = "Field `TRGOVF1` writer - Trigger overrun interrupt flag"]
pub type TRGOVF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGOVF2` reader - Trigger overrun interrupt flag"]
pub type TRGOVF2_R = crate::BitReader;
#[doc = "Field `TRGOVF2` writer - Trigger overrun interrupt flag"]
pub type TRGOVF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGOVF3` reader - Trigger overrun interrupt flag"]
pub type TRGOVF3_R = crate::BitReader;
#[doc = "Field `TRGOVF3` writer - Trigger overrun interrupt flag"]
pub type TRGOVF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGOVF4` reader - Trigger overrun interrupt flag"]
pub type TRGOVF4_R = crate::BitReader;
#[doc = "Field `TRGOVF4` writer - Trigger overrun interrupt flag"]
pub type TRGOVF4_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXGSTS")
            .field("trgovf1", &format_args!("{}", self.trgovf1().bit()))
            .field("trgovf2", &format_args!("{}", self.trgovf2().bit()))
            .field("trgovf3", &format_args!("{}", self.trgovf3().bit()))
            .field("trgovf4", &format_args!("{}", self.trgovf4().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MUXGSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovf1(&mut self) -> TRGOVF1_W<MUXGSTS_SPEC> {
        TRGOVF1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovf2(&mut self) -> TRGOVF2_W<MUXGSTS_SPEC> {
        TRGOVF2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovf3(&mut self) -> TRGOVF3_W<MUXGSTS_SPEC> {
        TRGOVF3_W::new(self, 2)
    }
    #[doc = "Bit 3 - Trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovf4(&mut self) -> TRGOVF4_W<MUXGSTS_SPEC> {
        TRGOVF4_W::new(self, 3)
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
#[doc = "Generator Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxgsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxgsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXGSTS_SPEC;
impl crate::RegisterSpec for MUXGSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxgsts::R`](R) reader structure"]
impl crate::Readable for MUXGSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxgsts::W`](W) writer structure"]
impl crate::Writable for MUXGSTS_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXGSTS to value 0"]
impl crate::Resettable for MUXGSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
