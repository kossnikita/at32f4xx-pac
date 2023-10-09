#[doc = "Register `RTOV` reader"]
pub type R = crate::R<RTOV_SPEC>;
#[doc = "Register `RTOV` writer"]
pub type W = crate::W<RTOV_SPEC>;
#[doc = "Field `RTOV` reader - Receiver time out value"]
pub type RTOV_R = crate::FieldReader<u32>;
#[doc = "Field `RTOV` writer - Receiver time out value"]
pub type RTOV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Receiver time out value"]
    #[inline(always)]
    pub fn rtov(&self) -> RTOV_R {
        RTOV_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTOV")
            .field("rtov", &format_args!("{}", self.rtov().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<RTOV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:23 - Receiver time out value"]
    #[inline(always)]
    #[must_use]
    pub fn rtov(&mut self) -> RTOV_W<RTOV_SPEC, 0> {
        RTOV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receiver time out value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtov::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtov::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTOV_SPEC;
impl crate::RegisterSpec for RTOV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtov::R`](R) reader structure"]
impl crate::Readable for RTOV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtov::W`](W) writer structure"]
impl crate::Writable for RTOV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTOV to value 0"]
impl crate::Resettable for RTOV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
