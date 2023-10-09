#[doc = "Register `CLR` writer"]
pub type W = crate::W<CLR_SPEC>;
#[doc = "Field `ADDRC` writer - Clear 0~7 bit address match flag"]
pub type ADDRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACKFAILC` writer - Clear acknowledge failure flag"]
pub type ACKFAILC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STOPC` writer - Clear stop condition generation complete flag"]
pub type STOPC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUSERRC` writer - Clear bus error flag"]
pub type BUSERRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ARLOSTC` writer - Clear arbitration lost flag"]
pub type ARLOSTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUFC` writer - Clear overload / underload flag"]
pub type OUFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PECERRC` writer - Clear PEC receive error flag"]
pub type PECERRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMOUTC` writer - Clear SMBus timeout flag"]
pub type TMOUTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALERTC` writer - Clear SMBus alert flag"]
pub type ALERTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl core::fmt::Debug for crate::generic::Reg<CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 3 - Clear 0~7 bit address match flag"]
    #[inline(always)]
    #[must_use]
    pub fn addrc(&mut self) -> ADDRC_W<CLR_SPEC, 3> {
        ADDRC_W::new(self)
    }
    #[doc = "Bit 4 - Clear acknowledge failure flag"]
    #[inline(always)]
    #[must_use]
    pub fn ackfailc(&mut self) -> ACKFAILC_W<CLR_SPEC, 4> {
        ACKFAILC_W::new(self)
    }
    #[doc = "Bit 5 - Clear stop condition generation complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn stopc(&mut self) -> STOPC_W<CLR_SPEC, 5> {
        STOPC_W::new(self)
    }
    #[doc = "Bit 8 - Clear bus error flag"]
    #[inline(always)]
    #[must_use]
    pub fn buserrc(&mut self) -> BUSERRC_W<CLR_SPEC, 8> {
        BUSERRC_W::new(self)
    }
    #[doc = "Bit 9 - Clear arbitration lost flag"]
    #[inline(always)]
    #[must_use]
    pub fn arlostc(&mut self) -> ARLOSTC_W<CLR_SPEC, 9> {
        ARLOSTC_W::new(self)
    }
    #[doc = "Bit 10 - Clear overload / underload flag"]
    #[inline(always)]
    #[must_use]
    pub fn oufc(&mut self) -> OUFC_W<CLR_SPEC, 10> {
        OUFC_W::new(self)
    }
    #[doc = "Bit 11 - Clear PEC receive error flag"]
    #[inline(always)]
    #[must_use]
    pub fn pecerrc(&mut self) -> PECERRC_W<CLR_SPEC, 11> {
        PECERRC_W::new(self)
    }
    #[doc = "Bit 12 - Clear SMBus timeout flag"]
    #[inline(always)]
    #[must_use]
    pub fn tmoutc(&mut self) -> TMOUTC_W<CLR_SPEC, 12> {
        TMOUTC_W::new(self)
    }
    #[doc = "Bit 13 - Clear SMBus alert flag"]
    #[inline(always)]
    #[must_use]
    pub fn alertc(&mut self) -> ALERTC_W<CLR_SPEC, 13> {
        ALERTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
