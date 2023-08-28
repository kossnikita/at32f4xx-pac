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
