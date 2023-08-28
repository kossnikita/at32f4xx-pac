#[doc = "Register `AHBLPEN1` reader"]
pub type R = crate::R<AHBLPEN1_SPEC>;
#[doc = "Register `AHBLPEN1` writer"]
pub type W = crate::W<AHBLPEN1_SPEC>;
#[doc = "Field `GPIOALPEN` reader - IO A clock enable during sleep mode"]
pub type GPIOALPEN_R = crate::BitReader;
#[doc = "Field `GPIOALPEN` writer - IO A clock enable during sleep mode"]
pub type GPIOALPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOBLPEN` reader - IO B clock enable during sleep mode"]
pub type GPIOBLPEN_R = crate::BitReader;
#[doc = "Field `GPIOBLPEN` writer - IO B clock enable during sleep mode"]
pub type GPIOBLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOCLPEN` reader - IO C clock enable during sleep mode"]
pub type GPIOCLPEN_R = crate::BitReader;
#[doc = "Field `GPIOCLPEN` writer - IO C clock enable during sleep mode"]
pub type GPIOCLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIODLPEN` reader - IO D clock enable during sleep mode"]
pub type GPIODLPEN_R = crate::BitReader;
#[doc = "Field `GPIODLPEN` writer - IO D clock enable during sleep mode"]
pub type GPIODLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOELPEN` reader - IO E clock enable during sleep mode"]
pub type GPIOELPEN_R = crate::BitReader;
#[doc = "Field `GPIOELPEN` writer - IO E clock enable during sleep mode"]
pub type GPIOELPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOFLPEN` reader - IO F clock enable during sleep mode"]
pub type GPIOFLPEN_R = crate::BitReader;
#[doc = "Field `GPIOFLPEN` writer - IO F clock enable during sleep mode"]
pub type GPIOFLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOGLPEN` reader - IO G clock enable during sleep mode"]
pub type GPIOGLPEN_R = crate::BitReader;
#[doc = "Field `GPIOGLPEN` writer - IO G clock enable during sleep mode"]
pub type GPIOGLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOHLPEN` reader - IO H clock enable during sleep mode"]
pub type GPIOHLPEN_R = crate::BitReader;
#[doc = "Field `GPIOHLPEN` writer - IO H clock enable during sleep mode"]
pub type GPIOHLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCLPEN` reader - CRC clock enable during sleep mode"]
pub type CRCLPEN_R = crate::BitReader;
#[doc = "Field `CRCLPEN` writer - CRC clock enable during sleep mode"]
pub type CRCLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASHLPEN` reader - Flash clock enable during sleep mode"]
pub type FLASHLPEN_R = crate::BitReader;
#[doc = "Field `FLASHLPEN` writer - Flash clock enable during sleep mode"]
pub type FLASHLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRAM1LPEN` reader - SRAM1 clock enable during sleep mode"]
pub type SRAM1LPEN_R = crate::BitReader;
#[doc = "Field `SRAM1LPEN` writer - SRAM1 clock enable during sleep mode"]
pub type SRAM1LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRAM2LPEN` reader - SRAM2 clock enable during sleep mode"]
pub type SRAM2LPEN_R = crate::BitReader;
#[doc = "Field `SRAM2LPEN` writer - SRAM2 clock enable during sleep mode"]
pub type SRAM2LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EDMALPEN` reader - EDMA clock enable during sleep mode"]
pub type EDMALPEN_R = crate::BitReader;
#[doc = "Field `EDMALPEN` writer - EDMA clock enable during sleep mode"]
pub type EDMALPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA1LPEN` reader - DMA1 clock enable during sleep mode"]
pub type DMA1LPEN_R = crate::BitReader;
#[doc = "Field `DMA1LPEN` writer - DMA1 clock enable during sleep mode"]
pub type DMA1LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA2LPEN` reader - DMA2 clock enable during sleep mode"]
pub type DMA2LPEN_R = crate::BitReader;
#[doc = "Field `DMA2LPEN` writer - DMA2 clock enable during sleep mode"]
pub type DMA2LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMACLPEN` reader - EMAC clock enable during sleep mode"]
pub type EMACLPEN_R = crate::BitReader;
#[doc = "Field `EMACLPEN` writer - EMAC clock enable during sleep mode"]
pub type EMACLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMACTXLPEN` reader - EMAC Tx clock enable during sleep mode"]
pub type EMACTXLPEN_R = crate::BitReader;
#[doc = "Field `EMACTXLPEN` writer - EMAC Tx clock enable during sleep mode"]
pub type EMACTXLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMACRXLPEN` reader - EMAC Rx clock enable during sleep mode"]
pub type EMACRXLPEN_R = crate::BitReader;
#[doc = "Field `EMACRXLPEN` writer - EMAC Rx clock enable during sleep mode"]
pub type EMACRXLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMACPTPLPEN` reader - EMAC PTP clock enable during sleep mode"]
pub type EMACPTPLPEN_R = crate::BitReader;
#[doc = "Field `EMACPTPLPEN` writer - EMAC PTP clock enable during sleep mode"]
pub type EMACPTPLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OTGFS2LPEN` reader - OTGFS2 clock enable during sleep mode"]
pub type OTGFS2LPEN_R = crate::BitReader;
#[doc = "Field `OTGFS2LPEN` writer - OTGFS2 clock enable during sleep mode"]
pub type OTGFS2LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - IO A clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO B clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO C clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO D clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO E clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO F clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO G clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO H clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode"]
    #[inline(always)]
    pub fn crclpen(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Flash clock enable during sleep mode"]
    #[inline(always)]
    pub fn flashlpen(&self) -> FLASHLPEN_R {
        FLASHLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SRAM1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SRAM2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - EDMA clock enable during sleep mode"]
    #[inline(always)]
    pub fn edmalpen(&self) -> EDMALPEN_R {
        EDMALPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EMAC clock enable during sleep mode"]
    #[inline(always)]
    pub fn emaclpen(&self) -> EMACLPEN_R {
        EMACLPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EMAC Tx clock enable during sleep mode"]
    #[inline(always)]
    pub fn emactxlpen(&self) -> EMACTXLPEN_R {
        EMACTXLPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - EMAC Rx clock enable during sleep mode"]
    #[inline(always)]
    pub fn emacrxlpen(&self) -> EMACRXLPEN_R {
        EMACRXLPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - EMAC PTP clock enable during sleep mode"]
    #[inline(always)]
    pub fn emacptplpen(&self) -> EMACPTPLPEN_R {
        EMACPTPLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OTGFS2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn otgfs2lpen(&self) -> OTGFS2LPEN_R {
        OTGFS2LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO A clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<AHBLPEN1_SPEC, 0> {
        GPIOALPEN_W::new(self)
    }
    #[doc = "Bit 1 - IO B clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<AHBLPEN1_SPEC, 1> {
        GPIOBLPEN_W::new(self)
    }
    #[doc = "Bit 2 - IO C clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<AHBLPEN1_SPEC, 2> {
        GPIOCLPEN_W::new(self)
    }
    #[doc = "Bit 3 - IO D clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<AHBLPEN1_SPEC, 3> {
        GPIODLPEN_W::new(self)
    }
    #[doc = "Bit 4 - IO E clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<AHBLPEN1_SPEC, 4> {
        GPIOELPEN_W::new(self)
    }
    #[doc = "Bit 5 - IO F clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<AHBLPEN1_SPEC, 5> {
        GPIOFLPEN_W::new(self)
    }
    #[doc = "Bit 6 - IO G clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W<AHBLPEN1_SPEC, 6> {
        GPIOGLPEN_W::new(self)
    }
    #[doc = "Bit 7 - IO H clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<AHBLPEN1_SPEC, 7> {
        GPIOHLPEN_W::new(self)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn crclpen(&mut self) -> CRCLPEN_W<AHBLPEN1_SPEC, 12> {
        CRCLPEN_W::new(self)
    }
    #[doc = "Bit 15 - Flash clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn flashlpen(&mut self) -> FLASHLPEN_W<AHBLPEN1_SPEC, 15> {
        FLASHLPEN_W::new(self)
    }
    #[doc = "Bit 16 - SRAM1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W<AHBLPEN1_SPEC, 16> {
        SRAM1LPEN_W::new(self)
    }
    #[doc = "Bit 17 - SRAM2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W<AHBLPEN1_SPEC, 17> {
        SRAM2LPEN_W::new(self)
    }
    #[doc = "Bit 21 - EDMA clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn edmalpen(&mut self) -> EDMALPEN_W<AHBLPEN1_SPEC, 21> {
        EDMALPEN_W::new(self)
    }
    #[doc = "Bit 22 - DMA1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W<AHBLPEN1_SPEC, 22> {
        DMA1LPEN_W::new(self)
    }
    #[doc = "Bit 24 - DMA2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W<AHBLPEN1_SPEC, 24> {
        DMA2LPEN_W::new(self)
    }
    #[doc = "Bit 25 - EMAC clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn emaclpen(&mut self) -> EMACLPEN_W<AHBLPEN1_SPEC, 25> {
        EMACLPEN_W::new(self)
    }
    #[doc = "Bit 26 - EMAC Tx clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn emactxlpen(&mut self) -> EMACTXLPEN_W<AHBLPEN1_SPEC, 26> {
        EMACTXLPEN_W::new(self)
    }
    #[doc = "Bit 27 - EMAC Rx clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn emacrxlpen(&mut self) -> EMACRXLPEN_W<AHBLPEN1_SPEC, 27> {
        EMACRXLPEN_W::new(self)
    }
    #[doc = "Bit 28 - EMAC PTP clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn emacptplpen(&mut self) -> EMACPTPLPEN_W<AHBLPEN1_SPEC, 28> {
        EMACPTPLPEN_W::new(self)
    }
    #[doc = "Bit 29 - OTGFS2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs2lpen(&mut self) -> OTGFS2LPEN_W<AHBLPEN1_SPEC, 29> {
        OTGFS2LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AHB Low-power Peripheral Clock enable register 1 (CRM_AHBLPEN1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblpen1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblpen1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBLPEN1_SPEC;
impl crate::RegisterSpec for AHBLPEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahblpen1::R`](R) reader structure"]
impl crate::Readable for AHBLPEN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahblpen1::W`](W) writer structure"]
impl crate::Writable for AHBLPEN1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBLPEN1 to value 0x3e63_90ff"]
impl crate::Resettable for AHBLPEN1_SPEC {
    const RESET_VALUE: Self::Ux = 0x3e63_90ff;
}
