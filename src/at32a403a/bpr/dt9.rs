#[doc = "Register `DT9` reader"]
pub type R = crate::R<DT9_SPEC>;
#[doc = "Register `DT9` writer"]
pub type W = crate::W<DT9_SPEC>;
#[doc = "Field `DT9` reader - BPR data9"]
pub type DT9_R = crate::FieldReader<u16>;
#[doc = "Field `DT9` writer - BPR data9"]
pub type DT9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data9"]
    #[inline(always)]
    pub fn dt9(&self) -> DT9_R {
        DT9_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data9"]
    #[inline(always)]
    #[must_use]
    pub fn dt9(&mut self) -> DT9_W<DT9_SPEC, 0> {
        DT9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT9_SPEC;
impl crate::RegisterSpec for DT9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt9::R`](R) reader structure"]
impl crate::Readable for DT9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt9::W`](W) writer structure"]
impl crate::Writable for DT9_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT9 to value 0"]
impl crate::Resettable for DT9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
