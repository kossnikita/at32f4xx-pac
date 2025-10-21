#[doc = "Register `ICLR` writer"]
pub type W = crate::W<ICLR_SPEC>;
#[doc = "Field `CFDIC` writer - Capture frame done interrupt clear"]
pub type CFDIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIC` writer - Data FIFO overrun interrupt clear"]
pub type OVRIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESEIC` writer - Embedded synchronization error interrupt clear"]
pub type ESEIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSIC` writer - Vertical synchronization interrupt clear"]
pub type VSIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIC` writer - Horizontal synchronization interrupt clear"]
pub type HSIC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Capture frame done interrupt clear"]
    #[inline(always)]
    pub fn cfdic(&mut self) -> CFDIC_W<'_, ICLR_SPEC> {
        CFDIC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data FIFO overrun interrupt clear"]
    #[inline(always)]
    pub fn ovric(&mut self) -> OVRIC_W<'_, ICLR_SPEC> {
        OVRIC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Embedded synchronization error interrupt clear"]
    #[inline(always)]
    pub fn eseic(&mut self) -> ESEIC_W<'_, ICLR_SPEC> {
        ESEIC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Vertical synchronization interrupt clear"]
    #[inline(always)]
    pub fn vsic(&mut self) -> VSIC_W<'_, ICLR_SPEC> {
        VSIC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Horizontal synchronization interrupt clear"]
    #[inline(always)]
    pub fn hsic(&mut self) -> HSIC_W<'_, ICLR_SPEC> {
        HSIC_W::new(self, 4)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICLR_SPEC;
impl crate::RegisterSpec for ICLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iclr::W`](W) writer structure"]
impl crate::Writable for ICLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for ICLR_SPEC {}
