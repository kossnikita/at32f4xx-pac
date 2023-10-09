#[doc = "Register `ARG` reader"]
pub type R = crate::R<ARG_SPEC>;
#[doc = "Register `ARG` writer"]
pub type W = crate::W<ARG_SPEC>;
#[doc = "Field `ARG` reader - Command argument"]
pub type ARG_R = crate::FieldReader<u32>;
#[doc = "Field `ARG` writer - Command argument"]
pub type ARG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Command argument"]
    #[inline(always)]
    pub fn arg(&self) -> ARG_R {
        ARG_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARG")
            .field("arg", &format_args!("{}", self.arg().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<ARG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command argument"]
    #[inline(always)]
    #[must_use]
    pub fn arg(&mut self) -> ARG_W<ARG_SPEC, 0> {
        ARG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Bits 31:0 = : Command argument\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARG_SPEC;
impl crate::RegisterSpec for ARG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arg::R`](R) reader structure"]
impl crate::Readable for ARG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arg::W`](W) writer structure"]
impl crate::Writable for ARG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARG to value 0"]
impl crate::Resettable for ARG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
