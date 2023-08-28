#[doc = "Register `BUF` reader"]
pub type R = crate::R<BUF_SPEC>;
#[doc = "Register `BUF` writer"]
pub type W = crate::W<BUF_SPEC>;
#[doc = "Field `DT` reader - FIFOData"]
pub type DT_R = crate::FieldReader<u32>;
#[doc = "Field `DT` writer - FIFOData"]
pub type DT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - FIFOData"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFOData"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<BUF_SPEC, 0> {
        DT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "bits 31:0 = FIFOData: Receive and transmit FIFO data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUF_SPEC;
impl crate::RegisterSpec for BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf::R`](R) reader structure"]
impl crate::Readable for BUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buf::W`](W) writer structure"]
impl crate::Writable for BUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUF to value 0"]
impl crate::Resettable for BUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
