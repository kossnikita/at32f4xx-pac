#[doc = "Register `DT40` reader"]
pub type R = crate::R<DT40_SPEC>;
#[doc = "Register `DT40` writer"]
pub type W = crate::W<DT40_SPEC>;
#[doc = "Field `DT40` reader - BPR data40"]
pub type DT40_R = crate::FieldReader<u16>;
#[doc = "Field `DT40` writer - BPR data40"]
pub type DT40_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data40"]
    #[inline(always)]
    pub fn dt40(&self) -> DT40_R {
        DT40_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data40"]
    #[inline(always)]
    #[must_use]
    pub fn dt40(&mut self) -> DT40_W<DT40_SPEC, 0> {
        DT40_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt40::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt40::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT40_SPEC;
impl crate::RegisterSpec for DT40_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt40::R`](R) reader structure"]
impl crate::Readable for DT40_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt40::W`](W) writer structure"]
impl crate::Writable for DT40_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT40 to value 0"]
impl crate::Resettable for DT40_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
