#[doc = "Register `BK2IS` reader"]
pub type R = crate::R<BK2IS_SPEC>;
#[doc = "Register `BK2IS` writer"]
pub type W = crate::W<BK2IS_SPEC>;
#[doc = "Field `RES` reader - Rising edge capture status"]
pub type RES_R = crate::BitReader;
#[doc = "Field `RES` writer - Rising edge capture status"]
pub type RES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HLS` reader - High-level status"]
pub type HLS_R = crate::BitReader;
#[doc = "Field `HLS` writer - High-level status"]
pub type HLS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FES` reader - Falling edge status"]
pub type FES_R = crate::BitReader;
#[doc = "Field `FES` writer - Falling edge status"]
pub type FES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REIEN` reader - Rising edge interrupt enable"]
pub type REIEN_R = crate::BitReader;
#[doc = "Field `REIEN` writer - Rising edge interrupt enable"]
pub type REIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HLIEN` reader - High-level interrupt enable"]
pub type HLIEN_R = crate::BitReader;
#[doc = "Field `HLIEN` writer - High-level interrupt enable"]
pub type HLIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIEN` reader - Falling edge interrupt enable"]
pub type FEIEN_R = crate::BitReader;
#[doc = "Field `FEIEN` writer - Falling edge interrupt enable"]
pub type FEIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOE` reader - FIFO empty"]
pub type FIFOE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Rising edge capture status"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High-level status"]
    #[inline(always)]
    pub fn hls(&self) -> HLS_R {
        HLS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling edge status"]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising edge interrupt enable"]
    #[inline(always)]
    pub fn reien(&self) -> REIEN_R {
        REIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - High-level interrupt enable"]
    #[inline(always)]
    pub fn hlien(&self) -> HLIEN_R {
        HLIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling edge interrupt enable"]
    #[inline(always)]
    pub fn feien(&self) -> FEIEN_R {
        FEIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO empty"]
    #[inline(always)]
    pub fn fifoe(&self) -> FIFOE_R {
        FIFOE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK2IS")
            .field("fifoe", &format_args!("{}", self.fifoe().bit()))
            .field("feien", &format_args!("{}", self.feien().bit()))
            .field("hlien", &format_args!("{}", self.hlien().bit()))
            .field("reien", &format_args!("{}", self.reien().bit()))
            .field("fes", &format_args!("{}", self.fes().bit()))
            .field("hls", &format_args!("{}", self.hls().bit()))
            .field("res", &format_args!("{}", self.res().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<BK2IS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Rising edge capture status"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<BK2IS_SPEC> {
        RES_W::new(self, 0)
    }
    #[doc = "Bit 1 - High-level status"]
    #[inline(always)]
    #[must_use]
    pub fn hls(&mut self) -> HLS_W<BK2IS_SPEC> {
        HLS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Falling edge status"]
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FES_W<BK2IS_SPEC> {
        FES_W::new(self, 2)
    }
    #[doc = "Bit 3 - Rising edge interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn reien(&mut self) -> REIEN_W<BK2IS_SPEC> {
        REIEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - High-level interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hlien(&mut self) -> HLIEN_W<BK2IS_SPEC> {
        HLIEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Falling edge interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn feien(&mut self) -> FEIEN_W<BK2IS_SPEC> {
        FEIEN_W::new(self, 5)
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
#[doc = "FIFO status and interrupt register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2is::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2is::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BK2IS_SPEC;
impl crate::RegisterSpec for BK2IS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk2is::R`](R) reader structure"]
impl crate::Readable for BK2IS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bk2is::W`](W) writer structure"]
impl crate::Writable for BK2IS_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BK2IS to value 0x40"]
impl crate::Resettable for BK2IS_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
