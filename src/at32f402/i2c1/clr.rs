#[doc = "Register `CLR` writer"]
pub type W = crate::W<CLR_SPEC>;
#[doc = "Field `ADDRC` writer - Clear 0~7 bit address match flag"]
pub type ADDRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKFAILC` writer - Clear acknowledge failure flag"]
pub type ACKFAILC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPC` writer - Clear stop condition generation complete flag"]
pub type STOPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERRC` writer - Clear bus error flag"]
pub type BUSERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARLOSTC` writer - Clear arbitration lost flag"]
pub type ARLOSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUFC` writer - Clear overload / underload flag"]
pub type OUFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECERRC` writer - Clear PEC receive error flag"]
pub type PECERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMOUTC` writer - Clear SMBus timeout flag"]
pub type TMOUTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERTC` writer - Clear SMBus alert flag"]
pub type ALERTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 3 - Clear 0~7 bit address match flag"]
    #[inline(always)]
    #[must_use]
    pub fn addrc(&mut self) -> ADDRC_W<CLR_SPEC> {
        ADDRC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear acknowledge failure flag"]
    #[inline(always)]
    #[must_use]
    pub fn ackfailc(&mut self) -> ACKFAILC_W<CLR_SPEC> {
        ACKFAILC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear stop condition generation complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn stopc(&mut self) -> STOPC_W<CLR_SPEC> {
        STOPC_W::new(self, 5)
    }
    #[doc = "Bit 8 - Clear bus error flag"]
    #[inline(always)]
    #[must_use]
    pub fn buserrc(&mut self) -> BUSERRC_W<CLR_SPEC> {
        BUSERRC_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear arbitration lost flag"]
    #[inline(always)]
    #[must_use]
    pub fn arlostc(&mut self) -> ARLOSTC_W<CLR_SPEC> {
        ARLOSTC_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear overload / underload flag"]
    #[inline(always)]
    #[must_use]
    pub fn oufc(&mut self) -> OUFC_W<CLR_SPEC> {
        OUFC_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear PEC receive error flag"]
    #[inline(always)]
    #[must_use]
    pub fn pecerrc(&mut self) -> PECERRC_W<CLR_SPEC> {
        PECERRC_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear SMBus timeout flag"]
    #[inline(always)]
    #[must_use]
    pub fn tmoutc(&mut self) -> TMOUTC_W<CLR_SPEC> {
        TMOUTC_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear SMBus alert flag"]
    #[inline(always)]
    #[must_use]
    pub fn alertc(&mut self) -> ALERTC_W<CLR_SPEC> {
        ALERTC_W::new(self, 13)
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
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for CLR_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
