#[doc = "Register `DT33` reader"]
pub type R = crate::R<DT33_SPEC>;
#[doc = "Register `DT33` writer"]
pub type W = crate::W<DT33_SPEC>;
#[doc = "Field `DT33` reader - BPR data33"]
pub type DT33_R = crate::FieldReader<u16>;
#[doc = "Field `DT33` writer - BPR data33"]
pub type DT33_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data33"]
    #[inline(always)]
    pub fn dt33(&self) -> DT33_R {
        DT33_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data33"]
    #[inline(always)]
    #[must_use]
    pub fn dt33(&mut self) -> DT33_W<DT33_SPEC, 0> {
        DT33_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt33::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt33::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT33_SPEC;
impl crate::RegisterSpec for DT33_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt33::R`](R) reader structure"]
impl crate::Readable for DT33_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt33::W`](W) writer structure"]
impl crate::Writable for DT33_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT33 to value 0"]
impl crate::Resettable for DT33_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
