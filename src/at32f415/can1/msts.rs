#[doc = "Register `MSTS` reader"]
pub type R = crate::R<MSTS_SPEC>;
#[doc = "Register `MSTS` writer"]
pub type W = crate::W<MSTS_SPEC>;
#[doc = "Freeze mode confirm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FZC_A {
    #[doc = "0: The CAN is not in Freeze mode"]
    NotFreeze = 0,
    #[doc = "1: The CAN is in Freeze mode"]
    Freeze = 1,
}
impl From<FZC_A> for bool {
    #[inline(always)]
    fn from(variant: FZC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FZC` reader - Freeze mode confirm"]
pub type FZC_R = crate::BitReader<FZC_A>;
impl FZC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FZC_A {
        match self.bits {
            false => FZC_A::NotFreeze,
            true => FZC_A::Freeze,
        }
    }
    #[doc = "The CAN is not in Freeze mode"]
    #[inline(always)]
    pub fn is_not_freeze(&self) -> bool {
        *self == FZC_A::NotFreeze
    }
    #[doc = "The CAN is in Freeze mode"]
    #[inline(always)]
    pub fn is_freeze(&self) -> bool {
        *self == FZC_A::Freeze
    }
}
#[doc = "Doze mode confirm\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DZC_A {
    #[doc = "0: The CAN is not in Sleep mode"]
    NotSleep = 0,
    #[doc = "1: The CAN is in Sleep mode"]
    Sleep = 1,
}
impl From<DZC_A> for bool {
    #[inline(always)]
    fn from(variant: DZC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DZC` reader - Doze mode confirm"]
pub type DZC_R = crate::BitReader<DZC_A>;
impl DZC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DZC_A {
        match self.bits {
            false => DZC_A::NotSleep,
            true => DZC_A::Sleep,
        }
    }
    #[doc = "The CAN is not in Sleep mode"]
    #[inline(always)]
    pub fn is_not_sleep(&self) -> bool {
        *self == DZC_A::NotSleep
    }
    #[doc = "The CAN is in Sleep mode"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == DZC_A::Sleep
    }
}
#[doc = "Error occur Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eoifr {
    #[doc = "0: No error interrupt or no condition for error interrupt flag"]
    NoError = 0,
    #[doc = "1: Error interrupt is generated"]
    Error = 1,
}
impl From<Eoifr> for bool {
    #[inline(always)]
    fn from(variant: Eoifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOIF` reader - Error occur Interrupt flag"]
pub type EOIF_R = crate::BitReader<Eoifr>;
impl EOIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eoifr {
        match self.bits {
            false => Eoifr::NoError,
            true => Eoifr::Error,
        }
    }
    #[doc = "No error interrupt or no condition for error interrupt flag"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Eoifr::NoError
    }
    #[doc = "Error interrupt is generated"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Eoifr::Error
    }
}
#[doc = "Error occur Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EoifwWO {
    #[doc = "1: Clear error flag"]
    Clear = 1,
}
impl From<EoifwWO> for bool {
    #[inline(always)]
    fn from(variant: EoifwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOIF` writer - Error occur Interrupt flag"]
pub type EOIF_W<'a, REG> = crate::BitWriter1C<'a, REG, EoifwWO>;
impl<'a, REG> EOIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EoifwWO::Clear)
    }
}
#[doc = "Quit doze mode interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qdzifr {
    #[doc = "0: Sleep mode is not left or no condition for exit"]
    InSleep = 0,
    #[doc = "1: Sleep mode has been left or exit condition has generated"]
    SleepExit = 1,
}
impl From<Qdzifr> for bool {
    #[inline(always)]
    fn from(variant: Qdzifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QDZIF` reader - Quit doze mode interrupt flag"]
pub type QDZIF_R = crate::BitReader<Qdzifr>;
impl QDZIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qdzifr {
        match self.bits {
            false => Qdzifr::InSleep,
            true => Qdzifr::SleepExit,
        }
    }
    #[doc = "Sleep mode is not left or no condition for exit"]
    #[inline(always)]
    pub fn is_in_sleep(&self) -> bool {
        *self == Qdzifr::InSleep
    }
    #[doc = "Sleep mode has been left or exit condition has generated"]
    #[inline(always)]
    pub fn is_sleep_exit(&self) -> bool {
        *self == Qdzifr::SleepExit
    }
}
#[doc = "Quit doze mode interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QdzifwWO {
    #[doc = "1: Clear exit doze flag"]
    Clear = 1,
}
impl From<QdzifwWO> for bool {
    #[inline(always)]
    fn from(variant: QdzifwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QDZIF` writer - Quit doze mode interrupt flag"]
pub type QDZIF_W<'a, REG> = crate::BitWriter1C<'a, REG, QdzifwWO>;
impl<'a, REG> QDZIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear exit doze flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(QdzifwWO::Clear)
    }
}
#[doc = "Enter doze mode interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edzifr {
    #[doc = "0: Sleep mode is not entered or no condition for flag set"]
    NotEnter = 0,
    #[doc = "1: Sleep mode is entered"]
    EnterSleep = 1,
}
impl From<Edzifr> for bool {
    #[inline(always)]
    fn from(variant: Edzifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDZIF` reader - Enter doze mode interrupt flag"]
pub type EDZIF_R = crate::BitReader<Edzifr>;
impl EDZIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edzifr {
        match self.bits {
            false => Edzifr::NotEnter,
            true => Edzifr::EnterSleep,
        }
    }
    #[doc = "Sleep mode is not entered or no condition for flag set"]
    #[inline(always)]
    pub fn is_not_enter(&self) -> bool {
        *self == Edzifr::NotEnter
    }
    #[doc = "Sleep mode is entered"]
    #[inline(always)]
    pub fn is_enter_sleep(&self) -> bool {
        *self == Edzifr::EnterSleep
    }
}
#[doc = "Enter doze mode interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdzifwWO {
    #[doc = "1: Clear enter doze flag"]
    Clear = 1,
}
impl From<EdzifwWO> for bool {
    #[inline(always)]
    fn from(variant: EdzifwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDZIF` writer - Enter doze mode interrupt flag"]
pub type EDZIF_W<'a, REG> = crate::BitWriter1C<'a, REG, EdzifwWO>;
impl<'a, REG> EDZIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear enter doze flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EdzifwWO::Clear)
    }
}
#[doc = "Currently sending status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CUSS_A {
    #[doc = "0: No transmit occurs"]
    Idle = 0,
    #[doc = "1: Transmit is in progress"]
    InProgress = 1,
}
impl From<CUSS_A> for bool {
    #[inline(always)]
    fn from(variant: CUSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CUSS` reader - Currently sending status"]
pub type CUSS_R = crate::BitReader<CUSS_A>;
impl CUSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CUSS_A {
        match self.bits {
            false => CUSS_A::Idle,
            true => CUSS_A::InProgress,
        }
    }
    #[doc = "No transmit occurs"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == CUSS_A::Idle
    }
    #[doc = "Transmit is in progress"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == CUSS_A::InProgress
    }
}
#[doc = "Currently receiving status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CURS_A {
    #[doc = "0: No reception occurs"]
    Idle = 0,
    #[doc = "1: Reception is in progress"]
    InProgress = 1,
}
impl From<CURS_A> for bool {
    #[inline(always)]
    fn from(variant: CURS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CURS` reader - Currently receiving status"]
pub type CURS_R = crate::BitReader<CURS_A>;
impl CURS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CURS_A {
        match self.bits {
            false => CURS_A::Idle,
            true => CURS_A::InProgress,
        }
    }
    #[doc = "No reception occurs"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == CURS_A::Idle
    }
    #[doc = "Reception is in progress"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == CURS_A::InProgress
    }
}
#[doc = "Last sample level of RX pin\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSAMPRX_A {
    #[doc = "0: Low"]
    Low = 0,
    #[doc = "1: High"]
    High = 1,
}
impl From<LSAMPRX_A> for bool {
    #[inline(always)]
    fn from(variant: LSAMPRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSAMPRX` reader - Last sample level of RX pin"]
pub type LSAMPRX_R = crate::BitReader<LSAMPRX_A>;
impl LSAMPRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSAMPRX_A {
        match self.bits {
            false => LSAMPRX_A::Low,
            true => LSAMPRX_A::High,
        }
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LSAMPRX_A::Low
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LSAMPRX_A::High
    }
}
#[doc = "Real time level of RX pin\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REALRX_A {
    #[doc = "0: Low"]
    Low = 0,
    #[doc = "1: High"]
    High = 1,
}
impl From<REALRX_A> for bool {
    #[inline(always)]
    fn from(variant: REALRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REALRX` reader - Real time level of RX pin"]
pub type REALRX_R = crate::BitReader<REALRX_A>;
impl REALRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REALRX_A {
        match self.bits {
            false => REALRX_A::Low,
            true => REALRX_A::High,
        }
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == REALRX_A::Low
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == REALRX_A::High
    }
}
impl R {
    #[doc = "Bit 0 - Freeze mode confirm"]
    #[inline(always)]
    pub fn fzc(&self) -> FZC_R {
        FZC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Doze mode confirm"]
    #[inline(always)]
    pub fn dzc(&self) -> DZC_R {
        DZC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error occur Interrupt flag"]
    #[inline(always)]
    pub fn eoif(&self) -> EOIF_R {
        EOIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Quit doze mode interrupt flag"]
    #[inline(always)]
    pub fn qdzif(&self) -> QDZIF_R {
        QDZIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enter doze mode interrupt flag"]
    #[inline(always)]
    pub fn edzif(&self) -> EDZIF_R {
        EDZIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Currently sending status"]
    #[inline(always)]
    pub fn cuss(&self) -> CUSS_R {
        CUSS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Currently receiving status"]
    #[inline(always)]
    pub fn curs(&self) -> CURS_R {
        CURS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Last sample level of RX pin"]
    #[inline(always)]
    pub fn lsamprx(&self) -> LSAMPRX_R {
        LSAMPRX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Real time level of RX pin"]
    #[inline(always)]
    pub fn realrx(&self) -> REALRX_R {
        REALRX_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTS")
            .field("realrx", &self.realrx())
            .field("lsamprx", &self.lsamprx())
            .field("curs", &self.curs())
            .field("cuss", &self.cuss())
            .field("edzif", &self.edzif())
            .field("qdzif", &self.qdzif())
            .field("eoif", &self.eoif())
            .field("dzc", &self.dzc())
            .field("fzc", &self.fzc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Error occur Interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn eoif(&mut self) -> EOIF_W<MSTS_SPEC> {
        EOIF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Quit doze mode interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn qdzif(&mut self) -> QDZIF_W<MSTS_SPEC> {
        QDZIF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enter doze mode interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn edzif(&mut self) -> EDZIF_W<MSTS_SPEC> {
        EDZIF_W::new(self, 4)
    }
}
#[doc = "Main status register\n\nYou can [`read`](crate::Reg::read) this register and get [`msts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSTS_SPEC;
impl crate::RegisterSpec for MSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msts::R`](R) reader structure"]
impl crate::Readable for MSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msts::W`](W) writer structure"]
impl crate::Writable for MSTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x1c;
}
#[doc = "`reset()` method sets MSTS to value 0x0c02"]
impl crate::Resettable for MSTS_SPEC {
    const RESET_VALUE: u32 = 0x0c02;
}
