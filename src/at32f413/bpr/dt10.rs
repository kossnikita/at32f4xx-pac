#[doc = "Register `DT10` reader"]
pub type R = crate::R<DT10_SPEC>;
#[doc = "Register `DT10` writer"]
pub type W = crate::W<DT10_SPEC>;
#[doc = "Field `DT10` reader - BPR data10"]
pub type DT10_R = crate::FieldReader<u16>;
#[doc = "Field `DT10` writer - BPR data10"]
pub type DT10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data10"]
    #[inline(always)]
    pub fn dt10(&self) -> DT10_R {
        DT10_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data10"]
    #[inline(always)]
    #[must_use]
    pub fn dt10(&mut self) -> DT10_W<DT10_SPEC, 0> {
        DT10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT10_SPEC;
impl crate::RegisterSpec for DT10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt10::R`](R) reader structure"]
impl crate::Readable for DT10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt10::W`](W) writer structure"]
impl crate::Writable for DT10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT10 to value 0"]
impl crate::Resettable for DT10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
