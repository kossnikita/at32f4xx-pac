#[doc = "Register `TMC` reader"]
pub type R = crate::R<TMC_SPEC>;
#[doc = "Register `TMC` writer"]
pub type W = crate::W<TMC_SPEC>;
#[doc = "Field `DTBL` reader - Transmit mailbox data byte length"]
pub type DTBL_R = crate::FieldReader;
#[doc = "Field `DTBL` writer - Transmit mailbox data byte length"]
pub type DTBL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `TSTEN` reader - Transmit mailbox time stamp transmit enable"]
pub type TSTEN_R = crate::BitReader<TSTENR_A>;
#[doc = "Transmit mailbox time stamp transmit enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTENR_A {
    #[doc = "0: Mailbox time stamp transmit is disabled"]
    Disabled = 0,
    #[doc = "1: Mailbox time stamp transmit is enabled"]
    Enabled = 1,
}
impl From<TSTENR_A> for bool {
    #[inline(always)]
    fn from(variant: TSTENR_A) -> Self {
        variant as u8 != 0
    }
}
impl TSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSTENR_A {
        match self.bits {
            false => TSTENR_A::Disabled,
            true => TSTENR_A::Enabled,
        }
    }
    #[doc = "Mailbox time stamp transmit is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSTENR_A::Disabled
    }
    #[doc = "Mailbox time stamp transmit is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSTENR_A::Enabled
    }
}
#[doc = "Transmit mailbox time stamp transmit enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTENW_AW {
    #[doc = "0: Mailbox time stamp transmit disable"]
    Disable = 0,
    #[doc = "1: Mailbox time stamp transmit enable"]
    Enable = 1,
}
impl From<TSTENW_AW> for bool {
    #[inline(always)]
    fn from(variant: TSTENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTEN` writer - Transmit mailbox time stamp transmit enable"]
pub type TSTEN_W<'a, REG> = crate::BitWriter<'a, REG, TSTENW_AW>;
impl<'a, REG> TSTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mailbox time stamp transmit disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TSTENW_AW::Disable)
    }
    #[doc = "Mailbox time stamp transmit enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TSTENW_AW::Enable)
    }
}
#[doc = "Field `TS` reader - Transmit mailbox time stamp"]
pub type TS_R = crate::FieldReader<u16>;
#[doc = "Field `TS` writer - Transmit mailbox time stamp"]
pub type TS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - Transmit mailbox data byte length"]
    #[inline(always)]
    pub fn dtbl(&self) -> DTBL_R {
        DTBL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Transmit mailbox time stamp transmit enable"]
    #[inline(always)]
    pub fn tsten(&self) -> TSTEN_R {
        TSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Transmit mailbox time stamp"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMC")
            .field("ts", &format_args!("{}", self.ts().bits()))
            .field("tsten", &format_args!("{}", self.tsten().bit()))
            .field("dtbl", &format_args!("{}", self.dtbl().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TMC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmit mailbox data byte length"]
    #[inline(always)]
    #[must_use]
    pub fn dtbl(&mut self) -> DTBL_W<TMC_SPEC> {
        DTBL_W::new(self, 0)
    }
    #[doc = "Bit 8 - Transmit mailbox time stamp transmit enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsten(&mut self) -> TSTEN_W<TMC_SPEC> {
        TSTEN_W::new(self, 8)
    }
    #[doc = "Bits 16:31 - Transmit mailbox time stamp"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<TMC_SPEC> {
        TS_W::new(self, 16)
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
#[doc = "Transmit mailbox data length and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMC_SPEC;
impl crate::RegisterSpec for TMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmc::R`](R) reader structure"]
impl crate::Readable for TMC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmc::W`](W) writer structure"]
impl crate::Writable for TMC_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMC to value 0"]
impl crate::Resettable for TMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
