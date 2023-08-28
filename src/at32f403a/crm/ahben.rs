#[doc = "Register `AHBEN` reader"]
pub type R = crate::R<AHBEN_SPEC>;
#[doc = "Register `AHBEN` writer"]
pub type W = crate::W<AHBEN_SPEC>;
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub type DMA1EN_R = crate::BitReader;
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub type DMA1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub type DMA2EN_R = crate::BitReader;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub type DMA2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRAMEN` reader - SRAM interface clock enable"]
pub type SRAMEN_R = crate::BitReader;
#[doc = "Field `SRAMEN` writer - SRAM interface clock enable"]
pub type SRAMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASHEN` reader - FLASH clock enable"]
pub type FLASHEN_R = crate::BitReader;
#[doc = "Field `FLASHEN` writer - FLASH clock enable"]
pub type FLASHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CRCEN_R = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CRCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XMCEN` reader - XMC clock enable"]
pub type XMCEN_R = crate::BitReader;
#[doc = "Field `XMCEN` writer - XMC clock enable"]
pub type XMCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIO1EN` reader - SDIO1 clock enable"]
pub type SDIO1EN_R = crate::BitReader;
#[doc = "Field `SDIO1EN` writer - SDIO1 clock enable"]
pub type SDIO1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIO2EN` reader - SDIO2 clock enable"]
pub type SDIO2EN_R = crate::BitReader;
#[doc = "Field `SDIO2EN` writer - SDIO2 clock enable"]
pub type SDIO2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMACEN` reader - EMACEN clock enable"]
pub type EMACEN_R = crate::BitReader;
#[doc = "Field `EMACEN` writer - EMACEN clock enable"]
pub type EMACEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMACTXEN` reader - EMACEN Tx clock enable"]
pub type EMACTXEN_R = crate::BitReader;
#[doc = "Field `EMACTXEN` writer - EMACEN Tx clock enable"]
pub type EMACTXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMACRXEN` reader - EMACEN Rx clock enable"]
pub type EMACRXEN_R = crate::BitReader;
#[doc = "Field `EMACRXEN` writer - EMACEN Rx clock enable"]
pub type EMACRXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMACPTPEN` reader - EMACPTP clock enable"]
pub type EMACPTPEN_R = crate::BitReader;
#[doc = "Field `EMACPTPEN` writer - EMACPTP clock enable"]
pub type EMACPTPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FLASH clock enable"]
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - XMC clock enable"]
    #[inline(always)]
    pub fn xmcen(&self) -> XMCEN_R {
        XMCEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - SDIO1 clock enable"]
    #[inline(always)]
    pub fn sdio1en(&self) -> SDIO1EN_R {
        SDIO1EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIO2 clock enable"]
    #[inline(always)]
    pub fn sdio2en(&self) -> SDIO2EN_R {
        SDIO2EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - EMACEN clock enable"]
    #[inline(always)]
    pub fn emacen(&self) -> EMACEN_R {
        EMACEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EMACEN Tx clock enable"]
    #[inline(always)]
    pub fn emactxen(&self) -> EMACTXEN_R {
        EMACTXEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - EMACEN Rx clock enable"]
    #[inline(always)]
    pub fn emacrxen(&self) -> EMACRXEN_R {
        EMACRXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - EMACPTP clock enable"]
    #[inline(always)]
    pub fn emacptpen(&self) -> EMACPTPEN_R {
        EMACPTPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<AHBEN_SPEC, 0> {
        DMA1EN_W::new(self)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<AHBEN_SPEC, 1> {
        DMA2EN_W::new(self)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sramen(&mut self) -> SRAMEN_W<AHBEN_SPEC, 2> {
        SRAMEN_W::new(self)
    }
    #[doc = "Bit 4 - FLASH clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn flashen(&mut self) -> FLASHEN_W<AHBEN_SPEC, 4> {
        FLASHEN_W::new(self)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<AHBEN_SPEC, 6> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 8 - XMC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn xmcen(&mut self) -> XMCEN_W<AHBEN_SPEC, 8> {
        XMCEN_W::new(self)
    }
    #[doc = "Bit 10 - SDIO1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1en(&mut self) -> SDIO1EN_W<AHBEN_SPEC, 10> {
        SDIO1EN_W::new(self)
    }
    #[doc = "Bit 11 - SDIO2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdio2en(&mut self) -> SDIO2EN_W<AHBEN_SPEC, 11> {
        SDIO2EN_W::new(self)
    }
    #[doc = "Bit 14 - EMACEN clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emacen(&mut self) -> EMACEN_W<AHBEN_SPEC, 14> {
        EMACEN_W::new(self)
    }
    #[doc = "Bit 15 - EMACEN Tx clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emactxen(&mut self) -> EMACTXEN_W<AHBEN_SPEC, 15> {
        EMACTXEN_W::new(self)
    }
    #[doc = "Bit 16 - EMACEN Rx clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emacrxen(&mut self) -> EMACRXEN_W<AHBEN_SPEC, 16> {
        EMACRXEN_W::new(self)
    }
    #[doc = "Bit 28 - EMACPTP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emacptpen(&mut self) -> EMACPTPEN_W<AHBEN_SPEC, 28> {
        EMACPTPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBEN to value 0x14"]
impl crate::Resettable for AHBEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x14;
}
