#[doc = "Register `DT29` reader"]
pub type R = crate::R<DT29_SPEC>;
#[doc = "Register `DT29` writer"]
pub type W = crate::W<DT29_SPEC>;
#[doc = "Field `DT29` reader - BPR data29"]
pub type DT29_R = crate::FieldReader<u16>;
#[doc = "Field `DT29` writer - BPR data29"]
pub type DT29_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data29"]
    #[inline(always)]
    pub fn dt29(&self) -> DT29_R {
        DT29_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data29"]
    #[inline(always)]
    #[must_use]
    pub fn dt29(&mut self) -> DT29_W<DT29_SPEC, 0> {
        DT29_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT29_SPEC;
impl crate::RegisterSpec for DT29_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt29::R`](R) reader structure"]
impl crate::Readable for DT29_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt29::W`](W) writer structure"]
impl crate::Writable for DT29_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT29 to value 0"]
impl crate::Resettable for DT29_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
