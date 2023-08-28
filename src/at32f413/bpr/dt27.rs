#[doc = "Register `DT27` reader"]
pub type R = crate::R<DT27_SPEC>;
#[doc = "Register `DT27` writer"]
pub type W = crate::W<DT27_SPEC>;
#[doc = "Field `DT27` reader - BPR data27"]
pub type DT27_R = crate::FieldReader<u16>;
#[doc = "Field `DT27` writer - BPR data27"]
pub type DT27_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data27"]
    #[inline(always)]
    pub fn dt27(&self) -> DT27_R {
        DT27_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data27"]
    #[inline(always)]
    #[must_use]
    pub fn dt27(&mut self) -> DT27_W<DT27_SPEC, 0> {
        DT27_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt27::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt27::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT27_SPEC;
impl crate::RegisterSpec for DT27_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt27::R`](R) reader structure"]
impl crate::Readable for DT27_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt27::W`](W) writer structure"]
impl crate::Writable for DT27_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT27 to value 0"]
impl crate::Resettable for DT27_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
