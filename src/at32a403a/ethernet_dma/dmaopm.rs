#[doc = "Register `DMAOPM` reader"]
pub type R = crate::R<DMAOPM_SPEC>;
#[doc = "Register `DMAOPM` writer"]
pub type W = crate::W<DMAOPM_SPEC>;
#[doc = "Field `SSR` reader - Start or stop receive"]
pub type SSR_R = crate::BitReader;
#[doc = "Field `SSR` writer - Start or stop receive"]
pub type SSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSF` reader - Operate on second frame"]
pub type OSF_R = crate::BitReader;
#[doc = "Field `OSF` writer - Operate on second frame"]
pub type OSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC` reader - Receive threshold control"]
pub type RTC_R = crate::FieldReader;
#[doc = "Field `RTC` writer - Receive threshold control"]
pub type RTC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FUGF` reader - Forward undersized good frames"]
pub type FUGF_R = crate::BitReader;
#[doc = "Field `FUGF` writer - Forward undersized good frames"]
pub type FUGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEF` reader - Forward error frames"]
pub type FEF_R = crate::BitReader;
#[doc = "Field `FEF` writer - Forward error frames"]
pub type FEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTC` reader - Start of stop transmission command"]
pub type SSTC_R = crate::BitReader;
#[doc = "Field `SSTC` writer - Start of stop transmission command"]
pub type SSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTC` reader - Transmit threshold control"]
pub type TTC_R = crate::FieldReader;
#[doc = "Field `TTC` writer - Transmit threshold control"]
pub type TTC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FTF` reader - Flush transmit FIFO"]
pub type FTF_R = crate::BitReader;
#[doc = "Field `FTF` writer - Flush transmit FIFO"]
pub type FTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSF` reader - Transmit store and forward"]
pub type TSF_R = crate::BitReader;
#[doc = "Field `TSF` writer - Transmit store and forward"]
pub type TSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFRF` reader - Disable flushing of received frames"]
pub type DFRF_R = crate::BitReader;
#[doc = "Field `DFRF` writer - Disable flushing of received frames"]
pub type DFRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSF` reader - Receive store and forward"]
pub type RSF_R = crate::BitReader;
#[doc = "Field `RSF` writer - Receive store and forward"]
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT` reader - Disable dropping of TCP/IP checksum error frames"]
pub type DT_R = crate::BitReader;
#[doc = "Field `DT` writer - Disable dropping of TCP/IP checksum error frames"]
pub type DT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Start or stop receive"]
    #[inline(always)]
    pub fn ssr(&self) -> SSR_R {
        SSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Forward undersized good frames"]
    #[inline(always)]
    pub fn fugf(&self) -> FUGF_R {
        FUGF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Forward error frames"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - Start of stop transmission command"]
    #[inline(always)]
    pub fn sstc(&self) -> SSTC_R {
        SSTC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - Flush transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit store and forward"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable flushing of received frames"]
    #[inline(always)]
    pub fn dfrf(&self) -> DFRF_R {
        DFRF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive store and forward"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Disable dropping of TCP/IP checksum error frames"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAOPM")
            .field("ssr", &self.ssr())
            .field("osf", &self.osf())
            .field("rtc", &self.rtc())
            .field("fugf", &self.fugf())
            .field("fef", &self.fef())
            .field("sstc", &self.sstc())
            .field("ttc", &self.ttc())
            .field("ftf", &self.ftf())
            .field("tsf", &self.tsf())
            .field("dfrf", &self.dfrf())
            .field("rsf", &self.rsf())
            .field("dt", &self.dt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Start or stop receive"]
    #[inline(always)]
    #[must_use]
    pub fn ssr(&mut self) -> SSR_W<DMAOPM_SPEC> {
        SSR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    #[must_use]
    pub fn osf(&mut self) -> OSF_W<DMAOPM_SPEC> {
        OSF_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<DMAOPM_SPEC> {
        RTC_W::new(self, 3)
    }
    #[doc = "Bit 6 - Forward undersized good frames"]
    #[inline(always)]
    #[must_use]
    pub fn fugf(&mut self) -> FUGF_W<DMAOPM_SPEC> {
        FUGF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Forward error frames"]
    #[inline(always)]
    #[must_use]
    pub fn fef(&mut self) -> FEF_W<DMAOPM_SPEC> {
        FEF_W::new(self, 7)
    }
    #[doc = "Bit 13 - Start of stop transmission command"]
    #[inline(always)]
    #[must_use]
    pub fn sstc(&mut self) -> SSTC_W<DMAOPM_SPEC> {
        SSTC_W::new(self, 13)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TTC_W<DMAOPM_SPEC> {
        TTC_W::new(self, 14)
    }
    #[doc = "Bit 20 - Flush transmit FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn ftf(&mut self) -> FTF_W<DMAOPM_SPEC> {
        FTF_W::new(self, 20)
    }
    #[doc = "Bit 21 - Transmit store and forward"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<DMAOPM_SPEC> {
        TSF_W::new(self, 21)
    }
    #[doc = "Bit 24 - Disable flushing of received frames"]
    #[inline(always)]
    #[must_use]
    pub fn dfrf(&mut self) -> DFRF_W<DMAOPM_SPEC> {
        DFRF_W::new(self, 24)
    }
    #[doc = "Bit 25 - Receive store and forward"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<DMAOPM_SPEC> {
        RSF_W::new(self, 25)
    }
    #[doc = "Bit 26 - Disable dropping of TCP/IP checksum error frames"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<DMAOPM_SPEC> {
        DT_W::new(self, 26)
    }
}
#[doc = "Ethernet DMA operation mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaopm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaopm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAOPM_SPEC;
impl crate::RegisterSpec for DMAOPM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaopm::R`](R) reader structure"]
impl crate::Readable for DMAOPM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmaopm::W`](W) writer structure"]
impl crate::Writable for DMAOPM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAOPM to value 0"]
impl crate::Resettable for DMAOPM_SPEC {
    const RESET_VALUE: u32 = 0;
}
