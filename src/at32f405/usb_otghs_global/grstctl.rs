#[doc = "Register `GRSTCTL` reader"]
pub type R = crate::R<GRSTCTL_SPEC>;
#[doc = "Register `GRSTCTL` writer"]
pub type W = crate::W<GRSTCTL_SPEC>;
#[doc = "Field `CSFTRST` reader - Core soft reset"]
pub type CSFTRST_R = crate::BitReader;
#[doc = "Field `CSFTRST` writer - Core soft reset"]
pub type CSFTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIUSFTRST` reader - PIU FS Dedicated Controller Soft Reset"]
pub type PIUSFTRST_R = crate::BitReader;
#[doc = "Field `PIUSFTRST` writer - PIU FS Dedicated Controller Soft Reset"]
pub type PIUSFTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMCNTRST` reader - Host frame counter reset"]
pub type FRMCNTRST_R = crate::BitReader;
#[doc = "Field `FRMCNTRST` writer - Host frame counter reset"]
pub type FRMCNTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFLSH` reader - RxFIFO flush"]
pub type RXFFLSH_R = crate::BitReader;
#[doc = "Field `RXFFLSH` writer - RxFIFO flush"]
pub type RXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFLSH` reader - TxFIFO flush"]
pub type TXFFLSH_R = crate::BitReader;
#[doc = "Field `TXFFLSH` writer - TxFIFO flush"]
pub type TXFFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - TxFIFO number"]
pub type TXFNUM_R = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - TxFIFO number"]
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DMAREQ` reader - DMA Request Signal"]
pub type DMAREQ_R = crate::BitReader;
#[doc = "Field `AHBIDLE` reader - AHB master idle"]
pub type AHBIDLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Core soft reset"]
    #[inline(always)]
    pub fn csftrst(&self) -> CSFTRST_R {
        CSFTRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PIU FS Dedicated Controller Soft Reset"]
    #[inline(always)]
    pub fn piusftrst(&self) -> PIUSFTRST_R {
        PIUSFTRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host frame counter reset"]
    #[inline(always)]
    pub fn frmcntrst(&self) -> FRMCNTRST_R {
        FRMCNTRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO flush"]
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TxFIFO flush"]
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - TxFIFO number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - DMA Request Signal"]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AHB master idle"]
    #[inline(always)]
    pub fn ahbidle(&self) -> AHBIDLE_R {
        AHBIDLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRSTCTL")
            .field("csftrst", &self.csftrst())
            .field("piusftrst", &self.piusftrst())
            .field("frmcntrst", &self.frmcntrst())
            .field("rxfflsh", &self.rxfflsh())
            .field("txfflsh", &self.txfflsh())
            .field("txfnum", &self.txfnum())
            .field("dmareq", &self.dmareq())
            .field("ahbidle", &self.ahbidle())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Core soft reset"]
    #[inline(always)]
    #[must_use]
    pub fn csftrst(&mut self) -> CSFTRST_W<GRSTCTL_SPEC> {
        CSFTRST_W::new(self, 0)
    }
    #[doc = "Bit 1 - PIU FS Dedicated Controller Soft Reset"]
    #[inline(always)]
    #[must_use]
    pub fn piusftrst(&mut self) -> PIUSFTRST_W<GRSTCTL_SPEC> {
        PIUSFTRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Host frame counter reset"]
    #[inline(always)]
    #[must_use]
    pub fn frmcntrst(&mut self) -> FRMCNTRST_W<GRSTCTL_SPEC> {
        FRMCNTRST_W::new(self, 2)
    }
    #[doc = "Bit 4 - RxFIFO flush"]
    #[inline(always)]
    #[must_use]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W<GRSTCTL_SPEC> {
        RXFFLSH_W::new(self, 4)
    }
    #[doc = "Bit 5 - TxFIFO flush"]
    #[inline(always)]
    #[must_use]
    pub fn txfflsh(&mut self) -> TXFFLSH_W<GRSTCTL_SPEC> {
        TXFFLSH_W::new(self, 5)
    }
    #[doc = "Bits 6:10 - TxFIFO number"]
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TXFNUM_W<GRSTCTL_SPEC> {
        TXFNUM_W::new(self, 6)
    }
}
#[doc = "OTGHS reset register (OTGHS_GRSTCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`grstctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRSTCTL_SPEC;
impl crate::RegisterSpec for GRSTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grstctl::R`](R) reader structure"]
impl crate::Readable for GRSTCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`grstctl::W`](W) writer structure"]
impl crate::Writable for GRSTCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRSTCTL to value 0x2000_0000"]
impl crate::Resettable for GRSTCTL_SPEC {
    const RESET_VALUE: u32 = 0x2000_0000;
}
