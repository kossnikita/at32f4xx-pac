#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Field `RDBF` reader - Receive data buffer full"]
pub type RDBF_R = crate::BitReader;
#[doc = "Field `TDBE` reader - Transmit data buffer empty"]
pub type TDBE_R = crate::BitReader;
#[doc = "Field `ACS` reader - Audio channel state"]
pub type ACS_R = crate::BitReader;
#[doc = "Field `TUERR` reader - Transmitter underload error"]
pub type TUERR_R = crate::BitReader;
#[doc = "Field `CCERR` reader - CRC calculation error"]
pub type CCERR_R = crate::BitReader;
#[doc = "Field `CCERR` writer - CRC calculation error"]
pub type CCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMERR` reader - Master mode error"]
pub type MMERR_R = crate::BitReader;
#[doc = "Field `ROERR` reader - Receiver overflow error"]
pub type ROERR_R = crate::BitReader;
#[doc = "Field `BF` reader - Busy flag"]
pub type BF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive data buffer full"]
    #[inline(always)]
    pub fn rdbf(&self) -> RDBF_R {
        RDBF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit data buffer empty"]
    #[inline(always)]
    pub fn tdbe(&self) -> TDBE_R {
        TDBE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Audio channel state"]
    #[inline(always)]
    pub fn acs(&self) -> ACS_R {
        ACS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter underload error"]
    #[inline(always)]
    pub fn tuerr(&self) -> TUERR_R {
        TUERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC calculation error"]
    #[inline(always)]
    pub fn ccerr(&self) -> CCERR_R {
        CCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master mode error"]
    #[inline(always)]
    pub fn mmerr(&self) -> MMERR_R {
        MMERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receiver overflow error"]
    #[inline(always)]
    pub fn roerr(&self) -> ROERR_R {
        ROERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Busy flag"]
    #[inline(always)]
    pub fn bf(&self) -> BF_R {
        BF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("bf", &self.bf())
            .field("roerr", &self.roerr())
            .field("mmerr", &self.mmerr())
            .field("ccerr", &self.ccerr())
            .field("tuerr", &self.tuerr())
            .field("acs", &self.acs())
            .field("tdbe", &self.tdbe())
            .field("rdbf", &self.rdbf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - CRC calculation error"]
    #[inline(always)]
    pub fn ccerr(&mut self) -> CCERR_W<'_, STS_SPEC> {
        CCERR_W::new(self, 4)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STS to value 0x02"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
