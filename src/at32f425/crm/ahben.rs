#[doc = "Register `AHBEN` reader"]
pub type R = crate::R<AHBEN_SPEC>;
#[doc = "Register `AHBEN` writer"]
pub type W = crate::W<AHBEN_SPEC>;
#[doc = "Field `DMA1` reader - DMA1 clock enable"]
pub type DMA1_R = crate::BitReader;
#[doc = "Field `DMA1` writer - DMA1 clock enable"]
pub type DMA1_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `OTGFS1` reader - OTGFS1 clock enable"]
pub type OTGFS1_R = crate::BitReader;
#[doc = "Field `OTGFS1` writer - OTGFS1 clock enable"]
pub type OTGFS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOA` reader - I/O port A clock enable"]
pub type GPIOA_R = crate::BitReader;
#[doc = "Field `GPIOA` writer - I/O port A clock enable"]
pub type GPIOA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOB` reader - I/O port B clock enable"]
pub type GPIOB_R = crate::BitReader;
#[doc = "Field `GPIOB` writer - I/O port B clock enable"]
pub type GPIOB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOC` reader - I/O port C clock enable"]
pub type GPIOC_R = crate::BitReader;
#[doc = "Field `GPIOC` writer - I/O port C clock enable"]
pub type GPIOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOD` reader - I/O port D clock enable"]
pub type GPIOD_R = crate::BitReader;
#[doc = "Field `GPIOD` writer - I/O port D clock enable"]
pub type GPIOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOF` reader - I/O port F clock enable"]
pub type GPIOF_R = crate::BitReader;
#[doc = "Field `GPIOF` writer - I/O port F clock enable"]
pub type GPIOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1(&self) -> DMA1_R {
        DMA1_R::new((self.bits & 1) != 0)
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
    #[doc = "Bit 12 - OTGFS1 clock enable"]
    #[inline(always)]
    pub fn otgfs1(&self) -> OTGFS1_R {
        OTGFS1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - I/O port A clock enable"]
    #[inline(always)]
    pub fn gpioa(&self) -> GPIOA_R {
        GPIOA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I/O port B clock enable"]
    #[inline(always)]
    pub fn gpiob(&self) -> GPIOB_R {
        GPIOB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I/O port C clock enable"]
    #[inline(always)]
    pub fn gpioc(&self) -> GPIOC_R {
        GPIOC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I/O port D clock enable"]
    #[inline(always)]
    pub fn gpiod(&self) -> GPIOD_R {
        GPIOD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - I/O port F clock enable"]
    #[inline(always)]
    pub fn gpiof(&self) -> GPIOF_R {
        GPIOF_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBEN")
            .field("dma1", &format_args!("{}", self.dma1().bit()))
            .field("sram", &format_args!("{}", self.sram().bit()))
            .field("flash", &format_args!("{}", self.flash().bit()))
            .field("crc", &format_args!("{}", self.crc().bit()))
            .field("otgfs1", &format_args!("{}", self.otgfs1().bit()))
            .field("gpioa", &format_args!("{}", self.gpioa().bit()))
            .field("gpiob", &format_args!("{}", self.gpiob().bit()))
            .field("gpioc", &format_args!("{}", self.gpioc().bit()))
            .field("gpiod", &format_args!("{}", self.gpiod().bit()))
            .field("gpiof", &format_args!("{}", self.gpiof().bit()))
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
    #[doc = "Bit 12 - OTGFS1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs1(&mut self) -> OTGFS1_W<AHBEN_SPEC> {
        OTGFS1_W::new(self, 12)
    }
    #[doc = "Bit 17 - I/O port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioa(&mut self) -> GPIOA_W<AHBEN_SPEC> {
        GPIOA_W::new(self, 17)
    }
    #[doc = "Bit 18 - I/O port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiob(&mut self) -> GPIOB_W<AHBEN_SPEC> {
        GPIOB_W::new(self, 18)
    }
    #[doc = "Bit 19 - I/O port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioc(&mut self) -> GPIOC_W<AHBEN_SPEC> {
        GPIOC_W::new(self, 19)
    }
    #[doc = "Bit 20 - I/O port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiod(&mut self) -> GPIOD_W<AHBEN_SPEC> {
        GPIOD_W::new(self, 20)
    }
    #[doc = "Bit 22 - I/O port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiof(&mut self) -> GPIOF_W<AHBEN_SPEC> {
        GPIOF_W::new(self, 22)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBEN to value 0x14"]
impl crate::Resettable for AHBEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x14;
}
