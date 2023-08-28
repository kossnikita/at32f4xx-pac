#[doc = "Register `DT19` reader"]
pub type R = crate::R<DT19_SPEC>;
#[doc = "Register `DT19` writer"]
pub type W = crate::W<DT19_SPEC>;
#[doc = "Field `DT19` reader - BPR data19"]
pub type DT19_R = crate::FieldReader<u16>;
#[doc = "Field `DT19` writer - BPR data19"]
pub type DT19_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data19"]
    #[inline(always)]
    pub fn dt19(&self) -> DT19_R {
        DT19_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data19"]
    #[inline(always)]
    #[must_use]
    pub fn dt19(&mut self) -> DT19_W<DT19_SPEC, 0> {
        DT19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT19_SPEC;
impl crate::RegisterSpec for DT19_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt19::R`](R) reader structure"]
impl crate::Readable for DT19_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt19::W`](W) writer structure"]
impl crate::Writable for DT19_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT19 to value 0"]
impl crate::Resettable for DT19_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
