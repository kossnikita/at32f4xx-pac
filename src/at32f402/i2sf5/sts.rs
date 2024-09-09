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
#[doc = "Field `ROERR` reader - Receiver overflow error"]
pub type ROERR_R = crate::BitReader;
#[doc = "Field `BF` reader - Busy flag"]
pub type BF_R = crate::BitReader;
#[doc = "Field `CSPAS` reader - CS pulse abnormal setting fiag"]
pub type CSPAS_R = crate::BitReader;
#[doc = "Field `CSPAS` writer - CS pulse abnormal setting fiag"]
pub type CSPAS_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 8 - CS pulse abnormal setting fiag"]
    #[inline(always)]
    pub fn cspas(&self) -> CSPAS_R {
        CSPAS_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("cspas", &self.cspas())
            .field("bf", &self.bf())
            .field("roerr", &self.roerr())
            .field("tuerr", &self.tuerr())
            .field("acs", &self.acs())
            .field("tdbe", &self.tdbe())
            .field("rdbf", &self.rdbf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - CS pulse abnormal setting fiag"]
    #[inline(always)]
    #[must_use]
    pub fn cspas(&mut self) -> CSPAS_W<STS_SPEC> {
        CSPAS_W::new(self, 8)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS to value 0x02"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
