#[doc = "Register `UHDRV` reader"]
pub type R = crate::R<UHDRV_SPEC>;
#[doc = "Register `UHDRV` writer"]
pub type W = crate::W<UHDRV_SPEC>;
#[doc = "Field `PB3_UH` reader - PB3 ultra high sourcing/sinking strength"]
pub type PB3_UH_R = crate::BitReader;
#[doc = "Field `PB3_UH` writer - PB3 ultra high sourcing/sinking strength"]
pub type PB3_UH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PB9_UH` reader - PB9 ultra high sourcing/sinking strength"]
pub type PB9_UH_R = crate::BitReader;
#[doc = "Field `PB9_UH` writer - PB9 ultra high sourcing/sinking strength"]
pub type PB9_UH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PB10_UH` reader - PB10 ultra high sourcing/sinking strength"]
pub type PB10_UH_R = crate::BitReader;
#[doc = "Field `PB10_UH` writer - PB10 ultra high sourcing/sinking strength"]
pub type PB10_UH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PD12_UH` reader - PD12 ultra high sourcing/sinking strength"]
pub type PD12_UH_R = crate::BitReader;
#[doc = "Field `PD12_UH` writer - PD12 ultra high sourcing/sinking strength"]
pub type PD12_UH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PD13_UH` reader - PD13 ultra high sourcing/sinking strength"]
pub type PD13_UH_R = crate::BitReader;
#[doc = "Field `PD13_UH` writer - PD13 ultra high sourcing/sinking strength"]
pub type PD13_UH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PD14_UH` reader - PD14 ultra high sourcing/sinking strength"]
pub type PD14_UH_R = crate::BitReader;
#[doc = "Field `PD14_UH` writer - PD14 ultra high sourcing/sinking strength"]
pub type PD14_UH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PD15_UH` reader - PD15 ultra high sourcing/sinking strength"]
pub type PD15_UH_R = crate::BitReader;
#[doc = "Field `PD15_UH` writer - PD15 ultra high sourcing/sinking strength"]
pub type PD15_UH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PF14_UH` reader - PF14 ultra high sourcing/sinking strength"]
pub type PF14_UH_R = crate::BitReader;
#[doc = "Field `PF14_UH` writer - PF14 ultra high sourcing/sinking strength"]
pub type PF14_UH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PF15_UH` reader - PF15 ultra high sourcing/sinking strength"]
pub type PF15_UH_R = crate::BitReader;
#[doc = "Field `PF15_UH` writer - PF15 ultra high sourcing/sinking strength"]
pub type PF15_UH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
impl W {
    #[doc = "Bit 0 - PB3 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pb3_uh(&mut self) -> PB3_UH_W<UHDRV_SPEC, 0> {
        PB3_UH_W::new(self)
    }
    #[doc = "Bit 1 - PB9 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pb9_uh(&mut self) -> PB9_UH_W<UHDRV_SPEC, 1> {
        PB9_UH_W::new(self)
    }
    #[doc = "Bit 2 - PB10 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pb10_uh(&mut self) -> PB10_UH_W<UHDRV_SPEC, 2> {
        PB10_UH_W::new(self)
    }
    #[doc = "Bit 5 - PD12 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pd12_uh(&mut self) -> PD12_UH_W<UHDRV_SPEC, 5> {
        PD12_UH_W::new(self)
    }
    #[doc = "Bit 6 - PD13 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pd13_uh(&mut self) -> PD13_UH_W<UHDRV_SPEC, 6> {
        PD13_UH_W::new(self)
    }
    #[doc = "Bit 7 - PD14 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pd14_uh(&mut self) -> PD14_UH_W<UHDRV_SPEC, 7> {
        PD14_UH_W::new(self)
    }
    #[doc = "Bit 8 - PD15 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pd15_uh(&mut self) -> PD15_UH_W<UHDRV_SPEC, 8> {
        PD15_UH_W::new(self)
    }
    #[doc = "Bit 9 - PF14 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pf14_uh(&mut self) -> PF14_UH_W<UHDRV_SPEC, 9> {
        PF14_UH_W::new(self)
    }
    #[doc = "Bit 10 - PF15 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pf15_uh(&mut self) -> PF15_UH_W<UHDRV_SPEC, 10> {
        PF15_UH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ultra high drive register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhdrv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhdrv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UHDRV_SPEC;
impl crate::RegisterSpec for UHDRV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhdrv::R`](R) reader structure"]
impl crate::Readable for UHDRV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uhdrv::W`](W) writer structure"]
impl crate::Writable for UHDRV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UHDRV to value 0"]
impl crate::Resettable for UHDRV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
