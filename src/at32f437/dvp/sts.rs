#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Field `HSYN` reader - Horizontal synchronization status"]
pub type HSYN_R = crate::BitReader;
#[doc = "Field `VSYN` reader - Vertical synchronization status"]
pub type VSYN_R = crate::BitReader;
#[doc = "Field `OFNE` reader - Output FIFO Non-empty"]
pub type OFNE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Horizontal synchronization status"]
    #[inline(always)]
    pub fn hsyn(&self) -> HSYN_R {
        HSYN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Vertical synchronization status"]
    #[inline(always)]
    pub fn vsyn(&self) -> VSYN_R {
        VSYN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output FIFO Non-empty"]
    #[inline(always)]
    pub fn ofne(&self) -> OFNE_R {
        OFNE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("ofne", &self.ofne())
            .field("vsyn", &self.vsyn())
            .field("hsyn", &self.hsyn())
            .finish()
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {}
