#[doc = "Register `DT12` reader"]
pub type R = crate::R<DT12_SPEC>;
#[doc = "Register `DT12` writer"]
pub type W = crate::W<DT12_SPEC>;
#[doc = "Field `DT12` reader - BPR data12"]
pub type DT12_R = crate::FieldReader<u16>;
#[doc = "Field `DT12` writer - BPR data12"]
pub type DT12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data12"]
    #[inline(always)]
    pub fn dt12(&self) -> DT12_R {
        DT12_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data12"]
    #[inline(always)]
    #[must_use]
    pub fn dt12(&mut self) -> DT12_W<DT12_SPEC, 0> {
        DT12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT12_SPEC;
impl crate::RegisterSpec for DT12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt12::R`](R) reader structure"]
impl crate::Readable for DT12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt12::W`](W) writer structure"]
impl crate::Writable for DT12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT12 to value 0"]
impl crate::Resettable for DT12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
