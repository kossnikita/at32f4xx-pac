#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `TCIEN` reader - Transmission complete interrupt enable"]
pub type TCIEN_R = crate::BitReader;
#[doc = "Field `TCIEN` writer - Transmission complete interrupt enable"]
pub type TCIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF0MIEN` reader - FIFO 0 receive message interrupt enable"]
pub type RF0MIEN_R = crate::BitReader;
#[doc = "Field `RF0MIEN` writer - FIFO 0 receive message interrupt enable"]
pub type RF0MIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF0FIEN` reader - Receive FIFO 0 full interrupt enable"]
pub type RF0FIEN_R = crate::BitReader;
#[doc = "Field `RF0FIEN` writer - Receive FIFO 0 full interrupt enable"]
pub type RF0FIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF0OIEN` reader - Receive FIFO 0 overflow interrupt enable"]
pub type RF0OIEN_R = crate::BitReader;
#[doc = "Field `RF0OIEN` writer - Receive FIFO 0 overflow interrupt enable"]
pub type RF0OIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF1MIEN` reader - FIFO 1 receive message interrupt enable"]
pub type RF1MIEN_R = crate::BitReader;
#[doc = "Field `RF1MIEN` writer - FIFO 1 receive message interrupt enable"]
pub type RF1MIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF1FIEN` reader - Receive FIFO 1 full interrupt enable"]
pub type RF1FIEN_R = crate::BitReader;
#[doc = "Field `RF1FIEN` writer - Receive FIFO 1 full interrupt enable"]
pub type RF1FIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RF1OIEN` reader - Receive FIFO 1 overflow interrupt enable"]
pub type RF1OIEN_R = crate::BitReader;
#[doc = "Field `RF1OIEN` writer - Receive FIFO 1 overflow interrupt enable"]
pub type RF1OIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EAIEN` reader - Error active interrupt enable"]
pub type EAIEN_R = crate::BitReader;
#[doc = "Field `EAIEN` writer - Error active interrupt enable"]
pub type EAIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPIEN` reader - Error passive interrupt enable"]
pub type EPIEN_R = crate::BitReader;
#[doc = "Field `EPIEN` writer - Error passive interrupt enable"]
pub type EPIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BOIEN` reader - Bus-off interrupt enable"]
pub type BOIEN_R = crate::BitReader;
#[doc = "Field `BOIEN` writer - Bus-off interrupt enable"]
pub type BOIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETRIEN` reader - Error type record interrupt enable"]
pub type ETRIEN_R = crate::BitReader;
#[doc = "Field `ETRIEN` writer - Error type record interrupt enable"]
pub type ETRIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOIEN` reader - Error occur interrupt enable"]
pub type EOIEN_R = crate::BitReader;
#[doc = "Field `EOIEN` writer - Error occur interrupt enable"]
pub type EOIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QDZIEN` reader - Quit doze mode interrupt enable"]
pub type QDZIEN_R = crate::BitReader;
#[doc = "Field `QDZIEN` writer - Quit doze mode interrupt enable"]
pub type QDZIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EDZIEN` reader - Enter doze mode interrupt enable"]
pub type EDZIEN_R = crate::BitReader;
#[doc = "Field `EDZIEN` writer - Enter doze mode interrupt enable"]
pub type EDZIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcien(&self) -> TCIEN_R {
        TCIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO 0 receive message interrupt enable"]
    #[inline(always)]
    pub fn rf0mien(&self) -> RF0MIEN_R {
        RF0MIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO 0 full interrupt enable"]
    #[inline(always)]
    pub fn rf0fien(&self) -> RF0FIEN_R {
        RF0FIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO 0 overflow interrupt enable"]
    #[inline(always)]
    pub fn rf0oien(&self) -> RF0OIEN_R {
        RF0OIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO 1 receive message interrupt enable"]
    #[inline(always)]
    pub fn rf1mien(&self) -> RF1MIEN_R {
        RF1MIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO 1 full interrupt enable"]
    #[inline(always)]
    pub fn rf1fien(&self) -> RF1FIEN_R {
        RF1FIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO 1 overflow interrupt enable"]
    #[inline(always)]
    pub fn rf1oien(&self) -> RF1OIEN_R {
        RF1OIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Error active interrupt enable"]
    #[inline(always)]
    pub fn eaien(&self) -> EAIEN_R {
        EAIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Error passive interrupt enable"]
    #[inline(always)]
    pub fn epien(&self) -> EPIEN_R {
        EPIEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus-off interrupt enable"]
    #[inline(always)]
    pub fn boien(&self) -> BOIEN_R {
        BOIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Error type record interrupt enable"]
    #[inline(always)]
    pub fn etrien(&self) -> ETRIEN_R {
        ETRIEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Error occur interrupt enable"]
    #[inline(always)]
    pub fn eoien(&self) -> EOIEN_R {
        EOIEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Quit doze mode interrupt enable"]
    #[inline(always)]
    pub fn qdzien(&self) -> QDZIEN_R {
        QDZIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enter doze mode interrupt enable"]
    #[inline(always)]
    pub fn edzien(&self) -> EDZIEN_R {
        EDZIEN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmission complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcien(&mut self) -> TCIEN_W<INTEN_SPEC, 0> {
        TCIEN_W::new(self)
    }
    #[doc = "Bit 1 - FIFO 0 receive message interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0mien(&mut self) -> RF0MIEN_W<INTEN_SPEC, 1> {
        RF0MIEN_W::new(self)
    }
    #[doc = "Bit 2 - Receive FIFO 0 full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0fien(&mut self) -> RF0FIEN_W<INTEN_SPEC, 2> {
        RF0FIEN_W::new(self)
    }
    #[doc = "Bit 3 - Receive FIFO 0 overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0oien(&mut self) -> RF0OIEN_W<INTEN_SPEC, 3> {
        RF0OIEN_W::new(self)
    }
    #[doc = "Bit 4 - FIFO 1 receive message interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1mien(&mut self) -> RF1MIEN_W<INTEN_SPEC, 4> {
        RF1MIEN_W::new(self)
    }
    #[doc = "Bit 5 - Receive FIFO 1 full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1fien(&mut self) -> RF1FIEN_W<INTEN_SPEC, 5> {
        RF1FIEN_W::new(self)
    }
    #[doc = "Bit 6 - Receive FIFO 1 overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1oien(&mut self) -> RF1OIEN_W<INTEN_SPEC, 6> {
        RF1OIEN_W::new(self)
    }
    #[doc = "Bit 8 - Error active interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eaien(&mut self) -> EAIEN_W<INTEN_SPEC, 8> {
        EAIEN_W::new(self)
    }
    #[doc = "Bit 9 - Error passive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn epien(&mut self) -> EPIEN_W<INTEN_SPEC, 9> {
        EPIEN_W::new(self)
    }
    #[doc = "Bit 10 - Bus-off interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn boien(&mut self) -> BOIEN_W<INTEN_SPEC, 10> {
        BOIEN_W::new(self)
    }
    #[doc = "Bit 11 - Error type record interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn etrien(&mut self) -> ETRIEN_W<INTEN_SPEC, 11> {
        ETRIEN_W::new(self)
    }
    #[doc = "Bit 15 - Error occur interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eoien(&mut self) -> EOIEN_W<INTEN_SPEC, 15> {
        EOIEN_W::new(self)
    }
    #[doc = "Bit 16 - Quit doze mode interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn qdzien(&mut self) -> QDZIEN_W<INTEN_SPEC, 16> {
        QDZIEN_W::new(self)
    }
    #[doc = "Bit 17 - Enter doze mode interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn edzien(&mut self) -> EDZIEN_W<INTEN_SPEC, 17> {
        EDZIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
