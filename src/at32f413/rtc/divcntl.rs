#[doc = "Register `DIVCNTL` reader"]
pub type R = crate::R<DIVCNTL_SPEC>;
#[doc = "Register `DIVCNTL` writer"]
pub type W = crate::W<DIVCNTL_SPEC>;
#[doc = "Field `DIVCNT` reader - RTC divider register low"]
pub type DIVCNT_R = crate::FieldReader<u16>;
#[doc = "Field `DIVCNT` writer - RTC divider register low"]
pub type DIVCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC divider register low"]
    #[inline(always)]
    pub fn divcnt(&self) -> DIVCNT_R {
        DIVCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC divider register low"]
    #[inline(always)]
    #[must_use]
    pub fn divcnt(&mut self) -> DIVCNT_W<DIVCNTL_SPEC, 0> {
        DIVCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC Divider Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divcntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divcntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIVCNTL_SPEC;
impl crate::RegisterSpec for DIVCNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divcntl::R`](R) reader structure"]
impl crate::Readable for DIVCNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`divcntl::W`](W) writer structure"]
impl crate::Writable for DIVCNTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIVCNTL to value 0x8000"]
impl crate::Resettable for DIVCNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}