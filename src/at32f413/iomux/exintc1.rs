#[doc = "Register `EXINTC1` reader"]
pub type R = crate::R<EXINTC1_SPEC>;
#[doc = "Register `EXINTC1` writer"]
pub type W = crate::W<EXINTC1_SPEC>;
#[doc = "Field `EXINT0` reader - Configure EXINT0 source"]
pub type EXINT0_R = crate::FieldReader;
#[doc = "Field `EXINT0` writer - Configure EXINT0 source"]
pub type EXINT0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT1` reader - Configure EXINT1 source"]
pub type EXINT1_R = crate::FieldReader;
#[doc = "Field `EXINT1` writer - Configure EXINT1 source"]
pub type EXINT1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT2` reader - Configure EXINT2 source"]
pub type EXINT2_R = crate::FieldReader;
#[doc = "Field `EXINT2` writer - Configure EXINT2 source"]
pub type EXINT2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT3` reader - Configure EXINT3 source"]
pub type EXINT3_R = crate::FieldReader;
#[doc = "Field `EXINT3` writer - Configure EXINT3 source"]
pub type EXINT3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configure EXINT0 source"]
    #[inline(always)]
    pub fn exint0(&self) -> EXINT0_R {
        EXINT0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configure EXINT1 source"]
    #[inline(always)]
    pub fn exint1(&self) -> EXINT1_R {
        EXINT1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Configure EXINT2 source"]
    #[inline(always)]
    pub fn exint2(&self) -> EXINT2_R {
        EXINT2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Configure EXINT3 source"]
    #[inline(always)]
    pub fn exint3(&self) -> EXINT3_R {
        EXINT3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXINTC1")
            .field("exint0", &format_args!("{}", self.exint0().bits()))
            .field("exint1", &format_args!("{}", self.exint1().bits()))
            .field("exint2", &format_args!("{}", self.exint2().bits()))
            .field("exint3", &format_args!("{}", self.exint3().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<EXINTC1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configure EXINT0 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint0(&mut self) -> EXINT0_W<EXINTC1_SPEC> {
        EXINT0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Configure EXINT1 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint1(&mut self) -> EXINT1_W<EXINTC1_SPEC> {
        EXINT1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Configure EXINT2 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint2(&mut self) -> EXINT2_W<EXINTC1_SPEC> {
        EXINT2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Configure EXINT3 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint3(&mut self) -> EXINT3_W<EXINTC1_SPEC> {
        EXINT3_W::new(self, 12)
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
#[doc = "External interrupt configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXINTC1_SPEC;
impl crate::RegisterSpec for EXINTC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc1::R`](R) reader structure"]
impl crate::Readable for EXINTC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exintc1::W`](W) writer structure"]
impl crate::Writable for EXINTC1_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXINTC1 to value 0"]
impl crate::Resettable for EXINTC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
