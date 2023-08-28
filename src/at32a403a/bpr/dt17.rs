#[doc = "Register `DT17` reader"]
pub type R = crate::R<DT17_SPEC>;
#[doc = "Register `DT17` writer"]
pub type W = crate::W<DT17_SPEC>;
#[doc = "Field `DT17` reader - BPR data17"]
pub type DT17_R = crate::FieldReader<u16>;
#[doc = "Field `DT17` writer - BPR data17"]
pub type DT17_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data17"]
    #[inline(always)]
    pub fn dt17(&self) -> DT17_R {
        DT17_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data17"]
    #[inline(always)]
    #[must_use]
    pub fn dt17(&mut self) -> DT17_W<DT17_SPEC, 0> {
        DT17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT17_SPEC;
impl crate::RegisterSpec for DT17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt17::R`](R) reader structure"]
impl crate::Readable for DT17_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt17::W`](W) writer structure"]
impl crate::Writable for DT17_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT17 to value 0"]
impl crate::Resettable for DT17_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
