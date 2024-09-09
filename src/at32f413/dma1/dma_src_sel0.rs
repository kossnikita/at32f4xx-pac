#[doc = "Register `DMA_SRC_SEL0` reader"]
pub type R = crate::R<DMA_SRC_SEL0_SPEC>;
#[doc = "Register `DMA_SRC_SEL0` writer"]
pub type W = crate::W<DMA_SRC_SEL0_SPEC>;
#[doc = "Field `CH1_SRC` reader - CH1 SRC select"]
pub type CH1_SRC_R = crate::FieldReader;
#[doc = "Field `CH1_SRC` writer - CH1 SRC select"]
pub type CH1_SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CH2_SRC` reader - CH2 SRC select"]
pub type CH2_SRC_R = crate::FieldReader;
#[doc = "Field `CH2_SRC` writer - CH2 SRC select"]
pub type CH2_SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CH3_SRC` reader - CH3 SRC select"]
pub type CH3_SRC_R = crate::FieldReader;
#[doc = "Field `CH3_SRC` writer - CH3 SRC select"]
pub type CH3_SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CH4_SRC` reader - CH4 SRC select"]
pub type CH4_SRC_R = crate::FieldReader;
#[doc = "Field `CH4_SRC` writer - CH4 SRC select"]
pub type CH4_SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CH1 SRC select"]
    #[inline(always)]
    pub fn ch1_src(&self) -> CH1_SRC_R {
        CH1_SRC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CH2 SRC select"]
    #[inline(always)]
    pub fn ch2_src(&self) -> CH2_SRC_R {
        CH2_SRC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CH3 SRC select"]
    #[inline(always)]
    pub fn ch3_src(&self) -> CH3_SRC_R {
        CH3_SRC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CH4 SRC select"]
    #[inline(always)]
    pub fn ch4_src(&self) -> CH4_SRC_R {
        CH4_SRC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_SRC_SEL0")
            .field("ch1_src", &self.ch1_src())
            .field("ch2_src", &self.ch2_src())
            .field("ch3_src", &self.ch3_src())
            .field("ch4_src", &self.ch4_src())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - CH1 SRC select"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_src(&mut self) -> CH1_SRC_W<DMA_SRC_SEL0_SPEC> {
        CH1_SRC_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - CH2 SRC select"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_src(&mut self) -> CH2_SRC_W<DMA_SRC_SEL0_SPEC> {
        CH2_SRC_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - CH3 SRC select"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_src(&mut self) -> CH3_SRC_W<DMA_SRC_SEL0_SPEC> {
        CH3_SRC_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - CH4 SRC select"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_src(&mut self) -> CH4_SRC_W<DMA_SRC_SEL0_SPEC> {
        CH4_SRC_W::new(self, 24)
    }
}
#[doc = "DMA channel source assignment register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_src_sel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_src_sel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_SRC_SEL0_SPEC;
impl crate::RegisterSpec for DMA_SRC_SEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_src_sel0::R`](R) reader structure"]
impl crate::Readable for DMA_SRC_SEL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_src_sel0::W`](W) writer structure"]
impl crate::Writable for DMA_SRC_SEL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_SRC_SEL0 to value 0"]
impl crate::Resettable for DMA_SRC_SEL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
