#[doc = "Register `MMCCTRL` reader"]
pub type R = crate::R<MMCCTRL_SPEC>;
#[doc = "Register `MMCCTRL` writer"]
pub type W = crate::W<MMCCTRL_SPEC>;
#[doc = "Field `RC` reader - Reset counter"]
pub type RC_R = crate::BitReader;
#[doc = "Field `RC` writer - Reset counter"]
pub type RC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCR` reader - Stop counter rollover"]
pub type SCR_R = crate::BitReader;
#[doc = "Field `SCR` writer - Stop counter rollover"]
pub type SCR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RR` reader - Reset on read"]
pub type RR_R = crate::BitReader;
#[doc = "Field `RR` writer - Reset on read"]
pub type RR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMC` reader - Freeze MMC counter"]
pub type FMC_R = crate::BitReader;
#[doc = "Field `FMC` writer - Freeze MMC counter"]
pub type FMC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset counter"]
    #[inline(always)]
    pub fn rc(&self) -> RC_R {
        RC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stop counter rollover"]
    #[inline(always)]
    pub fn scr(&self) -> SCR_R {
        SCR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    pub fn rr(&self) -> RR_R {
        RR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 31 - Freeze MMC counter"]
    #[inline(always)]
    pub fn fmc(&self) -> FMC_R {
        FMC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCCTRL")
            .field("rc", &self.rc())
            .field("scr", &self.scr())
            .field("rr", &self.rr())
            .field("fmc", &self.fmc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reset counter"]
    #[inline(always)]
    #[must_use]
    pub fn rc(&mut self) -> RC_W<MMCCTRL_SPEC> {
        RC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Stop counter rollover"]
    #[inline(always)]
    #[must_use]
    pub fn scr(&mut self) -> SCR_W<MMCCTRL_SPEC> {
        SCR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    #[must_use]
    pub fn rr(&mut self) -> RR_W<MMCCTRL_SPEC> {
        RR_W::new(self, 2)
    }
    #[doc = "Bit 31 - Freeze MMC counter"]
    #[inline(always)]
    #[must_use]
    pub fn fmc(&mut self) -> FMC_W<MMCCTRL_SPEC> {
        FMC_W::new(self, 31)
    }
}
#[doc = "Ethernet MMC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmcctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmcctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCCTRL_SPEC;
impl crate::RegisterSpec for MMCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcctrl::R`](R) reader structure"]
impl crate::Readable for MMCCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmcctrl::W`](W) writer structure"]
impl crate::Writable for MMCCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMCCTRL to value 0"]
impl crate::Resettable for MMCCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
