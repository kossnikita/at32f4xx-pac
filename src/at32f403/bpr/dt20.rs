#[doc = "Register `DT20` reader"]
pub type R = crate::R<DT20_SPEC>;
#[doc = "Register `DT20` writer"]
pub type W = crate::W<DT20_SPEC>;
#[doc = "Field `DT20` reader - BPR data20"]
pub type DT20_R = crate::FieldReader<u16>;
#[doc = "Field `DT20` writer - BPR data20"]
pub type DT20_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data20"]
    #[inline(always)]
    pub fn dt20(&self) -> DT20_R {
        DT20_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data20"]
    #[inline(always)]
    #[must_use]
    pub fn dt20(&mut self) -> DT20_W<DT20_SPEC, 0> {
        DT20_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT20_SPEC;
impl crate::RegisterSpec for DT20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt20::R`](R) reader structure"]
impl crate::Readable for DT20_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt20::W`](W) writer structure"]
impl crate::Writable for DT20_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT20 to value 0"]
impl crate::Resettable for DT20_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
