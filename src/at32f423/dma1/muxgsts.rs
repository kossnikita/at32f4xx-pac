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
            .field("trgovf1", &self.trgovf1())
            .field("trgovf2", &self.trgovf2())
            .field("trgovf3", &self.trgovf3())
            .field("trgovf4", &self.trgovf4())
            .finish()
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
}
#[doc = "Generator Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxgsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxgsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXGSTS_SPEC;
impl crate::RegisterSpec for MUXGSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxgsts::R`](R) reader structure"]
impl crate::Readable for MUXGSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxgsts::W`](W) writer structure"]
impl crate::Writable for MUXGSTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUXGSTS to value 0"]
impl crate::Resettable for MUXGSTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
