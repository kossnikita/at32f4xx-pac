#[doc = "Register `EXINTC4` reader"]
pub type R = crate::R<EXINTC4_SPEC>;
#[doc = "Register `EXINTC4` writer"]
pub type W = crate::W<EXINTC4_SPEC>;
#[doc = "Field `EXINT12` reader - Configure EXINT12 source"]
pub type EXINT12_R = crate::FieldReader;
#[doc = "Field `EXINT12` writer - Configure EXINT12 source"]
pub type EXINT12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXINT13` reader - Configure EXINT13 source"]
pub type EXINT13_R = crate::FieldReader;
#[doc = "Field `EXINT13` writer - Configure EXINT13 source"]
pub type EXINT13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXINT14` reader - Configure EXINT14 source"]
pub type EXINT14_R = crate::FieldReader;
#[doc = "Field `EXINT14` writer - Configure EXINT14 source"]
pub type EXINT14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXINT15` reader - Configure EXINT15 source"]
pub type EXINT15_R = crate::FieldReader;
#[doc = "Field `EXINT15` writer - Configure EXINT15 source"]
pub type EXINT15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Configure EXINT12 source"]
    #[inline(always)]
    pub fn exint12(&self) -> EXINT12_R {
        EXINT12_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configure EXINT13 source"]
    #[inline(always)]
    pub fn exint13(&self) -> EXINT13_R {
        EXINT13_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Configure EXINT14 source"]
    #[inline(always)]
    pub fn exint14(&self) -> EXINT14_R {
        EXINT14_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Configure EXINT15 source"]
    #[inline(always)]
    pub fn exint15(&self) -> EXINT15_R {
        EXINT15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configure EXINT12 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint12(&mut self) -> EXINT12_W<EXINTC4_SPEC, 0> {
        EXINT12_W::new(self)
    }
    #[doc = "Bits 4:7 - Configure EXINT13 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint13(&mut self) -> EXINT13_W<EXINTC4_SPEC, 4> {
        EXINT13_W::new(self)
    }
    #[doc = "Bits 8:11 - Configure EXINT14 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint14(&mut self) -> EXINT14_W<EXINTC4_SPEC, 8> {
        EXINT14_W::new(self)
    }
    #[doc = "Bits 12:15 - Configure EXINT15 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint15(&mut self) -> EXINT15_W<EXINTC4_SPEC, 12> {
        EXINT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "External interrupt configuration register 4 (IOMUX_EXINTC4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXINTC4_SPEC;
impl crate::RegisterSpec for EXINTC4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc4::R`](R) reader structure"]
impl crate::Readable for EXINTC4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exintc4::W`](W) writer structure"]
impl crate::Writable for EXINTC4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXINTC4 to value 0"]
impl crate::Resettable for EXINTC4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
