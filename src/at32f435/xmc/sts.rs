#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Field `ERR` reader - error flag"]
pub type ERR_R = crate::BitReader;
#[doc = "Field `BK1STS` reader - Bank 1 Status"]
pub type BK1STS_R = crate::FieldReader;
#[doc = "Field `BK2STS` reader - Bank 2 Status"]
pub type BK2STS_R = crate::FieldReader;
#[doc = "Field `BUSY` reader - Busy status"]
pub type BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - error flag"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Bank 1 Status"]
    #[inline(always)]
    pub fn bk1sts(&self) -> BK1STS_R {
        BK1STS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Bank 2 Status"]
    #[inline(always)]
    pub fn bk2sts(&self) -> BK2STS_R {
        BK2STS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Busy status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("err", &self.err())
            .field("bk1sts", &self.bk1sts())
            .field("bk2sts", &self.bk2sts())
            .field("busy", &self.busy())
            .finish()
    }
}
#[doc = "SDRAM Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {}
