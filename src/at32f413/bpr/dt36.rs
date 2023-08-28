#[doc = "Register `DT36` reader"]
pub type R = crate::R<DT36_SPEC>;
#[doc = "Register `DT36` writer"]
pub type W = crate::W<DT36_SPEC>;
#[doc = "Field `DT36` reader - BPR data36"]
pub type DT36_R = crate::FieldReader<u16>;
#[doc = "Field `DT36` writer - BPR data36"]
pub type DT36_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data36"]
    #[inline(always)]
    pub fn dt36(&self) -> DT36_R {
        DT36_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data36"]
    #[inline(always)]
    #[must_use]
    pub fn dt36(&mut self) -> DT36_W<DT36_SPEC, 0> {
        DT36_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt36::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt36::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT36_SPEC;
impl crate::RegisterSpec for DT36_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt36::R`](R) reader structure"]
impl crate::Readable for DT36_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt36::W`](W) writer structure"]
impl crate::Writable for DT36_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT36 to value 0"]
impl crate::Resettable for DT36_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
