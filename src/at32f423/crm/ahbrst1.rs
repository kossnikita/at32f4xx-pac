#[doc = "Register `AHBRST1` reader"]
pub type R = crate::R<AHBRST1_SPEC>;
#[doc = "Register `AHBRST1` writer"]
pub type W = crate::W<AHBRST1_SPEC>;
#[doc = "Field `GPIOA` reader - IO port A reset"]
pub type GPIOA_R = crate::BitReader;
#[doc = "Field `GPIOA` writer - IO port A reset"]
pub type GPIOA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOB` reader - IO port B reset"]
pub type GPIOB_R = crate::BitReader;
#[doc = "Field `GPIOB` writer - IO port B reset"]
pub type GPIOB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOC` reader - IO port C reset"]
pub type GPIOC_R = crate::BitReader;
#[doc = "Field `GPIOC` writer - IO port C reset"]
pub type GPIOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOD` reader - IO port D reset"]
pub type GPIOD_R = crate::BitReader;
#[doc = "Field `GPIOD` writer - IO port D reset"]
pub type GPIOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOE` reader - IO port E reset"]
pub type GPIOE_R = crate::BitReader;
#[doc = "Field `GPIOE` writer - IO port E reset"]
pub type GPIOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOF` reader - IO port F reset"]
pub type GPIOF_R = crate::BitReader;
#[doc = "Field `GPIOF` writer - IO port F reset"]
pub type GPIOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - CRC reset"]
pub type CRC_R = crate::BitReader;
#[doc = "Field `CRC` writer - CRC reset"]
pub type CRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1` reader - DMA1 reset"]
pub type DMA1_R = crate::BitReader;
#[doc = "Field `DMA1` writer - DMA1 reset"]
pub type DMA1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2` reader - DMA2 reset"]
pub type DMA2_R = crate::BitReader;
#[doc = "Field `DMA2` writer - DMA2 reset"]
pub type DMA2_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 12) & 1) != 0)
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
            .field("crc", &format_args!("{}", self.crc().bit()))
            .field("dma1", &format_args!("{}", self.dma1().bit()))
            .field("dma2", &format_args!("{}", self.dma2().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<AHBRST1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioa(&mut self) -> GPIOA_W<AHBRST1_SPEC> {
        GPIOA_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiob(&mut self) -> GPIOB_W<AHBRST1_SPEC> {
        GPIOB_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioc(&mut self) -> GPIOC_W<AHBRST1_SPEC> {
        GPIOC_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiod(&mut self) -> GPIOD_W<AHBRST1_SPEC> {
        GPIOD_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioe(&mut self) -> GPIOE_W<AHBRST1_SPEC> {
        GPIOE_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiof(&mut self) -> GPIOF_W<AHBRST1_SPEC> {
        GPIOF_W::new(self, 5)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<AHBRST1_SPEC> {
        CRC_W::new(self, 12)
    }
    #[doc = "Bit 22 - DMA1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma1(&mut self) -> DMA1_W<AHBRST1_SPEC> {
        DMA1_W::new(self, 22)
    }
    #[doc = "Bit 24 - DMA2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2(&mut self) -> DMA2_W<AHBRST1_SPEC> {
        DMA2_W::new(self, 24)
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
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRST1 to value 0"]
impl crate::Resettable for AHBRST1_SPEC {
    const RESET_VALUE: u32 = 0;
}
