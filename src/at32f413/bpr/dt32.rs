#[doc = "Register `DT32` reader"]
pub type R = crate::R<DT32_SPEC>;
#[doc = "Register `DT32` writer"]
pub type W = crate::W<DT32_SPEC>;
#[doc = "Field `DT32` reader - BPR data32"]
pub type DT32_R = crate::FieldReader<u16>;
#[doc = "Field `DT32` writer - BPR data32"]
pub type DT32_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data32"]
    #[inline(always)]
    pub fn dt32(&self) -> DT32_R {
        DT32_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data32"]
    #[inline(always)]
    #[must_use]
    pub fn dt32(&mut self) -> DT32_W<DT32_SPEC, 0> {
        DT32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT32_SPEC;
impl crate::RegisterSpec for DT32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt32::R`](R) reader structure"]
impl crate::Readable for DT32_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt32::W`](W) writer structure"]
impl crate::Writable for DT32_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT32 to value 0"]
impl crate::Resettable for DT32_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
