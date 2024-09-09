#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Field `TDBE` reader - Transmit data buffer empty flag"]
pub type TDBE_R = crate::BitReader;
#[doc = "Field `TDBE` writer - Transmit data buffer empty flag"]
pub type TDBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDIS` reader - Send interrupt status"]
pub type TDIS_R = crate::BitReader;
#[doc = "Field `TDIS` writer - Send interrupt status"]
pub type TDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDBF` reader - Receive data buffer full flag"]
pub type RDBF_R = crate::BitReader;
#[doc = "Field `ADDRF` reader - 0~7 bit address match flag"]
pub type ADDRF_R = crate::BitReader;
#[doc = "Field `ACKFAIL` reader - Acknowledge failure flag"]
pub type ACKFAIL_R = crate::BitReader;
#[doc = "Field `STOPF` reader - Stop condition generation complete flag"]
pub type STOPF_R = crate::BitReader;
#[doc = "Field `TDC` reader - Transmit data complete flag"]
pub type TDC_R = crate::BitReader;
#[doc = "Field `TCRLD` reader - Transmission is complete, waiting to load data"]
pub type TCRLD_R = crate::BitReader;
#[doc = "Field `BUSERR` reader - Bus error flag"]
pub type BUSERR_R = crate::BitReader;
#[doc = "Field `ARLOST` reader - Arbitration lost flag"]
pub type ARLOST_R = crate::BitReader;
#[doc = "Field `OUF` reader - Overflow or underflow flag"]
pub type OUF_R = crate::BitReader;
#[doc = "Field `PECERR` reader - PEC receive error flag"]
pub type PECERR_R = crate::BitReader;
#[doc = "Field `TMOUT` reader - SMBus timeout flag"]
pub type TMOUT_R = crate::BitReader;
#[doc = "Field `ALERTF` reader - SMBus alert flag"]
pub type ALERTF_R = crate::BitReader;
#[doc = "Field `BUSYF` reader - Bus busy"]
pub type BUSYF_R = crate::BitReader;
#[doc = "Field `SDIR` reader - Slave data transmit direction"]
pub type SDIR_R = crate::BitReader;
#[doc = "Field `ADDR` reader - Slave address matching value"]
pub type ADDR_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmit data buffer empty flag"]
    #[inline(always)]
    pub fn tdbe(&self) -> TDBE_R {
        TDBE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Send interrupt status"]
    #[inline(always)]
    pub fn tdis(&self) -> TDIS_R {
        TDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive data buffer full flag"]
    #[inline(always)]
    pub fn rdbf(&self) -> RDBF_R {
        RDBF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0~7 bit address match flag"]
    #[inline(always)]
    pub fn addrf(&self) -> ADDRF_R {
        ADDRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Acknowledge failure flag"]
    #[inline(always)]
    pub fn ackfail(&self) -> ACKFAIL_R {
        ACKFAIL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop condition generation complete flag"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit data complete flag"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission is complete, waiting to load data"]
    #[inline(always)]
    pub fn tcrld(&self) -> TCRLD_R {
        TCRLD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error flag"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost flag"]
    #[inline(always)]
    pub fn arlost(&self) -> ARLOST_R {
        ARLOST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overflow or underflow flag"]
    #[inline(always)]
    pub fn ouf(&self) -> OUF_R {
        OUF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PEC receive error flag"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SMBus timeout flag"]
    #[inline(always)]
    pub fn tmout(&self) -> TMOUT_R {
        TMOUT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus alert flag"]
    #[inline(always)]
    pub fn alertf(&self) -> ALERTF_R {
        ALERTF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Bus busy"]
    #[inline(always)]
    pub fn busyf(&self) -> BUSYF_R {
        BUSYF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave data transmit direction"]
    #[inline(always)]
    pub fn sdir(&self) -> SDIR_R {
        SDIR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Slave address matching value"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("addr", &self.addr())
            .field("sdir", &self.sdir())
            .field("busyf", &self.busyf())
            .field("alertf", &self.alertf())
            .field("tmout", &self.tmout())
            .field("pecerr", &self.pecerr())
            .field("ouf", &self.ouf())
            .field("arlost", &self.arlost())
            .field("buserr", &self.buserr())
            .field("tcrld", &self.tcrld())
            .field("tdc", &self.tdc())
            .field("stopf", &self.stopf())
            .field("ackfail", &self.ackfail())
            .field("addrf", &self.addrf())
            .field("rdbf", &self.rdbf())
            .field("tdis", &self.tdis())
            .field("tdbe", &self.tdbe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmit data buffer empty flag"]
    #[inline(always)]
    #[must_use]
    pub fn tdbe(&mut self) -> TDBE_W<STS_SPEC> {
        TDBE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Send interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn tdis(&mut self) -> TDIS_W<STS_SPEC> {
        TDIS_W::new(self, 1)
    }
}
#[doc = "Interrupt and Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS to value 0x01"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
