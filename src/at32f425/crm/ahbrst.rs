#[doc = "Register `AHBRST` reader"]
pub type R = crate::R<AHBRST_SPEC>;
#[doc = "Register `AHBRST` writer"]
pub type W = crate::W<AHBRST_SPEC>;
#[doc = "Field `OTGFS1` reader - OTGFS1 reset"]
pub type OTGFS1_R = crate::BitReader;
#[doc = "Field `OTGFS1` writer - OTGFS1 reset"]
pub type OTGFS1_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `GPIOF` reader - IO port F reset"]
pub type GPIOF_R = crate::BitReader;
#[doc = "Field `GPIOF` writer - IO port F reset"]
pub type GPIOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - OTGFS1 reset"]
    #[inline(always)]
    pub fn otgfs1(&self) -> OTGFS1_R {
        OTGFS1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - IO port A reset"]
    #[inline(always)]
    pub fn gpioa(&self) -> GPIOA_R {
        GPIOA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IO port B reset"]
    #[inline(always)]
    pub fn gpiob(&self) -> GPIOB_R {
        GPIOB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - IO port C reset"]
    #[inline(always)]
    pub fn gpioc(&self) -> GPIOC_R {
        GPIOC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IO port D reset"]
    #[inline(always)]
    pub fn gpiod(&self) -> GPIOD_R {
        GPIOD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - IO port F reset"]
    #[inline(always)]
    pub fn gpiof(&self) -> GPIOF_R {
        GPIOF_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRST")
            .field("otgfs1", &self.otgfs1())
            .field("gpioa", &self.gpioa())
            .field("gpiob", &self.gpiob())
            .field("gpioc", &self.gpioc())
            .field("gpiod", &self.gpiod())
            .field("gpiof", &self.gpiof())
            .finish()
    }
}
impl W {
    #[doc = "Bit 12 - OTGFS1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs1(&mut self) -> OTGFS1_W<AHBRST_SPEC> {
        OTGFS1_W::new(self, 12)
    }
    #[doc = "Bit 17 - IO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioa(&mut self) -> GPIOA_W<AHBRST_SPEC> {
        GPIOA_W::new(self, 17)
    }
    #[doc = "Bit 18 - IO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiob(&mut self) -> GPIOB_W<AHBRST_SPEC> {
        GPIOB_W::new(self, 18)
    }
    #[doc = "Bit 19 - IO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioc(&mut self) -> GPIOC_W<AHBRST_SPEC> {
        GPIOC_W::new(self, 19)
    }
    #[doc = "Bit 20 - IO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiod(&mut self) -> GPIOD_W<AHBRST_SPEC> {
        GPIOD_W::new(self, 20)
    }
    #[doc = "Bit 22 - IO port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiof(&mut self) -> GPIOF_W<AHBRST_SPEC> {
        GPIOF_W::new(self, 22)
    }
}
#[doc = "AHB reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRST_SPEC;
impl crate::RegisterSpec for AHBRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst::R`](R) reader structure"]
impl crate::Readable for AHBRST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbrst::W`](W) writer structure"]
impl crate::Writable for AHBRST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRST to value 0"]
impl crate::Resettable for AHBRST_SPEC {
    const RESET_VALUE: u32 = 0;
}
