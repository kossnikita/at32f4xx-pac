#[doc = "Register `DT5` reader"]
pub type R = crate::R<DT5_SPEC>;
#[doc = "Register `DT5` writer"]
pub type W = crate::W<DT5_SPEC>;
#[doc = "Field `DT5` reader - BPR data5"]
pub type DT5_R = crate::FieldReader<u16>;
#[doc = "Field `DT5` writer - BPR data5"]
pub type DT5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data5"]
    #[inline(always)]
    pub fn dt5(&self) -> DT5_R {
        DT5_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data5"]
    #[inline(always)]
    #[must_use]
    pub fn dt5(&mut self) -> DT5_W<DT5_SPEC, 0> {
        DT5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT5_SPEC;
impl crate::RegisterSpec for DT5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt5::R`](R) reader structure"]
impl crate::Readable for DT5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt5::W`](W) writer structure"]
impl crate::Writable for DT5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT5 to value 0"]
impl crate::Resettable for DT5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
