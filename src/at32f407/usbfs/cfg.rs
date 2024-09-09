#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `SOFOUTEN` reader - SOF output enable"]
pub type SOFOUTEN_R = crate::BitReader;
#[doc = "Field `SOFOUTEN` writer - SOF output enable"]
pub type SOFOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUO` reader - DP pullup off"]
pub type PUO_R = crate::BitReader;
#[doc = "Field `PUO` writer - DP pullup off"]
pub type PUO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SOF output enable"]
    #[inline(always)]
    pub fn sofouten(&self) -> SOFOUTEN_R {
        SOFOUTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DP pullup off"]
    #[inline(always)]
    pub fn puo(&self) -> PUO_R {
        PUO_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("puo", &self.puo())
            .field("sofouten", &self.sofouten())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SOF output enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofouten(&mut self) -> SOFOUTEN_W<CFG_SPEC> {
        SOFOUTEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DP pullup off"]
    #[inline(always)]
    #[must_use]
    pub fn puo(&mut self) -> PUO_W<CFG_SPEC> {
        PUO_W::new(self, 1)
    }
}
#[doc = "CFG control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
