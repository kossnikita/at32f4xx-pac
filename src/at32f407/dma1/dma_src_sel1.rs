#[doc = "Register `DMA_SRC_SEL1` reader"]
pub type R = crate::R<DMA_SRC_SEL1_SPEC>;
#[doc = "Register `DMA_SRC_SEL1` writer"]
pub type W = crate::W<DMA_SRC_SEL1_SPEC>;
#[doc = "Field `CH5_SRC` reader - CH5 SRC select"]
pub type CH5_SRC_R = crate::FieldReader;
#[doc = "Field `CH5_SRC` writer - CH5 SRC select"]
pub type CH5_SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CH6_SRC` reader - CH6 SRC select"]
pub type CH6_SRC_R = crate::FieldReader;
#[doc = "Field `CH6_SRC` writer - CH6 SRC select"]
pub type CH6_SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CH7_SRC` reader - CH7 SRC select"]
pub type CH7_SRC_R = crate::FieldReader;
#[doc = "Field `CH7_SRC` writer - CH7 SRC select"]
pub type CH7_SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DMA_FLEX_EN` reader - DMA FLEX Enable"]
pub type DMA_FLEX_EN_R = crate::BitReader;
#[doc = "Field `DMA_FLEX_EN` writer - DMA FLEX Enable"]
pub type DMA_FLEX_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - CH5 SRC select"]
    #[inline(always)]
    pub fn ch5_src(&self) -> CH5_SRC_R {
        CH5_SRC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CH6 SRC select"]
    #[inline(always)]
    pub fn ch6_src(&self) -> CH6_SRC_R {
        CH6_SRC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CH7 SRC select"]
    #[inline(always)]
    pub fn ch7_src(&self) -> CH7_SRC_R {
        CH7_SRC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - DMA FLEX Enable"]
    #[inline(always)]
    pub fn dma_flex_en(&self) -> DMA_FLEX_EN_R {
        DMA_FLEX_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - CH5 SRC select"]
    #[inline(always)]
    #[must_use]
    pub fn ch5_src(&mut self) -> CH5_SRC_W<DMA_SRC_SEL1_SPEC, 0> {
        CH5_SRC_W::new(self)
    }
    #[doc = "Bits 8:15 - CH6 SRC select"]
    #[inline(always)]
    #[must_use]
    pub fn ch6_src(&mut self) -> CH6_SRC_W<DMA_SRC_SEL1_SPEC, 8> {
        CH6_SRC_W::new(self)
    }
    #[doc = "Bits 16:23 - CH7 SRC select"]
    #[inline(always)]
    #[must_use]
    pub fn ch7_src(&mut self) -> CH7_SRC_W<DMA_SRC_SEL1_SPEC, 16> {
        CH7_SRC_W::new(self)
    }
    #[doc = "Bit 24 - DMA FLEX Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_flex_en(&mut self) -> DMA_FLEX_EN_W<DMA_SRC_SEL1_SPEC, 24> {
        DMA_FLEX_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA channel source assignment register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_src_sel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_src_sel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_SRC_SEL1_SPEC;
impl crate::RegisterSpec for DMA_SRC_SEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_src_sel1::R`](R) reader structure"]
impl crate::Readable for DMA_SRC_SEL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_src_sel1::W`](W) writer structure"]
impl crate::Writable for DMA_SRC_SEL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_SRC_SEL1 to value 0"]
impl crate::Resettable for DMA_SRC_SEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
