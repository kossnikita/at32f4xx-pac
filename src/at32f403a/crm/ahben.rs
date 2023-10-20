#[doc = "Register `AHBEN` reader"]
pub type R = crate::R<AHBEN_SPEC>;
#[doc = "Register `AHBEN` writer"]
pub type W = crate::W<AHBEN_SPEC>;
#[doc = "Field `DMA1` reader - DMA1 clock enable"]
pub type DMA1_R = crate::BitReader;
#[doc = "Field `DMA1` writer - DMA1 clock enable"]
pub type DMA1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2` reader - DMA2 clock enable"]
pub type DMA2_R = crate::BitReader;
#[doc = "Field `DMA2` writer - DMA2 clock enable"]
pub type DMA2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM` reader - SRAM interface clock enable"]
pub type SRAM_R = crate::BitReader;
#[doc = "Field `SRAM` writer - SRAM interface clock enable"]
pub type SRAM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH` reader - FLASH clock enable"]
pub type FLASH_R = crate::BitReader;
#[doc = "Field `FLASH` writer - FLASH clock enable"]
pub type FLASH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - CRC clock enable"]
pub type CRC_R = crate::BitReader;
#[doc = "Field `CRC` writer - CRC clock enable"]
pub type CRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XMC` reader - XMC clock enable"]
pub type XMC_R = crate::BitReader;
#[doc = "Field `XMC` writer - XMC clock enable"]
pub type XMC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO1` reader - SDIO1 clock enable"]
pub type SDIO1_R = crate::BitReader;
#[doc = "Field `SDIO1` writer - SDIO1 clock enable"]
pub type SDIO1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO2` reader - SDIO2 clock enable"]
pub type SDIO2_R = crate::BitReader;
#[doc = "Field `SDIO2` writer - SDIO2 clock enable"]
pub type SDIO2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC` reader - EMACEN clock enable"]
pub type EMAC_R = crate::BitReader;
#[doc = "Field `EMAC` writer - EMACEN clock enable"]
pub type EMAC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACTX` reader - EMACEN Tx clock enable"]
pub type EMACTX_R = crate::BitReader;
#[doc = "Field `EMACTX` writer - EMACEN Tx clock enable"]
pub type EMACTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACRX` reader - EMACEN Rx clock enable"]
pub type EMACRX_R = crate::BitReader;
#[doc = "Field `EMACRX` writer - EMACEN Rx clock enable"]
pub type EMACRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACPTP` reader - EMACPTP clock enable"]
pub type EMACPTP_R = crate::BitReader;
#[doc = "Field `EMACPTP` writer - EMACPTP clock enable"]
pub type EMACPTP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1(&self) -> DMA1_R {
        DMA1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2(&self) -> DMA2_R {
        DMA2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sram(&self) -> SRAM_R {
        SRAM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FLASH clock enable"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - XMC clock enable"]
    #[inline(always)]
    pub fn xmc(&self) -> XMC_R {
        XMC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - SDIO1 clock enable"]
    #[inline(always)]
    pub fn sdio1(&self) -> SDIO1_R {
        SDIO1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIO2 clock enable"]
    #[inline(always)]
    pub fn sdio2(&self) -> SDIO2_R {
        SDIO2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - EMACEN clock enable"]
    #[inline(always)]
    pub fn emac(&self) -> EMAC_R {
        EMAC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EMACEN Tx clock enable"]
    #[inline(always)]
    pub fn emactx(&self) -> EMACTX_R {
        EMACTX_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - EMACEN Rx clock enable"]
    #[inline(always)]
    pub fn emacrx(&self) -> EMACRX_R {
        EMACRX_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - EMACPTP clock enable"]
    #[inline(always)]
    pub fn emacptp(&self) -> EMACPTP_R {
        EMACPTP_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBEN")
            .field("dma1", &format_args!("{}", self.dma1().bit()))
            .field("dma2", &format_args!("{}", self.dma2().bit()))
            .field("sram", &format_args!("{}", self.sram().bit()))
            .field("flash", &format_args!("{}", self.flash().bit()))
            .field("crc", &format_args!("{}", self.crc().bit()))
            .field("xmc", &format_args!("{}", self.xmc().bit()))
            .field("sdio1", &format_args!("{}", self.sdio1().bit()))
            .field("sdio2", &format_args!("{}", self.sdio2().bit()))
            .field("emac", &format_args!("{}", self.emac().bit()))
            .field("emactx", &format_args!("{}", self.emactx().bit()))
            .field("emacrx", &format_args!("{}", self.emacrx().bit()))
            .field("emacptp", &format_args!("{}", self.emacptp().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<AHBEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1(&mut self) -> DMA1_W<AHBEN_SPEC> {
        DMA1_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2(&mut self) -> DMA2_W<AHBEN_SPEC> {
        DMA2_W::new(self, 1)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram(&mut self) -> SRAM_W<AHBEN_SPEC> {
        SRAM_W::new(self, 2)
    }
    #[doc = "Bit 4 - FLASH clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn flash(&mut self) -> FLASH_W<AHBEN_SPEC> {
        FLASH_W::new(self, 4)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<AHBEN_SPEC> {
        CRC_W::new(self, 6)
    }
    #[doc = "Bit 8 - XMC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn xmc(&mut self) -> XMC_W<AHBEN_SPEC> {
        XMC_W::new(self, 8)
    }
    #[doc = "Bit 10 - SDIO1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1(&mut self) -> SDIO1_W<AHBEN_SPEC> {
        SDIO1_W::new(self, 10)
    }
    #[doc = "Bit 11 - SDIO2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdio2(&mut self) -> SDIO2_W<AHBEN_SPEC> {
        SDIO2_W::new(self, 11)
    }
    #[doc = "Bit 14 - EMACEN clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emac(&mut self) -> EMAC_W<AHBEN_SPEC> {
        EMAC_W::new(self, 14)
    }
    #[doc = "Bit 15 - EMACEN Tx clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emactx(&mut self) -> EMACTX_W<AHBEN_SPEC> {
        EMACTX_W::new(self, 15)
    }
    #[doc = "Bit 16 - EMACEN Rx clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emacrx(&mut self) -> EMACRX_W<AHBEN_SPEC> {
        EMACRX_W::new(self, 16)
    }
    #[doc = "Bit 28 - EMACPTP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emacptp(&mut self) -> EMACPTP_W<AHBEN_SPEC> {
        EMACPTP_W::new(self, 28)
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
#[doc = "AHB Peripheral Clock enable register (CRM_AHBEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBEN_SPEC;
impl crate::RegisterSpec for AHBEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben::R`](R) reader structure"]
impl crate::Readable for AHBEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahben::W`](W) writer structure"]
impl crate::Writable for AHBEN_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBEN to value 0x14"]
impl crate::Resettable for AHBEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x14;
}
