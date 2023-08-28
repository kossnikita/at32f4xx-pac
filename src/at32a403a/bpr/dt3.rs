#[doc = "Register `DT3` reader"]
pub type R = crate::R<DT3_SPEC>;
#[doc = "Register `DT3` writer"]
pub type W = crate::W<DT3_SPEC>;
#[doc = "Field `DT3` reader - BPR data3"]
pub type DT3_R = crate::FieldReader<u16>;
#[doc = "Field `DT3` writer - BPR data3"]
pub type DT3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data3"]
    #[inline(always)]
    pub fn dt3(&self) -> DT3_R {
        DT3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data3"]
    #[inline(always)]
    #[must_use]
    pub fn dt3(&mut self) -> DT3_W<DT3_SPEC, 0> {
        DT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT3_SPEC;
impl crate::RegisterSpec for DT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt3::R`](R) reader structure"]
impl crate::Readable for DT3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt3::W`](W) writer structure"]
impl crate::Writable for DT3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT3 to value 0"]
impl crate::Resettable for DT3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
