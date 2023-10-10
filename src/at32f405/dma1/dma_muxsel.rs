#[doc = "Register `DMA_MUXSEL` reader"]
pub type R = crate::R<DMA_MUXSEL_SPEC>;
#[doc = "Register `DMA_MUXSEL` writer"]
pub type W = crate::W<DMA_MUXSEL_SPEC>;
#[doc = "Field `TBL_SEL` reader - Multiplexer Table Select"]
pub type TBL_SEL_R = crate::BitReader;
#[doc = "Field `TBL_SEL` writer - Multiplexer Table Select"]
pub type TBL_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Multiplexer Table Select"]
    #[inline(always)]
    pub fn tbl_sel(&self) -> TBL_SEL_R {
        TBL_SEL_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_MUXSEL")
            .field("tbl_sel", &format_args!("{}", self.tbl_sel().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DMA_MUXSEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Multiplexer Table Select"]
    #[inline(always)]
    #[must_use]
    pub fn tbl_sel(&mut self) -> TBL_SEL_W<DMA_MUXSEL_SPEC, 0> {
        TBL_SEL_W::new(self)
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
#[doc = "DMAMUX Table Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_muxsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_muxsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_MUXSEL_SPEC;
impl crate::RegisterSpec for DMA_MUXSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_muxsel::R`](R) reader structure"]
impl crate::Readable for DMA_MUXSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_muxsel::W`](W) writer structure"]
impl crate::Writable for DMA_MUXSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_MUXSEL to value 0"]
impl crate::Resettable for DMA_MUXSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
