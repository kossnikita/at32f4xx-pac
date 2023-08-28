#[doc = "Register `C2DT` reader"]
pub type R = crate::R<C2DT_SPEC>;
#[doc = "Register `C2DT` writer"]
pub type W = crate::W<C2DT_SPEC>;
#[doc = "Field `C2DT` reader - Channel 2 data register"]
pub type C2DT_R = crate::FieldReader<u32>;
#[doc = "Field `C2DT` writer - Channel 2 data register"]
pub type C2DT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 2 data register"]
    #[inline(always)]
    pub fn c2dt(&self) -> C2DT_R {
        C2DT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel 2 data register"]
    #[inline(always)]
    #[must_use]
    pub fn c2dt(&mut self) -> C2DT_W<C2DT_SPEC, 0> {
        C2DT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel 2 data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2dt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2dt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2DT_SPEC;
impl crate::RegisterSpec for C2DT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2dt::R`](R) reader structure"]
impl crate::Readable for C2DT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c2dt::W`](W) writer structure"]
impl crate::Writable for C2DT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2DT to value 0"]
impl crate::Resettable for C2DT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}