#[doc = "Register `S2DCTRL` reader"]
pub type R = crate::R<S2DCTRL_SPEC>;
#[doc = "Register `S2DCTRL` writer"]
pub type W = crate::W<S2DCTRL_SPEC>;
#[doc = "Field `S1_2DEN` reader - Stream 1 2D transfer enable"]
pub type S1_2DEN_R = crate::BitReader;
#[doc = "Field `S1_2DEN` writer - Stream 1 2D transfer enable"]
pub type S1_2DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2_2DEN` reader - Stream 2 2D transfer enable"]
pub type S2_2DEN_R = crate::BitReader;
#[doc = "Field `S2_2DEN` writer - Stream 2 2D transfer enable"]
pub type S2_2DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S3_2DEN` reader - Stream 3 2D transfer enable"]
pub type S3_2DEN_R = crate::BitReader;
#[doc = "Field `S3_2DEN` writer - Stream 3 2D transfer enable"]
pub type S3_2DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S4_2DEN` reader - Stream 4 2D transfer enable"]
pub type S4_2DEN_R = crate::BitReader;
#[doc = "Field `S4_2DEN` writer - Stream 4 2D transfer enable"]
pub type S4_2DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S5_2DEN` reader - Stream 5 2D transfer enable"]
pub type S5_2DEN_R = crate::BitReader;
#[doc = "Field `S5_2DEN` writer - Stream 5 2D transfer enable"]
pub type S5_2DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S6_2DEN` reader - Stream 6 2D transfer enable"]
pub type S6_2DEN_R = crate::BitReader;
#[doc = "Field `S6_2DEN` writer - Stream 6 2D transfer enable"]
pub type S6_2DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S7_2DEN` reader - Stream 7 2D transfer enable"]
pub type S7_2DEN_R = crate::BitReader;
#[doc = "Field `S7_2DEN` writer - Stream 7 2D transfer enable"]
pub type S7_2DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S8_2DEN` reader - Stream 8 2D transfer enable"]
pub type S8_2DEN_R = crate::BitReader;
#[doc = "Field `S8_2DEN` writer - Stream 8 2D transfer enable"]
pub type S8_2DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stream 1 2D transfer enable"]
    #[inline(always)]
    pub fn s1_2den(&self) -> S1_2DEN_R {
        S1_2DEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stream 2 2D transfer enable"]
    #[inline(always)]
    pub fn s2_2den(&self) -> S2_2DEN_R {
        S2_2DEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stream 3 2D transfer enable"]
    #[inline(always)]
    pub fn s3_2den(&self) -> S3_2DEN_R {
        S3_2DEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream 4 2D transfer enable"]
    #[inline(always)]
    pub fn s4_2den(&self) -> S4_2DEN_R {
        S4_2DEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream 5 2D transfer enable"]
    #[inline(always)]
    pub fn s5_2den(&self) -> S5_2DEN_R {
        S5_2DEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream 6 2D transfer enable"]
    #[inline(always)]
    pub fn s6_2den(&self) -> S6_2DEN_R {
        S6_2DEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream 7 2D transfer enable"]
    #[inline(always)]
    pub fn s7_2den(&self) -> S7_2DEN_R {
        S7_2DEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Stream 8 2D transfer enable"]
    #[inline(always)]
    pub fn s8_2den(&self) -> S8_2DEN_R {
        S8_2DEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S2DCTRL")
            .field("s1_2den", &self.s1_2den())
            .field("s2_2den", &self.s2_2den())
            .field("s3_2den", &self.s3_2den())
            .field("s4_2den", &self.s4_2den())
            .field("s5_2den", &self.s5_2den())
            .field("s6_2den", &self.s6_2den())
            .field("s7_2den", &self.s7_2den())
            .field("s8_2den", &self.s8_2den())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Stream 1 2D transfer enable"]
    #[inline(always)]
    pub fn s1_2den(&mut self) -> S1_2DEN_W<'_, S2DCTRL_SPEC> {
        S1_2DEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Stream 2 2D transfer enable"]
    #[inline(always)]
    pub fn s2_2den(&mut self) -> S2_2DEN_W<'_, S2DCTRL_SPEC> {
        S2_2DEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Stream 3 2D transfer enable"]
    #[inline(always)]
    pub fn s3_2den(&mut self) -> S3_2DEN_W<'_, S2DCTRL_SPEC> {
        S3_2DEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Stream 4 2D transfer enable"]
    #[inline(always)]
    pub fn s4_2den(&mut self) -> S4_2DEN_W<'_, S2DCTRL_SPEC> {
        S4_2DEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Stream 5 2D transfer enable"]
    #[inline(always)]
    pub fn s5_2den(&mut self) -> S5_2DEN_W<'_, S2DCTRL_SPEC> {
        S5_2DEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Stream 6 2D transfer enable"]
    #[inline(always)]
    pub fn s6_2den(&mut self) -> S6_2DEN_W<'_, S2DCTRL_SPEC> {
        S6_2DEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Stream 7 2D transfer enable"]
    #[inline(always)]
    pub fn s7_2den(&mut self) -> S7_2DEN_W<'_, S2DCTRL_SPEC> {
        S7_2DEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Stream 8 2D transfer enable"]
    #[inline(always)]
    pub fn s8_2den(&mut self) -> S8_2DEN_W<'_, S2DCTRL_SPEC> {
        S8_2DEN_W::new(self, 7)
    }
}
#[doc = "EDMA 2D Transfer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s2dctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2dctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2DCTRL_SPEC;
impl crate::RegisterSpec for S2DCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s2dctrl::R`](R) reader structure"]
impl crate::Readable for S2DCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s2dctrl::W`](W) writer structure"]
impl crate::Writable for S2DCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S2DCTRL to value 0"]
impl crate::Resettable for S2DCTRL_SPEC {}
