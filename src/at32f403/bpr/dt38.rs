#[doc = "Register `DT38` reader"]
pub type R = crate::R<DT38_SPEC>;
#[doc = "Register `DT38` writer"]
pub type W = crate::W<DT38_SPEC>;
#[doc = "Field `DT38` reader - BPR data38"]
pub type DT38_R = crate::FieldReader<u16>;
#[doc = "Field `DT38` writer - BPR data38"]
pub type DT38_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data38"]
    #[inline(always)]
    pub fn dt38(&self) -> DT38_R {
        DT38_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data38"]
    #[inline(always)]
    #[must_use]
    pub fn dt38(&mut self) -> DT38_W<DT38_SPEC, 0> {
        DT38_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt38::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt38::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT38_SPEC;
impl crate::RegisterSpec for DT38_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt38::R`](R) reader structure"]
impl crate::Readable for DT38_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt38::W`](W) writer structure"]
impl crate::Writable for DT38_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT38 to value 0"]
impl crate::Resettable for DT38_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
