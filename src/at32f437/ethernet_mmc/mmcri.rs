#[doc = "Register `MMCRI` reader"]
pub type R = crate::R<MMCRI_SPEC>;
#[doc = "Register `MMCRI` writer"]
pub type W = crate::W<MMCRI_SPEC>;
#[doc = "Field `RFCE` reader - Received frames CRC error"]
pub type RFCE_R = crate::BitReader;
#[doc = "Field `RFCE` writer - Received frames CRC error"]
pub type RFCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFAE` reader - Received frames alignment error"]
pub type RFAE_R = crate::BitReader;
#[doc = "Field `RFAE` writer - Received frames alignment error"]
pub type RFAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGUF` reader - Received good unicast frames"]
pub type RGUF_R = crate::BitReader;
#[doc = "Field `RGUF` writer - Received good unicast frames"]
pub type RGUF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Received frames CRC error"]
    #[inline(always)]
    pub fn rfce(&self) -> RFCE_R {
        RFCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received frames alignment error"]
    #[inline(always)]
    pub fn rfae(&self) -> RFAE_R {
        RFAE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - Received good unicast frames"]
    #[inline(always)]
    pub fn rguf(&self) -> RGUF_R {
        RGUF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCRI")
            .field("rfce", &format_args!("{}", self.rfce().bit()))
            .field("rfae", &format_args!("{}", self.rfae().bit()))
            .field("rguf", &format_args!("{}", self.rguf().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MMCRI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 5 - Received frames CRC error"]
    #[inline(always)]
    #[must_use]
    pub fn rfce(&mut self) -> RFCE_W<MMCRI_SPEC> {
        RFCE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Received frames alignment error"]
    #[inline(always)]
    #[must_use]
    pub fn rfae(&mut self) -> RFAE_W<MMCRI_SPEC> {
        RFAE_W::new(self, 6)
    }
    #[doc = "Bit 17 - Received good unicast frames"]
    #[inline(always)]
    #[must_use]
    pub fn rguf(&mut self) -> RGUF_W<MMCRI_SPEC> {
        RGUF_W::new(self, 17)
    }
}
#[doc = "Ethernet MMC receive interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcri::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcri::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCRI_SPEC;
impl crate::RegisterSpec for MMCRI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcri::R`](R) reader structure"]
impl crate::Readable for MMCRI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmcri::W`](W) writer structure"]
impl crate::Writable for MMCRI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMCRI to value 0"]
impl crate::Resettable for MMCRI_SPEC {
    const RESET_VALUE: u32 = 0;
}
