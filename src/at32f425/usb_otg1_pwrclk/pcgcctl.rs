#[doc = "Register `PCGCCTL` reader"]
pub type R = crate::R<PCGCCTL_SPEC>;
#[doc = "Register `PCGCCTL` writer"]
pub type W = crate::W<PCGCCTL_SPEC>;
#[doc = "Field `STOPPCLK` reader - Stop PHY clock"]
pub type STOPPCLK_R = crate::BitReader;
#[doc = "Field `STOPPCLK` writer - Stop PHY clock"]
pub type STOPPCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPENDM` reader - PHY Suspended"]
pub type SUSPENDM_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    pub fn stoppclk(&self) -> STOPPCLK_R {
        STOPPCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - PHY Suspended"]
    #[inline(always)]
    pub fn suspendm(&self) -> SUSPENDM_R {
        SUSPENDM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCGCCTL")
            .field("stoppclk", &self.stoppclk())
            .field("suspendm", &self.suspendm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    #[must_use]
    pub fn stoppclk(&mut self) -> STOPPCLK_W<PCGCCTL_SPEC> {
        STOPPCLK_W::new(self, 0)
    }
}
#[doc = "OTGFS power and clock gating control register (OTGFS_PCGCCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`pcgcctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCGCCTL_SPEC;
impl crate::RegisterSpec for PCGCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcgcctl::R`](R) reader structure"]
impl crate::Readable for PCGCCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcgcctl::W`](W) writer structure"]
impl crate::Writable for PCGCCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCGCCTL to value 0"]
impl crate::Resettable for PCGCCTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
