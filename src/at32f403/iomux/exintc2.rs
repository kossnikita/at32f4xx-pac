#[doc = "Register `EXINTC2` reader"]
pub type R = crate::R<EXINTC2_SPEC>;
#[doc = "Register `EXINTC2` writer"]
pub type W = crate::W<EXINTC2_SPEC>;
#[doc = "Field `EXINT4` reader - Configure EXINT4 source"]
pub type EXINT4_R = crate::FieldReader;
#[doc = "Field `EXINT4` writer - Configure EXINT4 source"]
pub type EXINT4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT5` reader - Configure EXINT5 source"]
pub type EXINT5_R = crate::FieldReader;
#[doc = "Field `EXINT5` writer - Configure EXINT5 source"]
pub type EXINT5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT6` reader - Configure EXINT6 source"]
pub type EXINT6_R = crate::FieldReader;
#[doc = "Field `EXINT6` writer - Configure EXINT6 source"]
pub type EXINT6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT7` reader - Configure EXINT7 source"]
pub type EXINT7_R = crate::FieldReader;
#[doc = "Field `EXINT7` writer - Configure EXINT7 source"]
pub type EXINT7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configure EXINT4 source"]
    #[inline(always)]
    pub fn exint4(&self) -> EXINT4_R {
        EXINT4_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configure EXINT5 source"]
    #[inline(always)]
    pub fn exint5(&self) -> EXINT5_R {
        EXINT5_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Configure EXINT6 source"]
    #[inline(always)]
    pub fn exint6(&self) -> EXINT6_R {
        EXINT6_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Configure EXINT7 source"]
    #[inline(always)]
    pub fn exint7(&self) -> EXINT7_R {
        EXINT7_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXINTC2")
            .field("exint4", &format_args!("{}", self.exint4().bits()))
            .field("exint5", &format_args!("{}", self.exint5().bits()))
            .field("exint6", &format_args!("{}", self.exint6().bits()))
            .field("exint7", &format_args!("{}", self.exint7().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<EXINTC2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configure EXINT4 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint4(&mut self) -> EXINT4_W<EXINTC2_SPEC> {
        EXINT4_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Configure EXINT5 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint5(&mut self) -> EXINT5_W<EXINTC2_SPEC> {
        EXINT5_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Configure EXINT6 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint6(&mut self) -> EXINT6_W<EXINTC2_SPEC> {
        EXINT6_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Configure EXINT7 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint7(&mut self) -> EXINT7_W<EXINTC2_SPEC> {
        EXINT7_W::new(self, 12)
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
#[doc = "External interrupt configuration register 2 (IOMUX_EXINTC2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXINTC2_SPEC;
impl crate::RegisterSpec for EXINTC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc2::R`](R) reader structure"]
impl crate::Readable for EXINTC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exintc2::W`](W) writer structure"]
impl crate::Writable for EXINTC2_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXINTC2 to value 0"]
impl crate::Resettable for EXINTC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
