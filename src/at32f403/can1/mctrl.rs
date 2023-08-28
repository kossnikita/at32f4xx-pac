#[doc = "Register `MCTRL` reader"]
pub type R = crate::R<MCTRL_SPEC>;
#[doc = "Register `MCTRL` writer"]
pub type W = crate::W<MCTRL_SPEC>;
#[doc = "Field `FZEN` reader - Freeze mode enable"]
pub type FZEN_R = crate::BitReader;
#[doc = "Field `FZEN` writer - Freeze mode enable"]
pub type FZEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DZEN` reader - Doze mode enable"]
pub type DZEN_R = crate::BitReader;
#[doc = "Field `DZEN` writer - Doze mode enable"]
pub type DZEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MMSSR` reader - Multiple message sending sequence rule"]
pub type MMSSR_R = crate::BitReader;
#[doc = "Field `MMSSR` writer - Multiple message sending sequence rule"]
pub type MMSSR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MDRSEL` reader - Message discarding rule select when overflow"]
pub type MDRSEL_R = crate::BitReader;
#[doc = "Field `MDRSEL` writer - Message discarding rule select when overflow"]
pub type MDRSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRSFEN` reader - Prohibit retransmission when sending fails enable"]
pub type PRSFEN_R = crate::BitReader;
#[doc = "Field `PRSFEN` writer - Prohibit retransmission when sending fails enable"]
pub type PRSFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AEDEN` reader - Automatic exit doze mode enable"]
pub type AEDEN_R = crate::BitReader;
#[doc = "Field `AEDEN` writer - Automatic exit doze mode enable"]
pub type AEDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AEBOEN` reader - Automatic exit bus-off enable"]
pub type AEBOEN_R = crate::BitReader;
#[doc = "Field `AEBOEN` writer - Automatic exit bus-off enable"]
pub type AEBOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TTCEN` reader - Time triggered communication mode enable"]
pub type TTCEN_R = crate::BitReader;
#[doc = "Field `TTCEN` writer - Time triggered communication mode enable"]
pub type TTCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPRST` reader - Software partial reset"]
pub type SPRST_R = crate::BitReader;
#[doc = "Field `SPRST` writer - Software partial reset"]
pub type SPRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTD` reader - Prohibit transmission when debug"]
pub type PTD_R = crate::BitReader;
#[doc = "Field `PTD` writer - Prohibit transmission when debug"]
pub type PTD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Freeze mode enable"]
    #[inline(always)]
    pub fn fzen(&self) -> FZEN_R {
        FZEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Doze mode enable"]
    #[inline(always)]
    pub fn dzen(&self) -> DZEN_R {
        DZEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Multiple message sending sequence rule"]
    #[inline(always)]
    pub fn mmssr(&self) -> MMSSR_R {
        MMSSR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Message discarding rule select when overflow"]
    #[inline(always)]
    pub fn mdrsel(&self) -> MDRSEL_R {
        MDRSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Prohibit retransmission when sending fails enable"]
    #[inline(always)]
    pub fn prsfen(&self) -> PRSFEN_R {
        PRSFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic exit doze mode enable"]
    #[inline(always)]
    pub fn aeden(&self) -> AEDEN_R {
        AEDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic exit bus-off enable"]
    #[inline(always)]
    pub fn aeboen(&self) -> AEBOEN_R {
        AEBOEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Time triggered communication mode enable"]
    #[inline(always)]
    pub fn ttcen(&self) -> TTCEN_R {
        TTCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Software partial reset"]
    #[inline(always)]
    pub fn sprst(&self) -> SPRST_R {
        SPRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Prohibit transmission when debug"]
    #[inline(always)]
    pub fn ptd(&self) -> PTD_R {
        PTD_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Freeze mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn fzen(&mut self) -> FZEN_W<MCTRL_SPEC, 0> {
        FZEN_W::new(self)
    }
    #[doc = "Bit 1 - Doze mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn dzen(&mut self) -> DZEN_W<MCTRL_SPEC, 1> {
        DZEN_W::new(self)
    }
    #[doc = "Bit 2 - Multiple message sending sequence rule"]
    #[inline(always)]
    #[must_use]
    pub fn mmssr(&mut self) -> MMSSR_W<MCTRL_SPEC, 2> {
        MMSSR_W::new(self)
    }
    #[doc = "Bit 3 - Message discarding rule select when overflow"]
    #[inline(always)]
    #[must_use]
    pub fn mdrsel(&mut self) -> MDRSEL_W<MCTRL_SPEC, 3> {
        MDRSEL_W::new(self)
    }
    #[doc = "Bit 4 - Prohibit retransmission when sending fails enable"]
    #[inline(always)]
    #[must_use]
    pub fn prsfen(&mut self) -> PRSFEN_W<MCTRL_SPEC, 4> {
        PRSFEN_W::new(self)
    }
    #[doc = "Bit 5 - Automatic exit doze mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn aeden(&mut self) -> AEDEN_W<MCTRL_SPEC, 5> {
        AEDEN_W::new(self)
    }
    #[doc = "Bit 6 - Automatic exit bus-off enable"]
    #[inline(always)]
    #[must_use]
    pub fn aeboen(&mut self) -> AEBOEN_W<MCTRL_SPEC, 6> {
        AEBOEN_W::new(self)
    }
    #[doc = "Bit 7 - Time triggered communication mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn ttcen(&mut self) -> TTCEN_W<MCTRL_SPEC, 7> {
        TTCEN_W::new(self)
    }
    #[doc = "Bit 15 - Software partial reset"]
    #[inline(always)]
    #[must_use]
    pub fn sprst(&mut self) -> SPRST_W<MCTRL_SPEC, 15> {
        SPRST_W::new(self)
    }
    #[doc = "Bit 16 - Prohibit transmission when debug"]
    #[inline(always)]
    #[must_use]
    pub fn ptd(&mut self) -> PTD_W<MCTRL_SPEC, 16> {
        PTD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Main control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCTRL_SPEC;
impl crate::RegisterSpec for MCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mctrl::R`](R) reader structure"]
impl crate::Readable for MCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mctrl::W`](W) writer structure"]
impl crate::Writable for MCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTRL to value 0x0001_0002"]
impl crate::Resettable for MCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0002;
}
