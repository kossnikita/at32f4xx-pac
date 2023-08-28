#[doc = "Register `WAT` reader"]
pub type R = crate::R<WAT_SPEC>;
#[doc = "Register `WAT` writer"]
pub type W = crate::W<WAT_SPEC>;
#[doc = "Field `VAL` reader - Wakeup timer reload value"]
pub type VAL_R = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - Wakeup timer reload value"]
pub type VAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Wakeup timer reload value"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Wakeup timer reload value"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<WAT_SPEC, 0> {
        VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Wakeup timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAT_SPEC;
impl crate::RegisterSpec for WAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wat::R`](R) reader structure"]
impl crate::Readable for WAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wat::W`](W) writer structure"]
impl crate::Writable for WAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAT to value 0xffff"]
impl crate::Resettable for WAT_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
