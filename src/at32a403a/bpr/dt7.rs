#[doc = "Register `DT7` reader"]
pub type R = crate::R<DT7_SPEC>;
#[doc = "Register `DT7` writer"]
pub type W = crate::W<DT7_SPEC>;
#[doc = "Field `DT7` reader - BPR data7"]
pub type DT7_R = crate::FieldReader<u16>;
#[doc = "Field `DT7` writer - BPR data7"]
pub type DT7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data7"]
    #[inline(always)]
    pub fn dt7(&self) -> DT7_R {
        DT7_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data7"]
    #[inline(always)]
    #[must_use]
    pub fn dt7(&mut self) -> DT7_W<DT7_SPEC, 0> {
        DT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT7_SPEC;
impl crate::RegisterSpec for DT7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt7::R`](R) reader structure"]
impl crate::Readable for DT7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt7::W`](W) writer structure"]
impl crate::Writable for DT7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT7 to value 0"]
impl crate::Resettable for DT7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
