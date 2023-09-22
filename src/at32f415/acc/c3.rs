#[doc = "Register `C3` reader"]
pub type R = crate::R<C3_SPEC>;
#[doc = "Register `C3` writer"]
pub type W = crate::W<C3_SPEC>;
#[doc = "Field `C3` reader - Compare 3"]
pub type C3_R = crate::FieldReader<u16>;
#[doc = "Field `C3` writer - Compare 3"]
pub type C3_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare 3"]
    #[inline(always)]
    pub fn c3(&self) -> C3_R {
        C3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare 3"]
    #[inline(always)]
    #[must_use]
    pub fn c3(&mut self) -> C3_W<C3_SPEC, 0> {
        C3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Compare 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C3_SPEC;
impl crate::RegisterSpec for C3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c3::R`](R) reader structure"]
impl crate::Readable for C3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c3::W`](W) writer structure"]
impl crate::Writable for C3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C3 to value 0x1f54"]
impl crate::Resettable for C3_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f54;
}
