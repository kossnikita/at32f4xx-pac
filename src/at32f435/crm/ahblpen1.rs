#[doc = "Register `AHBLPEN1` reader"]
pub type R = crate::R<AHBLPEN1_SPEC>;
#[doc = "Register `AHBLPEN1` writer"]
pub type W = crate::W<AHBLPEN1_SPEC>;
#[doc = "Field `GPIOALP` reader - IO A clock enable during sleep mode"]
pub type GPIOALP_R = crate::BitReader;
#[doc = "Field `GPIOALP` writer - IO A clock enable during sleep mode"]
pub type GPIOALP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBLP` reader - IO B clock enable during sleep mode"]
pub type GPIOBLP_R = crate::BitReader;
#[doc = "Field `GPIOBLP` writer - IO B clock enable during sleep mode"]
pub type GPIOBLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCLP` reader - IO C clock enable during sleep mode"]
pub type GPIOCLP_R = crate::BitReader;
#[doc = "Field `GPIOCLP` writer - IO C clock enable during sleep mode"]
pub type GPIOCLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODLP` reader - IO D clock enable during sleep mode"]
pub type GPIODLP_R = crate::BitReader;
#[doc = "Field `GPIODLP` writer - IO D clock enable during sleep mode"]
pub type GPIODLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOELP` reader - IO E clock enable during sleep mode"]
pub type GPIOELP_R = crate::BitReader;
#[doc = "Field `GPIOELP` writer - IO E clock enable during sleep mode"]
pub type GPIOELP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFLP` reader - IO F clock enable during sleep mode"]
pub type GPIOFLP_R = crate::BitReader;
#[doc = "Field `GPIOFLP` writer - IO F clock enable during sleep mode"]
pub type GPIOFLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGLP` reader - IO G clock enable during sleep mode"]
pub type GPIOGLP_R = crate::BitReader;
#[doc = "Field `GPIOGLP` writer - IO G clock enable during sleep mode"]
pub type GPIOGLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHLP` reader - IO H clock enable during sleep mode"]
pub type GPIOHLP_R = crate::BitReader;
#[doc = "Field `GPIOHLP` writer - IO H clock enable during sleep mode"]
pub type GPIOHLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCLP` reader - CRC clock enable during sleep mode"]
pub type CRCLP_R = crate::BitReader;
#[doc = "Field `CRCLP` writer - CRC clock enable during sleep mode"]
pub type CRCLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHLP` reader - Flash clock enable during sleep mode"]
pub type FLASHLP_R = crate::BitReader;
#[doc = "Field `FLASHLP` writer - Flash clock enable during sleep mode"]
pub type FLASHLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1LP` reader - SRAM1 clock enable during sleep mode"]
pub type SRAM1LP_R = crate::BitReader;
#[doc = "Field `SRAM1LP` writer - SRAM1 clock enable during sleep mode"]
pub type SRAM1LP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2LP` reader - SRAM2 clock enable during sleep mode"]
pub type SRAM2LP_R = crate::BitReader;
#[doc = "Field `SRAM2LP` writer - SRAM2 clock enable during sleep mode"]
pub type SRAM2LP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDMALP` reader - EDMA clock enable during sleep mode"]
pub type EDMALP_R = crate::BitReader;
#[doc = "Field `EDMALP` writer - EDMA clock enable during sleep mode"]
pub type EDMALP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1LP` reader - DMA1 clock enable during sleep mode"]
pub type DMA1LP_R = crate::BitReader;
#[doc = "Field `DMA1LP` writer - DMA1 clock enable during sleep mode"]
pub type DMA1LP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2LP` reader - DMA2 clock enable during sleep mode"]
pub type DMA2LP_R = crate::BitReader;
#[doc = "Field `DMA2LP` writer - DMA2 clock enable during sleep mode"]
pub type DMA2LP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACLP` reader - EMAC clock enable during sleep mode"]
pub type EMACLP_R = crate::BitReader;
#[doc = "Field `EMACLP` writer - EMAC clock enable during sleep mode"]
pub type EMACLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACTXLP` reader - EMAC Tx clock enable during sleep mode"]
pub type EMACTXLP_R = crate::BitReader;
#[doc = "Field `EMACTXLP` writer - EMAC Tx clock enable during sleep mode"]
pub type EMACTXLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACRXLP` reader - EMAC Rx clock enable during sleep mode"]
pub type EMACRXLP_R = crate::BitReader;
#[doc = "Field `EMACRXLP` writer - EMAC Rx clock enable during sleep mode"]
pub type EMACRXLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACPTPLP` reader - EMAC PTP clock enable during sleep mode"]
pub type EMACPTPLP_R = crate::BitReader;
#[doc = "Field `EMACPTPLP` writer - EMAC PTP clock enable during sleep mode"]
pub type EMACPTPLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGFS2LP` reader - OTGFS2 clock enable during sleep mode"]
pub type OTGFS2LP_R = crate::BitReader;
#[doc = "Field `OTGFS2LP` writer - OTGFS2 clock enable during sleep mode"]
pub type OTGFS2LP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO A clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioalp(&self) -> GPIOALP_R {
        GPIOALP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO B clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioblp(&self) -> GPIOBLP_R {
        GPIOBLP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO C clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioclp(&self) -> GPIOCLP_R {
        GPIOCLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO D clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpiodlp(&self) -> GPIODLP_R {
        GPIODLP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO E clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioelp(&self) -> GPIOELP_R {
        GPIOELP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO F clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioflp(&self) -> GPIOFLP_R {
        GPIOFLP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO G clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioglp(&self) -> GPIOGLP_R {
        GPIOGLP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO H clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpiohlp(&self) -> GPIOHLP_R {
        GPIOHLP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode"]
    #[inline(always)]
    pub fn crclp(&self) -> CRCLP_R {
        CRCLP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Flash clock enable during sleep mode"]
    #[inline(always)]
    pub fn flashlp(&self) -> FLASHLP_R {
        FLASHLP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SRAM1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sram1lp(&self) -> SRAM1LP_R {
        SRAM1LP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SRAM2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sram2lp(&self) -> SRAM2LP_R {
        SRAM2LP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - EDMA clock enable during sleep mode"]
    #[inline(always)]
    pub fn edmalp(&self) -> EDMALP_R {
        EDMALP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn dma1lp(&self) -> DMA1LP_R {
        DMA1LP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn dma2lp(&self) -> DMA2LP_R {
        DMA2LP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EMAC clock enable during sleep mode"]
    #[inline(always)]
    pub fn emaclp(&self) -> EMACLP_R {
        EMACLP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EMAC Tx clock enable during sleep mode"]
    #[inline(always)]
    pub fn emactxlp(&self) -> EMACTXLP_R {
        EMACTXLP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - EMAC Rx clock enable during sleep mode"]
    #[inline(always)]
    pub fn emacrxlp(&self) -> EMACRXLP_R {
        EMACRXLP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - EMAC PTP clock enable during sleep mode"]
    #[inline(always)]
    pub fn emacptplp(&self) -> EMACPTPLP_R {
        EMACPTPLP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OTGFS2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn otgfs2lp(&self) -> OTGFS2LP_R {
        OTGFS2LP_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBLPEN1")
            .field("gpioalp", &self.gpioalp())
            .field("gpioblp", &self.gpioblp())
            .field("gpioclp", &self.gpioclp())
            .field("gpiodlp", &self.gpiodlp())
            .field("gpioelp", &self.gpioelp())
            .field("gpioflp", &self.gpioflp())
            .field("gpioglp", &self.gpioglp())
            .field("gpiohlp", &self.gpiohlp())
            .field("crclp", &self.crclp())
            .field("flashlp", &self.flashlp())
            .field("sram1lp", &self.sram1lp())
            .field("sram2lp", &self.sram2lp())
            .field("edmalp", &self.edmalp())
            .field("dma1lp", &self.dma1lp())
            .field("dma2lp", &self.dma2lp())
            .field("emaclp", &self.emaclp())
            .field("emactxlp", &self.emactxlp())
            .field("emacrxlp", &self.emacrxlp())
            .field("emacptplp", &self.emacptplp())
            .field("otgfs2lp", &self.otgfs2lp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - IO A clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioalp(&mut self) -> GPIOALP_W<AHBLPEN1_SPEC> {
        GPIOALP_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO B clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioblp(&mut self) -> GPIOBLP_W<AHBLPEN1_SPEC> {
        GPIOBLP_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO C clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioclp(&mut self) -> GPIOCLP_W<AHBLPEN1_SPEC> {
        GPIOCLP_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO D clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodlp(&mut self) -> GPIODLP_W<AHBLPEN1_SPEC> {
        GPIODLP_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO E clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioelp(&mut self) -> GPIOELP_W<AHBLPEN1_SPEC> {
        GPIOELP_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO F clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioflp(&mut self) -> GPIOFLP_W<AHBLPEN1_SPEC> {
        GPIOFLP_W::new(self, 5)
    }
    #[doc = "Bit 6 - IO G clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioglp(&mut self) -> GPIOGLP_W<AHBLPEN1_SPEC> {
        GPIOGLP_W::new(self, 6)
    }
    #[doc = "Bit 7 - IO H clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohlp(&mut self) -> GPIOHLP_W<AHBLPEN1_SPEC> {
        GPIOHLP_W::new(self, 7)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn crclp(&mut self) -> CRCLP_W<AHBLPEN1_SPEC> {
        CRCLP_W::new(self, 12)
    }
    #[doc = "Bit 15 - Flash clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn flashlp(&mut self) -> FLASHLP_W<AHBLPEN1_SPEC> {
        FLASHLP_W::new(self, 15)
    }
    #[doc = "Bit 16 - SRAM1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram1lp(&mut self) -> SRAM1LP_W<AHBLPEN1_SPEC> {
        SRAM1LP_W::new(self, 16)
    }
    #[doc = "Bit 17 - SRAM2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram2lp(&mut self) -> SRAM2LP_W<AHBLPEN1_SPEC> {
        SRAM2LP_W::new(self, 17)
    }
    #[doc = "Bit 21 - EDMA clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn edmalp(&mut self) -> EDMALP_W<AHBLPEN1_SPEC> {
        EDMALP_W::new(self, 21)
    }
    #[doc = "Bit 22 - DMA1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma1lp(&mut self) -> DMA1LP_W<AHBLPEN1_SPEC> {
        DMA1LP_W::new(self, 22)
    }
    #[doc = "Bit 24 - DMA2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma2lp(&mut self) -> DMA2LP_W<AHBLPEN1_SPEC> {
        DMA2LP_W::new(self, 24)
    }
    #[doc = "Bit 25 - EMAC clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn emaclp(&mut self) -> EMACLP_W<AHBLPEN1_SPEC> {
        EMACLP_W::new(self, 25)
    }
    #[doc = "Bit 26 - EMAC Tx clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn emactxlp(&mut self) -> EMACTXLP_W<AHBLPEN1_SPEC> {
        EMACTXLP_W::new(self, 26)
    }
    #[doc = "Bit 27 - EMAC Rx clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn emacrxlp(&mut self) -> EMACRXLP_W<AHBLPEN1_SPEC> {
        EMACRXLP_W::new(self, 27)
    }
    #[doc = "Bit 28 - EMAC PTP clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn emacptplp(&mut self) -> EMACPTPLP_W<AHBLPEN1_SPEC> {
        EMACPTPLP_W::new(self, 28)
    }
    #[doc = "Bit 29 - OTGFS2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs2lp(&mut self) -> OTGFS2LP_W<AHBLPEN1_SPEC> {
        OTGFS2LP_W::new(self, 29)
    }
}
#[doc = "AHB Low-power Peripheral Clock enable register 1 (CRM_AHBLPEN1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahblpen1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahblpen1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBLPEN1_SPEC;
impl crate::RegisterSpec for AHBLPEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahblpen1::R`](R) reader structure"]
impl crate::Readable for AHBLPEN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahblpen1::W`](W) writer structure"]
impl crate::Writable for AHBLPEN1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBLPEN1 to value 0x3e63_90ff"]
impl crate::Resettable for AHBLPEN1_SPEC {
    const RESET_VALUE: u32 = 0x3e63_90ff;
}
