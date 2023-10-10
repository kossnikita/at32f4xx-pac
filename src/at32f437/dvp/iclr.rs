#[doc = "Register `ICLR` writer"]
pub type W = crate::W<ICLR_SPEC>;
#[doc = "Field `CFDIC` writer - Capture frame done interrupt clear"]
pub type CFDIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVRIC` writer - Data FIFO overrun interrupt clear"]
pub type OVRIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ESEIC` writer - Embedded synchronization error interrupt clear"]
pub type ESEIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VSIC` writer - Vertical synchronization interrupt clear"]
pub type VSIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSIC` writer - Horizontal synchronization interrupt clear"]
pub type HSIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl core::fmt::Debug for crate::generic::Reg<ICLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Capture frame done interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cfdic(&mut self) -> CFDIC_W<ICLR_SPEC, 0> {
        CFDIC_W::new(self)
    }
    #[doc = "Bit 1 - Data FIFO overrun interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovric(&mut self) -> OVRIC_W<ICLR_SPEC, 1> {
        OVRIC_W::new(self)
    }
    #[doc = "Bit 2 - Embedded synchronization error interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn eseic(&mut self) -> ESEIC_W<ICLR_SPEC, 2> {
        ESEIC_W::new(self)
    }
    #[doc = "Bit 3 - Vertical synchronization interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn vsic(&mut self) -> VSIC_W<ICLR_SPEC, 3> {
        VSIC_W::new(self)
    }
    #[doc = "Bit 4 - Horizontal synchronization interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsic(&mut self) -> HSIC_W<ICLR_SPEC, 4> {
        HSIC_W::new(self)
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
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICLR_SPEC;
impl crate::RegisterSpec for ICLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iclr::W`](W) writer structure"]
impl crate::Writable for ICLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for ICLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
