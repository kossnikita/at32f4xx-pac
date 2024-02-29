#[doc = "Register `GCCFG` reader"]
pub type R = crate::R<GCCFG_SPEC>;
#[doc = "Register `GCCFG` writer"]
pub type W = crate::W<GCCFG_SPEC>;
#[doc = "Field `PWRDOWN` reader - Power down"]
pub type PWRDOWN_R = crate::BitReader;
#[doc = "Field `PWRDOWN` writer - Power down"]
pub type PWRDOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVALIDSESEN` reader - sense Avalid enable"]
pub type AVALIDSESEN_R = crate::BitReader;
#[doc = "Field `AVALIDSESEN` writer - sense Avalid enable"]
pub type AVALIDSESEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BVALIDSESEN` reader - sense Bvalid enable"]
pub type BVALIDSESEN_R = crate::BitReader;
#[doc = "Field `BVALIDSESEN` writer - sense Bvalid enable"]
pub type BVALIDSESEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFOUTEN` reader - SOF output enable"]
pub type SOFOUTEN_R = crate::BitReader;
#[doc = "Field `SOFOUTEN` writer - SOF output enable"]
pub type SOFOUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSIG` reader - VBUS Ignored"]
pub type VBUSIG_R = crate::BitReader;
#[doc = "Field `VBUSIG` writer - VBUS Ignored"]
pub type VBUSIG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    pub fn pwrdown(&self) -> PWRDOWN_R {
        PWRDOWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - sense Avalid enable"]
    #[inline(always)]
    pub fn avalidsesen(&self) -> AVALIDSESEN_R {
        AVALIDSESEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - sense Bvalid enable"]
    #[inline(always)]
    pub fn bvalidsesen(&self) -> BVALIDSESEN_R {
        BVALIDSESEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    pub fn sofouten(&self) -> SOFOUTEN_R {
        SOFOUTEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - VBUS Ignored"]
    #[inline(always)]
    pub fn vbusig(&self) -> VBUSIG_R {
        VBUSIG_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCCFG")
            .field("pwrdown", &format_args!("{}", self.pwrdown().bit()))
            .field("avalidsesen", &format_args!("{}", self.avalidsesen().bit()))
            .field("bvalidsesen", &format_args!("{}", self.bvalidsesen().bit()))
            .field("sofouten", &format_args!("{}", self.sofouten().bit()))
            .field("vbusig", &format_args!("{}", self.vbusig().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GCCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdown(&mut self) -> PWRDOWN_W<GCCFG_SPEC> {
        PWRDOWN_W::new(self, 16)
    }
    #[doc = "Bit 18 - sense Avalid enable"]
    #[inline(always)]
    #[must_use]
    pub fn avalidsesen(&mut self) -> AVALIDSESEN_W<GCCFG_SPEC> {
        AVALIDSESEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - sense Bvalid enable"]
    #[inline(always)]
    #[must_use]
    pub fn bvalidsesen(&mut self) -> BVALIDSESEN_W<GCCFG_SPEC> {
        BVALIDSESEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofouten(&mut self) -> SOFOUTEN_W<GCCFG_SPEC> {
        SOFOUTEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - VBUS Ignored"]
    #[inline(always)]
    #[must_use]
    pub fn vbusig(&mut self) -> VBUSIG_W<GCCFG_SPEC> {
        VBUSIG_W::new(self, 21)
    }
}
#[doc = "OTGFS general core configuration register (OTGFS_GCCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCCFG_SPEC;
impl crate::RegisterSpec for GCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gccfg::R`](R) reader structure"]
impl crate::Readable for GCCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gccfg::W`](W) writer structure"]
impl crate::Writable for GCCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCCFG to value 0"]
impl crate::Resettable for GCCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
