#[doc = "Register `DT8` reader"]
pub type R = crate::R<DT8_SPEC>;
#[doc = "Register `DT8` writer"]
pub type W = crate::W<DT8_SPEC>;
#[doc = "Field `DT8` reader - BPR data8"]
pub type DT8_R = crate::FieldReader<u16>;
#[doc = "Field `DT8` writer - BPR data8"]
pub type DT8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data8"]
    #[inline(always)]
    pub fn dt8(&self) -> DT8_R {
        DT8_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data8"]
    #[inline(always)]
    #[must_use]
    pub fn dt8(&mut self) -> DT8_W<DT8_SPEC, 0> {
        DT8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT8_SPEC;
impl crate::RegisterSpec for DT8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt8::R`](R) reader structure"]
impl crate::Readable for DT8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt8::W`](W) writer structure"]
impl crate::Writable for DT8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT8 to value 0"]
impl crate::Resettable for DT8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
