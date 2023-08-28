#[doc = "Register `DT25` reader"]
pub type R = crate::R<DT25_SPEC>;
#[doc = "Register `DT25` writer"]
pub type W = crate::W<DT25_SPEC>;
#[doc = "Field `DT25` reader - BPR data25"]
pub type DT25_R = crate::FieldReader<u16>;
#[doc = "Field `DT25` writer - BPR data25"]
pub type DT25_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data25"]
    #[inline(always)]
    pub fn dt25(&self) -> DT25_R {
        DT25_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data25"]
    #[inline(always)]
    #[must_use]
    pub fn dt25(&mut self) -> DT25_W<DT25_SPEC, 0> {
        DT25_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT25_SPEC;
impl crate::RegisterSpec for DT25_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt25::R`](R) reader structure"]
impl crate::Readable for DT25_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt25::W`](W) writer structure"]
impl crate::Writable for DT25_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT25 to value 0"]
impl crate::Resettable for DT25_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
