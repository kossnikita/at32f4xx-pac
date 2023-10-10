#[doc = "Register `AHBRST1` reader"]
pub type R = crate::R<AHBRST1_SPEC>;
#[doc = "Register `AHBRST1` writer"]
pub type W = crate::W<AHBRST1_SPEC>;
#[doc = "Field `GPIOA` reader - IO port A reset"]
pub type GPIOA_R = crate::BitReader;
#[doc = "Field `GPIOA` writer - IO port A reset"]
pub type GPIOA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOB` reader - IO port B reset"]
pub type GPIOB_R = crate::BitReader;
#[doc = "Field `GPIOB` writer - IO port B reset"]
pub type GPIOB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOC` reader - IO port C reset"]
pub type GPIOC_R = crate::BitReader;
#[doc = "Field `GPIOC` writer - IO port C reset"]
pub type GPIOC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOD` reader - IO port D reset"]
pub type GPIOD_R = crate::BitReader;
#[doc = "Field `GPIOD` writer - IO port D reset"]
pub type GPIOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOE` reader - IO port E reset"]
pub type GPIOE_R = crate::BitReader;
#[doc = "Field `GPIOE` writer - IO port E reset"]
pub type GPIOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOF` reader - IO port F reset"]
pub type GPIOF_R = crate::BitReader;
#[doc = "Field `GPIOF` writer - IO port F reset"]
pub type GPIOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOG` reader - IO port G reset"]
pub type GPIOG_R = crate::BitReader;
#[doc = "Field `GPIOG` writer - IO port G reset"]
pub type GPIOG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOH` reader - IO port H reset"]
pub type GPIOH_R = crate::BitReader;
#[doc = "Field `GPIOH` writer - IO port H reset"]
pub type GPIOH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRC` reader - CRC reset"]
pub type CRC_R = crate::BitReader;
#[doc = "Field `CRC` writer - CRC reset"]
pub type CRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EDMA` reader - EDMA reset"]
pub type EDMA_R = crate::BitReader;
#[doc = "Field `EDMA` writer - EDMA reset"]
pub type EDMA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA1` reader - DMA1 reset"]
pub type DMA1_R = crate::BitReader;
#[doc = "Field `DMA1` writer - DMA1 reset"]
pub type DMA1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA2` reader - DMA2 reset"]
pub type DMA2_R = crate::BitReader;
#[doc = "Field `DMA2` writer - DMA2 reset"]
pub type DMA2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMAC` reader - EMAC reset"]
pub type EMAC_R = crate::BitReader;
#[doc = "Field `EMAC` writer - EMAC reset"]
pub type EMAC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OTGFS2` reader - OTGFS2 interface reset"]
pub type OTGFS2_R = crate::BitReader;
#[doc = "Field `OTGFS2` writer - OTGFS2 interface reset"]
pub type OTGFS2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    pub fn gpioa(&self) -> GPIOA_R {
        GPIOA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    pub fn gpiob(&self) -> GPIOB_R {
        GPIOB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    pub fn gpioc(&self) -> GPIOC_R {
        GPIOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    pub fn gpiod(&self) -> GPIOD_R {
        GPIOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    pub fn gpioe(&self) -> GPIOE_R {
        GPIOE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F reset"]
    #[inline(always)]
    pub fn gpiof(&self) -> GPIOF_R {
        GPIOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G reset"]
    #[inline(always)]
    pub fn gpiog(&self) -> GPIOG_R {
        GPIOG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H reset"]
    #[inline(always)]
    pub fn gpioh(&self) -> GPIOH_R {
        GPIOH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - EDMA reset"]
    #[inline(always)]
    pub fn edma(&self) -> EDMA_R {
        EDMA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1(&self) -> DMA1_R {
        DMA1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2(&self) -> DMA2_R {
        DMA2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EMAC reset"]
    #[inline(always)]
    pub fn emac(&self) -> EMAC_R {
        EMAC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - OTGFS2 interface reset"]
    #[inline(always)]
    pub fn otgfs2(&self) -> OTGFS2_R {
        OTGFS2_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRST1")
            .field("gpioa", &format_args!("{}", self.gpioa().bit()))
            .field("gpiob", &format_args!("{}", self.gpiob().bit()))
            .field("gpioc", &format_args!("{}", self.gpioc().bit()))
            .field("gpiod", &format_args!("{}", self.gpiod().bit()))
            .field("gpioe", &format_args!("{}", self.gpioe().bit()))
            .field("gpiof", &format_args!("{}", self.gpiof().bit()))
            .field("gpiog", &format_args!("{}", self.gpiog().bit()))
            .field("gpioh", &format_args!("{}", self.gpioh().bit()))
            .field("crc", &format_args!("{}", self.crc().bit()))
            .field("edma", &format_args!("{}", self.edma().bit()))
            .field("dma1", &format_args!("{}", self.dma1().bit()))
            .field("dma2", &format_args!("{}", self.dma2().bit()))
            .field("emac", &format_args!("{}", self.emac().bit()))
            .field("otgfs2", &format_args!("{}", self.otgfs2().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<AHBRST1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioa(&mut self) -> GPIOA_W<AHBRST1_SPEC, 0> {
        GPIOA_W::new(self)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiob(&mut self) -> GPIOB_W<AHBRST1_SPEC, 1> {
        GPIOB_W::new(self)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioc(&mut self) -> GPIOC_W<AHBRST1_SPEC, 2> {
        GPIOC_W::new(self)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiod(&mut self) -> GPIOD_W<AHBRST1_SPEC, 3> {
        GPIOD_W::new(self)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioe(&mut self) -> GPIOE_W<AHBRST1_SPEC, 4> {
        GPIOE_W::new(self)
    }
    #[doc = "Bit 5 - IO port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiof(&mut self) -> GPIOF_W<AHBRST1_SPEC, 5> {
        GPIOF_W::new(self)
    }
    #[doc = "Bit 6 - IO port G reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiog(&mut self) -> GPIOG_W<AHBRST1_SPEC, 6> {
        GPIOG_W::new(self)
    }
    #[doc = "Bit 7 - IO port H reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioh(&mut self) -> GPIOH_W<AHBRST1_SPEC, 7> {
        GPIOH_W::new(self)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<AHBRST1_SPEC, 12> {
        CRC_W::new(self)
    }
    #[doc = "Bit 21 - EDMA reset"]
    #[inline(always)]
    #[must_use]
    pub fn edma(&mut self) -> EDMA_W<AHBRST1_SPEC, 21> {
        EDMA_W::new(self)
    }
    #[doc = "Bit 22 - DMA1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma1(&mut self) -> DMA1_W<AHBRST1_SPEC, 22> {
        DMA1_W::new(self)
    }
    #[doc = "Bit 24 - DMA2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2(&mut self) -> DMA2_W<AHBRST1_SPEC, 24> {
        DMA2_W::new(self)
    }
    #[doc = "Bit 25 - EMAC reset"]
    #[inline(always)]
    #[must_use]
    pub fn emac(&mut self) -> EMAC_W<AHBRST1_SPEC, 25> {
        EMAC_W::new(self)
    }
    #[doc = "Bit 29 - OTGFS2 interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs2(&mut self) -> OTGFS2_W<AHBRST1_SPEC, 29> {
        OTGFS2_W::new(self)
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
#[doc = "AHB peripheral reset register1 (CRM_AHBRST1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRST1_SPEC;
impl crate::RegisterSpec for AHBRST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst1::R`](R) reader structure"]
impl crate::Readable for AHBRST1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbrst1::W`](W) writer structure"]
impl crate::Writable for AHBRST1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBRST1 to value 0"]
impl crate::Resettable for AHBRST1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
