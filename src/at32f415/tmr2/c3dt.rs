#[doc = "Register `C3DT` reader"]
pub type R = crate::R<C3DT_SPEC>;
#[doc = "Register `C3DT` writer"]
pub type W = crate::W<C3DT_SPEC>;
#[doc = "Field `C3DT` reader - Channel 3 data register"]
pub type C3DT_R = crate::FieldReader<u32>;
#[doc = "Field `C3DT` writer - Channel 3 data register"]
pub type C3DT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 3 data register"]
    #[inline(always)]
    pub fn c3dt(&self) -> C3DT_R {
        C3DT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel 3 data register"]
    #[inline(always)]
    #[must_use]
    pub fn c3dt(&mut self) -> C3DT_W<C3DT_SPEC, 0> {
        C3DT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel 3 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3dt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3dt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C3DT_SPEC;
impl crate::RegisterSpec for C3DT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c3dt::R`](R) reader structure"]
impl crate::Readable for C3DT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c3dt::W`](W) writer structure"]
impl crate::Writable for C3DT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C3DT to value 0"]
impl crate::Resettable for C3DT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
