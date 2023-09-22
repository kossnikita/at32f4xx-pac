#[doc = "Register `C1` reader"]
pub type R = crate::R<C1_SPEC>;
#[doc = "Register `C1` writer"]
pub type W = crate::W<C1_SPEC>;
#[doc = "Field `C1` reader - Compare 1"]
pub type C1_R = crate::FieldReader<u16>;
#[doc = "Field `C1` writer - Compare 1"]
pub type C1_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare 1"]
    #[inline(always)]
    pub fn c1(&self) -> C1_R {
        C1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn c1(&mut self) -> C1_W<C1_SPEC, 0> {
        C1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "compare value 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1_SPEC;
impl crate::RegisterSpec for C1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1::R`](R) reader structure"]
impl crate::Readable for C1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c1::W`](W) writer structure"]
impl crate::Writable for C1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1 to value 0x1f2c"]
impl crate::Resettable for C1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f2c;
}
