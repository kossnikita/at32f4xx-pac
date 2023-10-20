#[doc = "Register `STS1` reader"]
pub type R = crate::R<STS1_SPEC>;
#[doc = "Register `STS1` writer"]
pub type W = crate::W<STS1_SPEC>;
#[doc = "Field `STARTF` reader - Start bit (Master mode)"]
pub type STARTF_R = crate::BitReader;
#[doc = "Field `ADDR7F` reader - Address sent (master mode)/matched (slave mode)"]
pub type ADDR7F_R = crate::BitReader;
#[doc = "Field `TDC` reader - Transmit data complete"]
pub type TDC_R = crate::BitReader;
#[doc = "Field `ADDRHF` reader - address header match (Master mode)"]
pub type ADDRHF_R = crate::BitReader;
#[doc = "Field `STOPF` reader - Stop detection (slave mode)"]
pub type STOPF_R = crate::BitReader;
#[doc = "Field `RDBF` reader - Receive data buffer full (receivers)"]
pub type RDBF_R = crate::BitReader;
#[doc = "Field `TDBE` reader - Transmit data buffer empty (transmitters)"]
pub type TDBE_R = crate::BitReader;
#[doc = "Field `BUSERR` reader - Bus error"]
pub type BUSERR_R = crate::BitReader;
#[doc = "Field `BUSERR` writer - Bus error"]
pub type BUSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARLOST` reader - Arbitration lost (master mode)"]
pub type ARLOST_R = crate::BitReader;
#[doc = "Field `ARLOST` writer - Arbitration lost (master mode)"]
pub type ARLOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKFAIL` reader - Acknowledge failure"]
pub type ACKFAIL_R = crate::BitReader;
#[doc = "Field `ACKFAIL` writer - Acknowledge failure"]
pub type ACKFAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUF` reader - Overflow or underflow"]
pub type OUF_R = crate::BitReader;
#[doc = "Field `OUF` writer - Overflow or underflow"]
pub type OUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECERR` reader - PEC receive error"]
pub type PECERR_R = crate::BitReader;
#[doc = "Field `PECERR` writer - PEC receive error"]
pub type PECERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMOUT` reader - Timeout error"]
pub type TMOUT_R = crate::BitReader;
#[doc = "Field `TMOUT` writer - Timeout error"]
pub type TMOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERTF` reader - SMBus alert"]
pub type ALERTF_R = crate::BitReader;
#[doc = "Field `ALERTF` writer - SMBus alert"]
pub type ALERTF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start bit (Master mode)"]
    #[inline(always)]
    pub fn startf(&self) -> STARTF_R {
        STARTF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address sent (master mode)/matched (slave mode)"]
    #[inline(always)]
    pub fn addr7f(&self) -> ADDR7F_R {
        ADDR7F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit data complete"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - address header match (Master mode)"]
    #[inline(always)]
    pub fn addrhf(&self) -> ADDRHF_R {
        ADDRHF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop detection (slave mode)"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive data buffer full (receivers)"]
    #[inline(always)]
    pub fn rdbf(&self) -> RDBF_R {
        RDBF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data buffer empty (transmitters)"]
    #[inline(always)]
    pub fn tdbe(&self) -> TDBE_R {
        TDBE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn arlost(&self) -> ARLOST_R {
        ARLOST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    pub fn ackfail(&self) -> ACKFAIL_R {
        ACKFAIL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Overflow or underflow"]
    #[inline(always)]
    pub fn ouf(&self) -> OUF_R {
        OUF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PEC receive error"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Timeout error"]
    #[inline(always)]
    pub fn tmout(&self) -> TMOUT_R {
        TMOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    pub fn alertf(&self) -> ALERTF_R {
        ALERTF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS1")
            .field("alertf", &format_args!("{}", self.alertf().bit()))
            .field("tmout", &format_args!("{}", self.tmout().bit()))
            .field("pecerr", &format_args!("{}", self.pecerr().bit()))
            .field("ouf", &format_args!("{}", self.ouf().bit()))
            .field("ackfail", &format_args!("{}", self.ackfail().bit()))
            .field("arlost", &format_args!("{}", self.arlost().bit()))
            .field("buserr", &format_args!("{}", self.buserr().bit()))
            .field("tdbe", &format_args!("{}", self.tdbe().bit()))
            .field("rdbf", &format_args!("{}", self.rdbf().bit()))
            .field("stopf", &format_args!("{}", self.stopf().bit()))
            .field("addrhf", &format_args!("{}", self.addrhf().bit()))
            .field("tdc", &format_args!("{}", self.tdc().bit()))
            .field("addr7f", &format_args!("{}", self.addr7f().bit()))
            .field("startf", &format_args!("{}", self.startf().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STS1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    #[must_use]
    pub fn buserr(&mut self) -> BUSERR_W<STS1_SPEC> {
        BUSERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn arlost(&mut self) -> ARLOST_W<STS1_SPEC> {
        ARLOST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    #[must_use]
    pub fn ackfail(&mut self) -> ACKFAIL_W<STS1_SPEC> {
        ACKFAIL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Overflow or underflow"]
    #[inline(always)]
    #[must_use]
    pub fn ouf(&mut self) -> OUF_W<STS1_SPEC> {
        OUF_W::new(self, 11)
    }
    #[doc = "Bit 12 - PEC receive error"]
    #[inline(always)]
    #[must_use]
    pub fn pecerr(&mut self) -> PECERR_W<STS1_SPEC> {
        PECERR_W::new(self, 12)
    }
    #[doc = "Bit 14 - Timeout error"]
    #[inline(always)]
    #[must_use]
    pub fn tmout(&mut self) -> TMOUT_W<STS1_SPEC> {
        TMOUT_W::new(self, 14)
    }
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    #[must_use]
    pub fn alertf(&mut self) -> ALERTF_W<STS1_SPEC> {
        ALERTF_W::new(self, 15)
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
#[doc = "Status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS1_SPEC;
impl crate::RegisterSpec for STS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts1::R`](R) reader structure"]
impl crate::Readable for STS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts1::W`](W) writer structure"]
impl crate::Writable for STS1_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STS1 to value 0"]
impl crate::Resettable for STS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
