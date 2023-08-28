#[doc = "Register `DT1` reader"]
pub type R = crate::R<DT1_SPEC>;
#[doc = "Register `DT1` writer"]
pub type W = crate::W<DT1_SPEC>;
#[doc = "Field `DT1` reader - BPR data1"]
pub type DT1_R = crate::FieldReader<u16>;
#[doc = "Field `DT1` writer - BPR data1"]
pub type DT1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data1"]
    #[inline(always)]
    pub fn dt1(&self) -> DT1_R {
        DT1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data1"]
    #[inline(always)]
    #[must_use]
    pub fn dt1(&mut self) -> DT1_W<DT1_SPEC, 0> {
        DT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT1_SPEC;
impl crate::RegisterSpec for DT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt1::R`](R) reader structure"]
impl crate::Readable for DT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt1::W`](W) writer structure"]
impl crate::Writable for DT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT1 to value 0"]
impl crate::Resettable for DT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
