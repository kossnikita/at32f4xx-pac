#[doc = "Register `TMC` reader"]
pub type R = crate::R<TMC_SPEC>;
#[doc = "Register `TMC` writer"]
pub type W = crate::W<TMC_SPEC>;
#[doc = "Field `DTBL` reader - Transmit mailbox data byte length"]
pub type DTBL_R = crate::FieldReader;
#[doc = "Field `DTBL` writer - Transmit mailbox data byte length"]
pub type DTBL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Transmit mailbox time stamp transmit enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstenr {
    #[doc = "0: Mailbox time stamp transmit is disabled"]
    Disabled = 0,
    #[doc = "1: Mailbox time stamp transmit is enabled"]
    Enabled = 1,
}
impl From<Tstenr> for bool {
    #[inline(always)]
    fn from(variant: Tstenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTEN` reader - Transmit mailbox time stamp transmit enable"]
pub type TSTEN_R = crate::BitReader<Tstenr>;
impl TSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstenr {
        match self.bits {
            false => Tstenr::Disabled,
            true => Tstenr::Enabled,
        }
    }
    #[doc = "Mailbox time stamp transmit is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tstenr::Disabled
    }
    #[doc = "Mailbox time stamp transmit is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tstenr::Enabled
    }
}
#[doc = "Transmit mailbox time stamp transmit enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TstenwWO {
    #[doc = "0: Mailbox time stamp transmit disable"]
    Disable = 0,
    #[doc = "1: Mailbox time stamp transmit enable"]
    Enable = 1,
}
impl From<TstenwWO> for bool {
    #[inline(always)]
    fn from(variant: TstenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTEN` writer - Transmit mailbox time stamp transmit enable"]
pub type TSTEN_W<'a, REG> = crate::BitWriter<'a, REG, TstenwWO>;
impl<'a, REG> TSTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mailbox time stamp transmit disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TstenwWO::Disable)
    }
    #[doc = "Mailbox time stamp transmit enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TstenwWO::Enable)
    }
}
#[doc = "Field `TS` reader - Transmit mailbox time stamp"]
pub type TS_R = crate::FieldReader<u16>;
#[doc = "Field `TS` writer - Transmit mailbox time stamp"]
pub type TS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
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
            .field("ts", &self.ts())
            .field("tsten", &self.tsten())
            .field("dtbl", &self.dtbl())
            .finish()
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
}
#[doc = "Transmit mailbox data length and time stamp register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMC_SPEC;
impl crate::RegisterSpec for TMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmc::R`](R) reader structure"]
impl crate::Readable for TMC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmc::W`](W) writer structure"]
impl crate::Writable for TMC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMC to value 0"]
impl crate::Resettable for TMC_SPEC {
    const RESET_VALUE: u32 = 0;
}
