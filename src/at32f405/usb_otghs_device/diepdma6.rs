#[doc = "Register `DIEPDMA6` reader"]
pub type R = crate::R<DIEPDMA6_SPEC>;
#[doc = "Register `DIEPDMA6` writer"]
pub type W = crate::W<DIEPDMA6_SPEC>;
#[doc = "Field `DMAADDR` reader - DMA Address"]
pub type DMAADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - DMA Address"]
pub type DMAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPDMA6")
            .field("dmaaddr", &format_args!("{}", self.dmaaddr().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DIEPDMA6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<DIEPDMA6_SPEC> {
        DMAADDR_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device IN Endpoint 6 DMA Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPDMA6_SPEC;
impl crate::RegisterSpec for DIEPDMA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdma6::R`](R) reader structure"]
impl crate::Readable for DIEPDMA6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepdma6::W`](W) writer structure"]
impl crate::Writable for DIEPDMA6_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPDMA6 to value 0"]
impl crate::Resettable for DIEPDMA6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
