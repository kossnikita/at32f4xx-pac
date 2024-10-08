#[doc = "Register `UHDRV` reader"]
pub type R = crate::R<UHDRV_SPEC>;
#[doc = "Register `UHDRV` writer"]
pub type W = crate::W<UHDRV_SPEC>;
#[doc = "Field `PB3_UH` reader - PB3 ultra high sourcing/sinking strength"]
pub type PB3_UH_R = crate::BitReader;
#[doc = "Field `PB3_UH` writer - PB3 ultra high sourcing/sinking strength"]
pub type PB3_UH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB9_UH` reader - PB9 ultra high sourcing/sinking strength"]
pub type PB9_UH_R = crate::BitReader;
#[doc = "Field `PB9_UH` writer - PB9 ultra high sourcing/sinking strength"]
pub type PB9_UH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB10_UH` reader - PB10 ultra high sourcing/sinking strength"]
pub type PB10_UH_R = crate::BitReader;
#[doc = "Field `PB10_UH` writer - PB10 ultra high sourcing/sinking strength"]
pub type PB10_UH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD12_UH` reader - PD12 ultra high sourcing/sinking strength"]
pub type PD12_UH_R = crate::BitReader;
#[doc = "Field `PD12_UH` writer - PD12 ultra high sourcing/sinking strength"]
pub type PD12_UH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD13_UH` reader - PD13 ultra high sourcing/sinking strength"]
pub type PD13_UH_R = crate::BitReader;
#[doc = "Field `PD13_UH` writer - PD13 ultra high sourcing/sinking strength"]
pub type PD13_UH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD14_UH` reader - PD14 ultra high sourcing/sinking strength"]
pub type PD14_UH_R = crate::BitReader;
#[doc = "Field `PD14_UH` writer - PD14 ultra high sourcing/sinking strength"]
pub type PD14_UH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD15_UH` reader - PD15 ultra high sourcing/sinking strength"]
pub type PD15_UH_R = crate::BitReader;
#[doc = "Field `PD15_UH` writer - PD15 ultra high sourcing/sinking strength"]
pub type PD15_UH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PF14_UH` reader - PF14 ultra high sourcing/sinking strength"]
pub type PF14_UH_R = crate::BitReader;
#[doc = "Field `PF14_UH` writer - PF14 ultra high sourcing/sinking strength"]
pub type PF14_UH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PF15_UH` reader - PF15 ultra high sourcing/sinking strength"]
pub type PF15_UH_R = crate::BitReader;
#[doc = "Field `PF15_UH` writer - PF15 ultra high sourcing/sinking strength"]
pub type PF15_UH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PB3 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pb3_uh(&self) -> PB3_UH_R {
        PB3_UH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PB9 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pb9_uh(&self) -> PB9_UH_R {
        PB9_UH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PB10 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pb10_uh(&self) -> PB10_UH_R {
        PB10_UH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - PD12 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pd12_uh(&self) -> PD12_UH_R {
        PD12_UH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PD13 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pd13_uh(&self) -> PD13_UH_R {
        PD13_UH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PD14 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pd14_uh(&self) -> PD14_UH_R {
        PD14_UH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PD15 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pd15_uh(&self) -> PD15_UH_R {
        PD15_UH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PF14 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pf14_uh(&self) -> PF14_UH_R {
        PF14_UH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PF15 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pf15_uh(&self) -> PF15_UH_R {
        PF15_UH_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UHDRV")
            .field("pf15_uh", &self.pf15_uh())
            .field("pf14_uh", &self.pf14_uh())
            .field("pd15_uh", &self.pd15_uh())
            .field("pd14_uh", &self.pd14_uh())
            .field("pd13_uh", &self.pd13_uh())
            .field("pd12_uh", &self.pd12_uh())
            .field("pb10_uh", &self.pb10_uh())
            .field("pb9_uh", &self.pb9_uh())
            .field("pb3_uh", &self.pb3_uh())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PB3 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pb3_uh(&mut self) -> PB3_UH_W<UHDRV_SPEC> {
        PB3_UH_W::new(self, 0)
    }
    #[doc = "Bit 1 - PB9 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pb9_uh(&mut self) -> PB9_UH_W<UHDRV_SPEC> {
        PB9_UH_W::new(self, 1)
    }
    #[doc = "Bit 2 - PB10 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pb10_uh(&mut self) -> PB10_UH_W<UHDRV_SPEC> {
        PB10_UH_W::new(self, 2)
    }
    #[doc = "Bit 5 - PD12 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pd12_uh(&mut self) -> PD12_UH_W<UHDRV_SPEC> {
        PD12_UH_W::new(self, 5)
    }
    #[doc = "Bit 6 - PD13 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pd13_uh(&mut self) -> PD13_UH_W<UHDRV_SPEC> {
        PD13_UH_W::new(self, 6)
    }
    #[doc = "Bit 7 - PD14 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pd14_uh(&mut self) -> PD14_UH_W<UHDRV_SPEC> {
        PD14_UH_W::new(self, 7)
    }
    #[doc = "Bit 8 - PD15 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pd15_uh(&mut self) -> PD15_UH_W<UHDRV_SPEC> {
        PD15_UH_W::new(self, 8)
    }
    #[doc = "Bit 9 - PF14 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pf14_uh(&mut self) -> PF14_UH_W<UHDRV_SPEC> {
        PF14_UH_W::new(self, 9)
    }
    #[doc = "Bit 10 - PF15 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pf15_uh(&mut self) -> PF15_UH_W<UHDRV_SPEC> {
        PF15_UH_W::new(self, 10)
    }
}
#[doc = "Ultra high drive register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhdrv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhdrv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UHDRV_SPEC;
impl crate::RegisterSpec for UHDRV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhdrv::R`](R) reader structure"]
impl crate::Readable for UHDRV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uhdrv::W`](W) writer structure"]
impl crate::Writable for UHDRV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UHDRV to value 0"]
impl crate::Resettable for UHDRV_SPEC {
    const RESET_VALUE: u32 = 0;
}
