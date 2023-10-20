#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `CNT` reader - Decrement counter"]
pub type CNT_R = crate::FieldReader;
#[doc = "Field `CNT` writer - Decrement counter"]
pub type CNT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Field `WWDTEN` reader - Window watchdog enable"]
pub type WWDTEN_R = crate::BitReader<WWDTENR_A>;
#[doc = "Window watchdog enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDTENR_A {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<WWDTENR_A> for bool {
    #[inline(always)]
    fn from(variant: WWDTENR_A) -> Self {
        variant as u8 != 0
    }
}
impl WWDTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WWDTENR_A {
        match self.bits {
            false => WWDTENR_A::Disabled,
            true => WWDTENR_A::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WWDTENR_A::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WWDTENR_A::Enabled
    }
}
#[doc = "Window watchdog enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDTENW_AW {
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<WWDTENW_AW> for bool {
    #[inline(always)]
    fn from(variant: WWDTENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDTEN` writer - Window watchdog enable"]
pub type WWDTEN_W<'a, REG> = crate::BitWriter1S<'a, REG, WWDTENW_AW>;
impl<'a, REG> WWDTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WWDTENW_AW::Enable)
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
            .field("cnt", &format_args!("{}", self.cnt().bits()))
            .field("wwdten", &format_args!("{}", self.wwdten().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Decrement counter"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<CTRL_SPEC> {
        CNT_W::new(self, 0)
    }
    #[doc = "Bit 7 - Window watchdog enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdten(&mut self) -> WWDTEN_W<CTRL_SPEC> {
        WWDTEN_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0x80;
}
#[doc = "`reset()` method sets CTRL to value 0x7f"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
