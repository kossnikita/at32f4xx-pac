#[doc = "Register `UHDRV` reader"]
pub type R = crate::R<UHDRV_SPEC>;
#[doc = "Register `UHDRV` writer"]
pub type W = crate::W<UHDRV_SPEC>;
#[doc = "Field `PB9_UH` reader - PB9 ultra high sourcing/sinking strength"]
pub type PB9_UH_R = crate::BitReader;
#[doc = "Field `PB9_UH` writer - PB9 ultra high sourcing/sinking strength"]
pub type PB9_UH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PB8_UH` reader - PB8 ultra high sourcing/sinking strength"]
pub type PB8_UH_R = crate::BitReader;
#[doc = "Field `PB8_UH` writer - PB8 ultra high sourcing/sinking strength"]
pub type PB8_UH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PD12_UH` reader - PD12 ultra high sourcing/sinking strength"]
pub type PD12_UH_R = crate::BitReader;
#[doc = "Field `PD12_UH` writer - PD12 ultra high sourcing/sinking strength"]
pub type PD12_UH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PD13_UH` reader - PD13 ultra high sourcing/sinking strength"]
pub type PD13_UH_R = crate::BitReader;
#[doc = "Field `PD13_UH` writer - PD13 ultra high sourcing/sinking strength"]
pub type PD13_UH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - PB9 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pb9_uh(&self) -> PB9_UH_R {
        PB9_UH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - PB8 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pb8_uh(&self) -> PB8_UH_R {
        PB8_UH_R::new(((self.bits >> 3) & 1) != 0)
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
}
impl W {
    #[doc = "Bit 1 - PB9 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pb9_uh(&mut self) -> PB9_UH_W<UHDRV_SPEC, 1> {
        PB9_UH_W::new(self)
    }
    #[doc = "Bit 3 - PB8 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pb8_uh(&mut self) -> PB8_UH_W<UHDRV_SPEC, 3> {
        PB8_UH_W::new(self)
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
