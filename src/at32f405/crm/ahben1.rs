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
#[doc = "Field `GPIOF` reader - IO F clock enable"]
pub type GPIOF_R = crate::BitReader;
#[doc = "Field `GPIOF` writer - IO F clock enable"]
pub type GPIOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - CRC clock enable"]
pub type CRC_R = crate::BitReader;
#[doc = "Field `CRC` writer - CRC clock enable"]
pub type CRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1` reader - DMA1 clock enable"]
pub type DMA1_R = crate::BitReader;
#[doc = "Field `DMA1` writer - DMA1 clock enable"]
pub type DMA1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2` reader - DMA2 clock enable"]
pub type DMA2_R = crate::BitReader;
#[doc = "Field `DMA2` writer - DMA2 clock enable"]
pub type DMA2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGHS` reader - OTGHS clock enable"]
pub type OTGHS_R = crate::BitReader;
#[doc = "Field `OTGHS` writer - OTGHS clock enable"]
pub type OTGHS_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 5 - IO F clock enable"]
    #[inline(always)]
    pub fn gpiof(&self) -> GPIOF_R {
        GPIOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 12) & 1) != 0)
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
    #[doc = "Bit 29 - OTGHS clock enable"]
    #[inline(always)]
    pub fn otghs(&self) -> OTGHS_R {
        OTGHS_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBEN1")
            .field("gpioa", &format_args!("{}", self.gpioa().bit()))
            .field("gpiob", &format_args!("{}", self.gpiob().bit()))
            .field("gpioc", &format_args!("{}", self.gpioc().bit()))
            .field("gpiod", &format_args!("{}", self.gpiod().bit()))
            .field("gpiof", &format_args!("{}", self.gpiof().bit()))
            .field("crc", &format_args!("{}", self.crc().bit()))
            .field("dma1", &format_args!("{}", self.dma1().bit()))
            .field("dma2", &format_args!("{}", self.dma2().bit()))
            .field("otghs", &format_args!("{}", self.otghs().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<AHBEN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
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
    #[doc = "Bit 5 - IO F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiof(&mut self) -> GPIOF_W<AHBEN1_SPEC> {
        GPIOF_W::new(self, 5)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<AHBEN1_SPEC> {
        CRC_W::new(self, 12)
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
    #[doc = "Bit 29 - OTGHS clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn otghs(&mut self) -> OTGHS_W<AHBEN1_SPEC> {
        OTGHS_W::new(self, 29)
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
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBEN1 to value 0"]
impl crate::Resettable for AHBEN1_SPEC {
    const RESET_VALUE: u32 = 0;
}
