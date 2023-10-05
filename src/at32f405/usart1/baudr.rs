#[doc = "Register `BAUDR` reader"]
pub type R = crate::R<BAUDR_SPEC>;
#[doc = "Register `BAUDR` writer"]
pub type W = crate::W<BAUDR_SPEC>;
#[doc = "Field `DIV` reader - Division"]
pub type DIV_R = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Division"]
pub type DIV_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Division"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Division"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<BAUDR_SPEC, 0> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Baud rate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baudr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baudr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BAUDR_SPEC;
impl crate::RegisterSpec for BAUDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baudr::R`](R) reader structure"]
impl crate::Readable for BAUDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`baudr::W`](W) writer structure"]
impl crate::Writable for BAUDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAUDR to value 0"]
impl crate::Resettable for BAUDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
