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
#[doc = "Field `GPIOFLPEN` reader - IO F clock enable during sleep mode"]
pub type GPIOFLPEN_R = crate::BitReader;
#[doc = "Field `GPIOFLPEN` writer - IO F clock enable during sleep mode"]
pub type GPIOFLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCLPEN` reader - CRC clock enable during sleep mode"]
pub type CRCLPEN_R = crate::BitReader;
#[doc = "Field `CRCLPEN` writer - CRC clock enable during sleep mode"]
pub type CRCLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASHLPEN` reader - Flash clock enable during sleep mode"]
pub type FLASHLPEN_R = crate::BitReader;
#[doc = "Field `FLASHLPEN` writer - Flash clock enable during sleep mode"]
pub type FLASHLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRAMLPEN` reader - SRAM clock enable during sleep mode"]
pub type SRAMLPEN_R = crate::BitReader;
#[doc = "Field `SRAMLPEN` writer - SRAM clock enable during sleep mode"]
pub type SRAMLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA1LPEN` reader - DMA1 clock enable during sleep mode"]
pub type DMA1LPEN_R = crate::BitReader;
#[doc = "Field `DMA1LPEN` writer - DMA1 clock enable during sleep mode"]
pub type DMA1LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA2LPEN` reader - DMA2 clock enable during sleep mode"]
pub type DMA2LPEN_R = crate::BitReader;
#[doc = "Field `DMA2LPEN` writer - DMA2 clock enable during sleep mode"]
pub type DMA2LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OTGHSLPEN` reader - OTGHS clock enable during sleep mode"]
pub type OTGHSLPEN_R = crate::BitReader;
#[doc = "Field `OTGHSLPEN` writer - OTGHS clock enable during sleep mode"]
pub type OTGHSLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 5 - IO F clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bit 16 - SRAM clock enable during sleep mode"]
    #[inline(always)]
    pub fn sramlpen(&self) -> SRAMLPEN_R {
        SRAMLPEN_R::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bit 29 - OTGHS clock enable during sleep mode"]
    #[inline(always)]
    pub fn otghslpen(&self) -> OTGHSLPEN_R {
        OTGHSLPEN_R::new(((self.bits >> 29) & 1) != 0)
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
    #[doc = "Bit 5 - IO F clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<AHBLPEN1_SPEC, 5> {
        GPIOFLPEN_W::new(self)
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
    #[doc = "Bit 16 - SRAM clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sramlpen(&mut self) -> SRAMLPEN_W<AHBLPEN1_SPEC, 16> {
        SRAMLPEN_W::new(self)
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
    #[doc = "Bit 29 - OTGHS clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn otghslpen(&mut self) -> OTGHSLPEN_W<AHBLPEN1_SPEC, 29> {
        OTGHSLPEN_W::new(self)
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
