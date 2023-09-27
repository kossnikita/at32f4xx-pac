#[doc = "Register `BPR%sDT` reader"]
pub type R = crate::R<BPRDT_SPEC>;
#[doc = "Register `BPR%sDT` writer"]
pub type W = crate::W<BPRDT_SPEC>;
#[doc = "Field `DT` reader - Battery powered domain data"]
pub type DT_R = crate::FieldReader<u32>;
#[doc = "Field `DT` writer - Battery powered domain data"]
pub type DT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Battery powered domain data"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Battery powered domain data"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<BPRDT_SPEC, 0> {
        DT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bprdt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bprdt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BPRDT_SPEC;
impl crate::RegisterSpec for BPRDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bprdt::R`](R) reader structure"]
impl crate::Readable for BPRDT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bprdt::W`](W) writer structure"]
impl crate::Writable for BPRDT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BPR%sDT to value 0"]
impl crate::Resettable for BPRDT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
