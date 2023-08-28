#[doc = "Register `BPR17DT` reader"]
pub type R = crate::R<BPR17DT_SPEC>;
#[doc = "Register `BPR17DT` writer"]
pub type W = crate::W<BPR17DT_SPEC>;
#[doc = "Field `DT` reader - Battery powered domain data"]
pub type DT_R = crate::FieldReader<u32>;
#[doc = "Field `DT` writer - Battery powered domain data"]
pub type DT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
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
    pub fn dt(&mut self) -> DT_W<BPR17DT_SPEC, 0> {
        DT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr17dt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr17dt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BPR17DT_SPEC;
impl crate::RegisterSpec for BPR17DT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bpr17dt::R`](R) reader structure"]
impl crate::Readable for BPR17DT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bpr17dt::W`](W) writer structure"]
impl crate::Writable for BPR17DT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BPR17DT to value 0"]
impl crate::Resettable for BPR17DT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}