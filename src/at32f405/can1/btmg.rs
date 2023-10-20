#[doc = "Register `BTMG` reader"]
pub type R = crate::R<BTMG_SPEC>;
#[doc = "Register `BTMG` writer"]
pub type W = crate::W<BTMG_SPEC>;
#[doc = "Field `BRDIV` reader - Baud rate division"]
pub type BRDIV_R = crate::FieldReader<u16>;
#[doc = "Field `BRDIV` writer - Baud rate division"]
pub type BRDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
#[doc = "Field `BTS1` reader - Bit time segment 1"]
pub type BTS1_R = crate::FieldReader;
#[doc = "Field `BTS1` writer - Bit time segment 1"]
pub type BTS1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `BTS2` reader - Bit time segment 2"]
pub type BTS2_R = crate::FieldReader;
#[doc = "Field `BTS2` writer - Bit time segment 2"]
pub type BTS2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Field `RSAW` reader - Resynchronization adjust width"]
pub type RSAW_R = crate::FieldReader;
#[doc = "Field `RSAW` writer - Resynchronization adjust width"]
pub type RSAW_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `LBEN` reader - Loop back mode"]
pub type LBEN_R = crate::BitReader<LBENR_A>;
#[doc = "Loop back mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBENR_A {
    #[doc = "0: Loop back mode is disabled"]
    Disabled = 0,
    #[doc = "1: Loop back mode is enabled"]
    Enabled = 1,
}
impl From<LBENR_A> for bool {
    #[inline(always)]
    fn from(variant: LBENR_A) -> Self {
        variant as u8 != 0
    }
}
impl LBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LBENR_A {
        match self.bits {
            false => LBENR_A::Disabled,
            true => LBENR_A::Enabled,
        }
    }
    #[doc = "Loop back mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBENR_A::Disabled
    }
    #[doc = "Loop back mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBENR_A::Enabled
    }
}
#[doc = "Loop back mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBENW_AW {
    #[doc = "0: Loop back mode disable"]
    Disable = 0,
    #[doc = "1: Loop back mode enable"]
    Enable = 1,
}
impl From<LBENW_AW> for bool {
    #[inline(always)]
    fn from(variant: LBENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBEN` writer - Loop back mode"]
pub type LBEN_W<'a, REG> = crate::BitWriter<'a, REG, LBENW_AW>;
impl<'a, REG> LBEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Loop back mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LBENW_AW::Disable)
    }
    #[doc = "Loop back mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LBENW_AW::Enable)
    }
}
#[doc = "Field `LOEN` reader - Listen-Only mode"]
pub type LOEN_R = crate::BitReader<LOENR_A>;
#[doc = "Listen-Only mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOENR_A {
    #[doc = "0: Listen-Only mode is disabled"]
    Disabled = 0,
    #[doc = "1: Listen-Only mode is enabled"]
    Enabled = 1,
}
impl From<LOENR_A> for bool {
    #[inline(always)]
    fn from(variant: LOENR_A) -> Self {
        variant as u8 != 0
    }
}
impl LOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOENR_A {
        match self.bits {
            false => LOENR_A::Disabled,
            true => LOENR_A::Enabled,
        }
    }
    #[doc = "Listen-Only mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOENR_A::Disabled
    }
    #[doc = "Listen-Only mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOENR_A::Enabled
    }
}
#[doc = "Listen-Only mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOENW_AW {
    #[doc = "0: Listen-Only mode disable"]
    Disable = 0,
    #[doc = "1: Listen-Only mode enable"]
    Enable = 1,
}
impl From<LOENW_AW> for bool {
    #[inline(always)]
    fn from(variant: LOENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOEN` writer - Listen-Only mode"]
pub type LOEN_W<'a, REG> = crate::BitWriter<'a, REG, LOENW_AW>;
impl<'a, REG> LOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Listen-Only mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LOENW_AW::Disable)
    }
    #[doc = "Listen-Only mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LOENW_AW::Enable)
    }
}
impl R {
    #[doc = "Bits 0:11 - Baud rate division"]
    #[inline(always)]
    pub fn brdiv(&self) -> BRDIV_R {
        BRDIV_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Bit time segment 1"]
    #[inline(always)]
    pub fn bts1(&self) -> BTS1_R {
        BTS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Bit time segment 2"]
    #[inline(always)]
    pub fn bts2(&self) -> BTS2_R {
        BTS2_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Resynchronization adjust width"]
    #[inline(always)]
    pub fn rsaw(&self) -> RSAW_R {
        RSAW_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - Loop back mode"]
    #[inline(always)]
    pub fn lben(&self) -> LBEN_R {
        LBEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Listen-Only mode"]
    #[inline(always)]
    pub fn loen(&self) -> LOEN_R {
        LOEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BTMG")
            .field("loen", &format_args!("{}", self.loen().bit()))
            .field("lben", &format_args!("{}", self.lben().bit()))
            .field("rsaw", &format_args!("{}", self.rsaw().bits()))
            .field("bts2", &format_args!("{}", self.bts2().bits()))
            .field("bts1", &format_args!("{}", self.bts1().bits()))
            .field("brdiv", &format_args!("{}", self.brdiv().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<BTMG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - Baud rate division"]
    #[inline(always)]
    #[must_use]
    pub fn brdiv(&mut self) -> BRDIV_W<BTMG_SPEC> {
        BRDIV_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - Bit time segment 1"]
    #[inline(always)]
    #[must_use]
    pub fn bts1(&mut self) -> BTS1_W<BTMG_SPEC> {
        BTS1_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Bit time segment 2"]
    #[inline(always)]
    #[must_use]
    pub fn bts2(&mut self) -> BTS2_W<BTMG_SPEC> {
        BTS2_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - Resynchronization adjust width"]
    #[inline(always)]
    #[must_use]
    pub fn rsaw(&mut self) -> RSAW_W<BTMG_SPEC> {
        RSAW_W::new(self, 24)
    }
    #[doc = "Bit 30 - Loop back mode"]
    #[inline(always)]
    #[must_use]
    pub fn lben(&mut self) -> LBEN_W<BTMG_SPEC> {
        LBEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Listen-Only mode"]
    #[inline(always)]
    #[must_use]
    pub fn loen(&mut self) -> LOEN_W<BTMG_SPEC> {
        LOEN_W::new(self, 31)
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
#[doc = "Bit timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btmg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btmg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTMG_SPEC;
impl crate::RegisterSpec for BTMG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btmg::R`](R) reader structure"]
impl crate::Readable for BTMG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btmg::W`](W) writer structure"]
impl crate::Writable for BTMG_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BTMG to value 0"]
impl crate::Resettable for BTMG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
