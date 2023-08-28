#[doc = "Register `DT26` reader"]
pub type R = crate::R<DT26_SPEC>;
#[doc = "Register `DT26` writer"]
pub type W = crate::W<DT26_SPEC>;
#[doc = "Field `DT26` reader - BPR data26"]
pub type DT26_R = crate::FieldReader<u16>;
#[doc = "Field `DT26` writer - BPR data26"]
pub type DT26_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data26"]
    #[inline(always)]
    pub fn dt26(&self) -> DT26_R {
        DT26_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data26"]
    #[inline(always)]
    #[must_use]
    pub fn dt26(&mut self) -> DT26_W<DT26_SPEC, 0> {
        DT26_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt26::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT26_SPEC;
impl crate::RegisterSpec for DT26_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt26::R`](R) reader structure"]
impl crate::Readable for DT26_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt26::W`](W) writer structure"]
impl crate::Writable for DT26_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT26 to value 0"]
impl crate::Resettable for DT26_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
