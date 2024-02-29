#[doc = "Register `DIVCNTH` reader"]
pub type R = crate::R<DIVCNTH_SPEC>;
#[doc = "Register `DIVCNTH` writer"]
pub type W = crate::W<DIVCNTH_SPEC>;
#[doc = "Field `DIVCNT` reader - RTC divider register high"]
pub type DIVCNT_R = crate::FieldReader;
#[doc = "Field `DIVCNT` writer - RTC divider register high"]
pub type DIVCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - RTC divider register high"]
    #[inline(always)]
    pub fn divcnt(&self) -> DIVCNT_R {
        DIVCNT_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIVCNTH")
            .field("divcnt", &format_args!("{}", self.divcnt().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DIVCNTH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - RTC divider register high"]
    #[inline(always)]
    #[must_use]
    pub fn divcnt(&mut self) -> DIVCNT_W<DIVCNTH_SPEC> {
        DIVCNT_W::new(self, 0)
    }
}
#[doc = "RTC Divider Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divcnth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divcnth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIVCNTH_SPEC;
impl crate::RegisterSpec for DIVCNTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divcnth::R`](R) reader structure"]
impl crate::Readable for DIVCNTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`divcnth::W`](W) writer structure"]
impl crate::Writable for DIVCNTH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIVCNTH to value 0"]
impl crate::Resettable for DIVCNTH_SPEC {
    const RESET_VALUE: u32 = 0;
}
