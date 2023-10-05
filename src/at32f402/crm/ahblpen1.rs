#[doc = "Register `AHBLPEN1` reader"]
pub type R = crate::R<AHBLPEN1_SPEC>;
#[doc = "Register `AHBLPEN1` writer"]
pub type W = crate::W<AHBLPEN1_SPEC>;
#[doc = "Field `GPIOALP` reader - IO A clock enable during sleep mode"]
pub type GPIOALP_R = crate::BitReader;
#[doc = "Field `GPIOALP` writer - IO A clock enable during sleep mode"]
pub type GPIOALP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOBLP` reader - IO B clock enable during sleep mode"]
pub type GPIOBLP_R = crate::BitReader;
#[doc = "Field `GPIOBLP` writer - IO B clock enable during sleep mode"]
pub type GPIOBLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOCLP` reader - IO C clock enable during sleep mode"]
pub type GPIOCLP_R = crate::BitReader;
#[doc = "Field `GPIOCLP` writer - IO C clock enable during sleep mode"]
pub type GPIOCLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIODLP` reader - IO D clock enable during sleep mode"]
pub type GPIODLP_R = crate::BitReader;
#[doc = "Field `GPIODLP` writer - IO D clock enable during sleep mode"]
pub type GPIODLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOFLP` reader - IO F clock enable during sleep mode"]
pub type GPIOFLP_R = crate::BitReader;
#[doc = "Field `GPIOFLP` writer - IO F clock enable during sleep mode"]
pub type GPIOFLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCLP` reader - CRC clock enable during sleep mode"]
pub type CRCLP_R = crate::BitReader;
#[doc = "Field `CRCLP` writer - CRC clock enable during sleep mode"]
pub type CRCLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLASHLP` reader - Flash clock enable during sleep mode"]
pub type FLASHLP_R = crate::BitReader;
#[doc = "Field `FLASHLP` writer - Flash clock enable during sleep mode"]
pub type FLASHLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRAMLP` reader - SRAM clock enable during sleep mode"]
pub type SRAMLP_R = crate::BitReader;
#[doc = "Field `SRAMLP` writer - SRAM clock enable during sleep mode"]
pub type SRAMLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA1LP` reader - DMA1 clock enable during sleep mode"]
pub type DMA1LP_R = crate::BitReader;
#[doc = "Field `DMA1LP` writer - DMA1 clock enable during sleep mode"]
pub type DMA1LP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA2LP` reader - DMA2 clock enable during sleep mode"]
pub type DMA2LP_R = crate::BitReader;
#[doc = "Field `DMA2LP` writer - DMA2 clock enable during sleep mode"]
pub type DMA2LP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OTGHSLP` reader - OTGHS clock enable during sleep mode"]
pub type OTGHSLP_R = crate::BitReader;
#[doc = "Field `OTGHSLP` writer - OTGHS clock enable during sleep mode"]
pub type OTGHSLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 5 - IO F clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioflp(&self) -> GPIOFLP_R {
        GPIOFLP_R::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bit 16 - SRAM clock enable during sleep mode"]
    #[inline(always)]
    pub fn sramlp(&self) -> SRAMLP_R {
        SRAMLP_R::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bit 29 - OTGHS clock enable during sleep mode"]
    #[inline(always)]
    pub fn otghslp(&self) -> OTGHSLP_R {
        OTGHSLP_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO A clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioalp(&mut self) -> GPIOALP_W<AHBLPEN1_SPEC, 0> {
        GPIOALP_W::new(self)
    }
    #[doc = "Bit 1 - IO B clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioblp(&mut self) -> GPIOBLP_W<AHBLPEN1_SPEC, 1> {
        GPIOBLP_W::new(self)
    }
    #[doc = "Bit 2 - IO C clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioclp(&mut self) -> GPIOCLP_W<AHBLPEN1_SPEC, 2> {
        GPIOCLP_W::new(self)
    }
    #[doc = "Bit 3 - IO D clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodlp(&mut self) -> GPIODLP_W<AHBLPEN1_SPEC, 3> {
        GPIODLP_W::new(self)
    }
    #[doc = "Bit 5 - IO F clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioflp(&mut self) -> GPIOFLP_W<AHBLPEN1_SPEC, 5> {
        GPIOFLP_W::new(self)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn crclp(&mut self) -> CRCLP_W<AHBLPEN1_SPEC, 12> {
        CRCLP_W::new(self)
    }
    #[doc = "Bit 15 - Flash clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn flashlp(&mut self) -> FLASHLP_W<AHBLPEN1_SPEC, 15> {
        FLASHLP_W::new(self)
    }
    #[doc = "Bit 16 - SRAM clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sramlp(&mut self) -> SRAMLP_W<AHBLPEN1_SPEC, 16> {
        SRAMLP_W::new(self)
    }
    #[doc = "Bit 22 - DMA1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma1lp(&mut self) -> DMA1LP_W<AHBLPEN1_SPEC, 22> {
        DMA1LP_W::new(self)
    }
    #[doc = "Bit 24 - DMA2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma2lp(&mut self) -> DMA2LP_W<AHBLPEN1_SPEC, 24> {
        DMA2LP_W::new(self)
    }
    #[doc = "Bit 29 - OTGHS clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn otghslp(&mut self) -> OTGHSLP_W<AHBLPEN1_SPEC, 29> {
        OTGHSLP_W::new(self)
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
