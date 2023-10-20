#[doc = "Register `AHBEN1` reader"]
pub type R = crate::R<AHBEN1_SPEC>;
#[doc = "Register `AHBEN1` writer"]
pub type W = crate::W<AHBEN1_SPEC>;
#[doc = "Field `GPIOA` reader - IO A clock enable"]
pub type GPIOA_R = crate::BitReader;
#[doc = "Field `GPIOA` writer - IO A clock enable"]
pub type GPIOA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOB` reader - IO B clock enable"]
pub type GPIOB_R = crate::BitReader;
#[doc = "Field `GPIOB` writer - IO B clock enable"]
pub type GPIOB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOC` reader - IO C clock enable"]
pub type GPIOC_R = crate::BitReader;
#[doc = "Field `GPIOC` writer - IO C clock enable"]
pub type GPIOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOD` reader - IO D clock enable"]
pub type GPIOD_R = crate::BitReader;
#[doc = "Field `GPIOD` writer - IO D clock enable"]
pub type GPIOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOE` reader - IO E clock enable"]
pub type GPIOE_R = crate::BitReader;
#[doc = "Field `GPIOE` writer - IO E clock enable"]
pub type GPIOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOF` reader - IO F clock enable"]
pub type GPIOF_R = crate::BitReader;
#[doc = "Field `GPIOF` writer - IO F clock enable"]
pub type GPIOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOG` reader - IO G clock enable"]
pub type GPIOG_R = crate::BitReader;
#[doc = "Field `GPIOG` writer - IO G clock enable"]
pub type GPIOG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOH` reader - IO H clock enable"]
pub type GPIOH_R = crate::BitReader;
#[doc = "Field `GPIOH` writer - IO H clock enable"]
pub type GPIOH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - CRC clock enable"]
pub type CRC_R = crate::BitReader;
#[doc = "Field `CRC` writer - CRC clock enable"]
pub type CRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDMA` reader - DMA1 clock enable"]
pub type EDMA_R = crate::BitReader;
#[doc = "Field `EDMA` writer - DMA1 clock enable"]
pub type EDMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1` reader - DMA1 clock enable"]
pub type DMA1_R = crate::BitReader;
#[doc = "Field `DMA1` writer - DMA1 clock enable"]
pub type DMA1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2` reader - DMA2 clock enable"]
pub type DMA2_R = crate::BitReader;
#[doc = "Field `DMA2` writer - DMA2 clock enable"]
pub type DMA2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC` reader - EMAC clock enable"]
pub type EMAC_R = crate::BitReader;
#[doc = "Field `EMAC` writer - EMAC clock enable"]
pub type EMAC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACTX` reader - EMAC Tx clock enable"]
pub type EMACTX_R = crate::BitReader;
#[doc = "Field `EMACTX` writer - EMAC Tx clock enable"]
pub type EMACTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACRX` reader - EMAC Rx clock enable"]
pub type EMACRX_R = crate::BitReader;
#[doc = "Field `EMACRX` writer - EMAC Rx clock enable"]
pub type EMACRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACPTP` reader - EMAC PTP clock enable"]
pub type EMACPTP_R = crate::BitReader;
#[doc = "Field `EMACPTP` writer - EMAC PTP clock enable"]
pub type EMACPTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGFS2` reader - OTGFS2 clock enable"]
pub type OTGFS2_R = crate::BitReader;
#[doc = "Field `OTGFS2` writer - OTGFS2 clock enable"]
pub type OTGFS2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO A clock enable"]
    #[inline(always)]
    pub fn gpioa(&self) -> GPIOA_R {
        GPIOA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO B clock enable"]
    #[inline(always)]
    pub fn gpiob(&self) -> GPIOB_R {
        GPIOB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO C clock enable"]
    #[inline(always)]
    pub fn gpioc(&self) -> GPIOC_R {
        GPIOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO D clock enable"]
    #[inline(always)]
    pub fn gpiod(&self) -> GPIOD_R {
        GPIOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO E clock enable"]
    #[inline(always)]
    pub fn gpioe(&self) -> GPIOE_R {
        GPIOE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO F clock enable"]
    #[inline(always)]
    pub fn gpiof(&self) -> GPIOF_R {
        GPIOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO G clock enable"]
    #[inline(always)]
    pub fn gpiog(&self) -> GPIOG_R {
        GPIOG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO H clock enable"]
    #[inline(always)]
    pub fn gpioh(&self) -> GPIOH_R {
        GPIOH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA1 clock enable"]
    #[inline(always)]
    pub fn edma(&self) -> EDMA_R {
        EDMA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1(&self) -> DMA1_R {
        DMA1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2(&self) -> DMA2_R {
        DMA2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EMAC clock enable"]
    #[inline(always)]
    pub fn emac(&self) -> EMAC_R {
        EMAC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EMAC Tx clock enable"]
    #[inline(always)]
    pub fn emactx(&self) -> EMACTX_R {
        EMACTX_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - EMAC Rx clock enable"]
    #[inline(always)]
    pub fn emacrx(&self) -> EMACRX_R {
        EMACRX_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - EMAC PTP clock enable"]
    #[inline(always)]
    pub fn emacptp(&self) -> EMACPTP_R {
        EMACPTP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OTGFS2 clock enable"]
    #[inline(always)]
    pub fn otgfs2(&self) -> OTGFS2_R {
        OTGFS2_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBEN1")
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
            .field("emactx", &format_args!("{}", self.emactx().bit()))
            .field("emacrx", &format_args!("{}", self.emacrx().bit()))
            .field("emacptp", &format_args!("{}", self.emacptp().bit()))
            .field("otgfs2", &format_args!("{}", self.otgfs2().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<AHBEN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - IO A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioa(&mut self) -> GPIOA_W<AHBEN1_SPEC> {
        GPIOA_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiob(&mut self) -> GPIOB_W<AHBEN1_SPEC> {
        GPIOB_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioc(&mut self) -> GPIOC_W<AHBEN1_SPEC> {
        GPIOC_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiod(&mut self) -> GPIOD_W<AHBEN1_SPEC> {
        GPIOD_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioe(&mut self) -> GPIOE_W<AHBEN1_SPEC> {
        GPIOE_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiof(&mut self) -> GPIOF_W<AHBEN1_SPEC> {
        GPIOF_W::new(self, 5)
    }
    #[doc = "Bit 6 - IO G clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiog(&mut self) -> GPIOG_W<AHBEN1_SPEC> {
        GPIOG_W::new(self, 6)
    }
    #[doc = "Bit 7 - IO H clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioh(&mut self) -> GPIOH_W<AHBEN1_SPEC> {
        GPIOH_W::new(self, 7)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<AHBEN1_SPEC> {
        CRC_W::new(self, 12)
    }
    #[doc = "Bit 21 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn edma(&mut self) -> EDMA_W<AHBEN1_SPEC> {
        EDMA_W::new(self, 21)
    }
    #[doc = "Bit 22 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1(&mut self) -> DMA1_W<AHBEN1_SPEC> {
        DMA1_W::new(self, 22)
    }
    #[doc = "Bit 24 - DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2(&mut self) -> DMA2_W<AHBEN1_SPEC> {
        DMA2_W::new(self, 24)
    }
    #[doc = "Bit 25 - EMAC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emac(&mut self) -> EMAC_W<AHBEN1_SPEC> {
        EMAC_W::new(self, 25)
    }
    #[doc = "Bit 26 - EMAC Tx clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emactx(&mut self) -> EMACTX_W<AHBEN1_SPEC> {
        EMACTX_W::new(self, 26)
    }
    #[doc = "Bit 27 - EMAC Rx clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emacrx(&mut self) -> EMACRX_W<AHBEN1_SPEC> {
        EMACRX_W::new(self, 27)
    }
    #[doc = "Bit 28 - EMAC PTP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emacptp(&mut self) -> EMACPTP_W<AHBEN1_SPEC> {
        EMACPTP_W::new(self, 28)
    }
    #[doc = "Bit 29 - OTGFS2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs2(&mut self) -> OTGFS2_W<AHBEN1_SPEC> {
        OTGFS2_W::new(self, 29)
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
#[doc = "AHB Peripheral Clock enable register 1 (CRM_AHBEN1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBEN1_SPEC;
impl crate::RegisterSpec for AHBEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben1::R`](R) reader structure"]
impl crate::Readable for AHBEN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahben1::W`](W) writer structure"]
impl crate::Writable for AHBEN1_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBEN1 to value 0"]
impl crate::Resettable for AHBEN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
