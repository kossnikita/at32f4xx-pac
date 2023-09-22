#[doc = "Register `C2` reader"]
pub type R = crate::R<C2_SPEC>;
#[doc = "Register `C2` writer"]
pub type W = crate::W<C2_SPEC>;
#[doc = "Field `C2` reader - Compare 2"]
pub type C2_R = crate::FieldReader<u16>;
#[doc = "Field `C2` writer - Compare 2"]
pub type C2_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare 2"]
    #[inline(always)]
    pub fn c2(&self) -> C2_R {
        C2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare 2"]
    #[inline(always)]
    #[must_use]
    pub fn c2(&mut self) -> C2_W<C2_SPEC, 0> {
        C2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Compare 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2_SPEC;
impl crate::RegisterSpec for C2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2::R`](R) reader structure"]
impl crate::Readable for C2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c2::W`](W) writer structure"]
impl crate::Writable for C2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2 to value 0x1f40"]
impl crate::Resettable for C2_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f40;
}
