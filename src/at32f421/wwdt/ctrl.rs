#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `CNT` reader - Decrement counter"]
pub type CNT_R = crate::FieldReader;
#[doc = "Field `CNT` writer - Decrement counter"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
#[doc = "Window watchdog enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdtenr {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Wwdtenr> for bool {
    #[inline(always)]
    fn from(variant: Wwdtenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDTEN` reader - Window watchdog enable"]
pub type WWDTEN_R = crate::BitReader<Wwdtenr>;
impl WWDTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdtenr {
        match self.bits {
            false => Wwdtenr::Disabled,
            true => Wwdtenr::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wwdtenr::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wwdtenr::Enabled
    }
}
#[doc = "Window watchdog enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WwdtenwWO {
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<WwdtenwWO> for bool {
    #[inline(always)]
    fn from(variant: WwdtenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDTEN` writer - Window watchdog enable"]
pub type WWDTEN_W<'a, REG> = crate::BitWriter1S<'a, REG, WwdtenwWO>;
impl<'a, REG> WWDTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WwdtenwWO::Enable)
    }
}
impl R {
    #[doc = "Bits 0:6 - Decrement counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Window watchdog enable"]
    #[inline(always)]
    pub fn wwdten(&self) -> WWDTEN_R {
        WWDTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("cnt", &self.cnt())
            .field("wwdten", &self.wwdten())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Decrement counter"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<'_, CTRL_SPEC> {
        CNT_W::new(self, 0)
    }
    #[doc = "Bit 7 - Window watchdog enable"]
    #[inline(always)]
    pub fn wwdten(&mut self) -> WWDTEN_W<'_, CTRL_SPEC> {
        WWDTEN_W::new(self, 7)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x80;
}
#[doc = "`reset()` method sets CTRL to value 0x7f"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x7f;
}
