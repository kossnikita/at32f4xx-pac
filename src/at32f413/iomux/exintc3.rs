#[doc = "Register `EXINTC3` reader"]
pub type R = crate::R<EXINTC3_SPEC>;
#[doc = "Register `EXINTC3` writer"]
pub type W = crate::W<EXINTC3_SPEC>;
#[doc = "Field `EXINT8` reader - Configure EXINT8 source"]
pub type EXINT8_R = crate::FieldReader;
#[doc = "Field `EXINT8` writer - Configure EXINT8 source"]
pub type EXINT8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT9` reader - Configure EXINT9 source"]
pub type EXINT9_R = crate::FieldReader;
#[doc = "Field `EXINT9` writer - Configure EXINT9 source"]
pub type EXINT9_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT10` reader - Configure EXINT10 source"]
pub type EXINT10_R = crate::FieldReader;
#[doc = "Field `EXINT10` writer - Configure EXINT10 source"]
pub type EXINT10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT11` reader - Configure EXINT11 source"]
pub type EXINT11_R = crate::FieldReader;
#[doc = "Field `EXINT11` writer - Configure EXINT11 source"]
pub type EXINT11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configure EXINT8 source"]
    #[inline(always)]
    pub fn exint8(&self) -> EXINT8_R {
        EXINT8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configure EXINT9 source"]
    #[inline(always)]
    pub fn exint9(&self) -> EXINT9_R {
        EXINT9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Configure EXINT10 source"]
    #[inline(always)]
    pub fn exint10(&self) -> EXINT10_R {
        EXINT10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Configure EXINT11 source"]
    #[inline(always)]
    pub fn exint11(&self) -> EXINT11_R {
        EXINT11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXINTC3")
            .field("exint8", &format_args!("{}", self.exint8().bits()))
            .field("exint9", &format_args!("{}", self.exint9().bits()))
            .field("exint10", &format_args!("{}", self.exint10().bits()))
            .field("exint11", &format_args!("{}", self.exint11().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<EXINTC3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configure EXINT8 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint8(&mut self) -> EXINT8_W<EXINTC3_SPEC> {
        EXINT8_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Configure EXINT9 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint9(&mut self) -> EXINT9_W<EXINTC3_SPEC> {
        EXINT9_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Configure EXINT10 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint10(&mut self) -> EXINT10_W<EXINTC3_SPEC> {
        EXINT10_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Configure EXINT11 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint11(&mut self) -> EXINT11_W<EXINTC3_SPEC> {
        EXINT11_W::new(self, 12)
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
#[doc = "External interrupt configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXINTC3_SPEC;
impl crate::RegisterSpec for EXINTC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc3::R`](R) reader structure"]
impl crate::Readable for EXINTC3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exintc3::W`](W) writer structure"]
impl crate::Writable for EXINTC3_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXINTC3 to value 0"]
impl crate::Resettable for EXINTC3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
