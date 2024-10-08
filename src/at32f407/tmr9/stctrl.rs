#[doc = "Register `STCTRL` reader"]
pub type R = crate::R<STCTRL_SPEC>;
#[doc = "Register `STCTRL` writer"]
pub type W = crate::W<STCTRL_SPEC>;
#[doc = "Subordinate TMR mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMSEL_A {
    #[doc = "0: Slave mode is disabled"]
    Disabled = 0,
    #[doc = "1: Encoder mode A"]
    EncoderA = 1,
    #[doc = "2: Encoder mode B"]
    EncoderB = 2,
    #[doc = "3: Encoder mode C"]
    EncoderC = 3,
    #[doc = "4: Reset mode - Rising edge of the TRGIN input reinitializes the counter"]
    Reset = 4,
    #[doc = "5: Suspend mode - The counter starts counting when the TRGIN is high"]
    Suspend = 5,
    #[doc = "6: Trigger mode - A trigger event is generated at the rising edge of the TRGIN input"]
    Trigger = 6,
    #[doc = "7: External clock mode A - Rising edge of the TRGIN input clocks the counter"]
    External = 7,
}
impl From<SMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SMSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for SMSEL_A {}
#[doc = "Field `SMSEL` reader - Subordinate TMR mode selection"]
pub type SMSEL_R = crate::FieldReader<SMSEL_A>;
impl SMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMSEL_A {
        match self.bits {
            0 => SMSEL_A::Disabled,
            1 => SMSEL_A::EncoderA,
            2 => SMSEL_A::EncoderB,
            3 => SMSEL_A::EncoderC,
            4 => SMSEL_A::Reset,
            5 => SMSEL_A::Suspend,
            6 => SMSEL_A::Trigger,
            7 => SMSEL_A::External,
            _ => unreachable!(),
        }
    }
    #[doc = "Slave mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMSEL_A::Disabled
    }
    #[doc = "Encoder mode A"]
    #[inline(always)]
    pub fn is_encoder_a(&self) -> bool {
        *self == SMSEL_A::EncoderA
    }
    #[doc = "Encoder mode B"]
    #[inline(always)]
    pub fn is_encoder_b(&self) -> bool {
        *self == SMSEL_A::EncoderB
    }
    #[doc = "Encoder mode C"]
    #[inline(always)]
    pub fn is_encoder_c(&self) -> bool {
        *self == SMSEL_A::EncoderC
    }
    #[doc = "Reset mode - Rising edge of the TRGIN input reinitializes the counter"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SMSEL_A::Reset
    }
    #[doc = "Suspend mode - The counter starts counting when the TRGIN is high"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == SMSEL_A::Suspend
    }
    #[doc = "Trigger mode - A trigger event is generated at the rising edge of the TRGIN input"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == SMSEL_A::Trigger
    }
    #[doc = "External clock mode A - Rising edge of the TRGIN input clocks the counter"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == SMSEL_A::External
    }
}
#[doc = "Field `SMSEL` writer - Subordinate TMR mode selection"]
pub type SMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMSEL_A, crate::Safe>;
impl<'a, REG> SMSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slave mode is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMSEL_A::Disabled)
    }
    #[doc = "Encoder mode A"]
    #[inline(always)]
    pub fn encoder_a(self) -> &'a mut crate::W<REG> {
        self.variant(SMSEL_A::EncoderA)
    }
    #[doc = "Encoder mode B"]
    #[inline(always)]
    pub fn encoder_b(self) -> &'a mut crate::W<REG> {
        self.variant(SMSEL_A::EncoderB)
    }
    #[doc = "Encoder mode C"]
    #[inline(always)]
    pub fn encoder_c(self) -> &'a mut crate::W<REG> {
        self.variant(SMSEL_A::EncoderC)
    }
    #[doc = "Reset mode - Rising edge of the TRGIN input reinitializes the counter"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SMSEL_A::Reset)
    }
    #[doc = "Suspend mode - The counter starts counting when the TRGIN is high"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(SMSEL_A::Suspend)
    }
    #[doc = "Trigger mode - A trigger event is generated at the rising edge of the TRGIN input"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(SMSEL_A::Trigger)
    }
    #[doc = "External clock mode A - Rising edge of the TRGIN input clocks the counter"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(SMSEL_A::External)
    }
}
#[doc = "Subordinate TMR input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STIS_A {
    #[doc = "0: Internal selection 0"]
    Is0 = 0,
    #[doc = "1: Internal selection 1"]
    Is1 = 1,
    #[doc = "2: Internal selection 2"]
    Is2 = 2,
    #[doc = "3: Internal selection 3"]
    Is3 = 3,
    #[doc = "4: C1IRAW input detector"]
    C1inc = 4,
    #[doc = "5: Filtered input 1"]
    C1if1 = 5,
    #[doc = "6: Filtered input 2"]
    C1if2 = 6,
    #[doc = "7: External input"]
    Ext = 7,
}
impl From<STIS_A> for u8 {
    #[inline(always)]
    fn from(variant: STIS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STIS_A {
    type Ux = u8;
}
impl crate::IsEnum for STIS_A {}
#[doc = "Field `STIS` reader - Subordinate TMR input selection"]
pub type STIS_R = crate::FieldReader<STIS_A>;
impl STIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STIS_A {
        match self.bits {
            0 => STIS_A::Is0,
            1 => STIS_A::Is1,
            2 => STIS_A::Is2,
            3 => STIS_A::Is3,
            4 => STIS_A::C1inc,
            5 => STIS_A::C1if1,
            6 => STIS_A::C1if2,
            7 => STIS_A::Ext,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal selection 0"]
    #[inline(always)]
    pub fn is_is0(&self) -> bool {
        *self == STIS_A::Is0
    }
    #[doc = "Internal selection 1"]
    #[inline(always)]
    pub fn is_is1(&self) -> bool {
        *self == STIS_A::Is1
    }
    #[doc = "Internal selection 2"]
    #[inline(always)]
    pub fn is_is2(&self) -> bool {
        *self == STIS_A::Is2
    }
    #[doc = "Internal selection 3"]
    #[inline(always)]
    pub fn is_is3(&self) -> bool {
        *self == STIS_A::Is3
    }
    #[doc = "C1IRAW input detector"]
    #[inline(always)]
    pub fn is_c1inc(&self) -> bool {
        *self == STIS_A::C1inc
    }
    #[doc = "Filtered input 1"]
    #[inline(always)]
    pub fn is_c1if1(&self) -> bool {
        *self == STIS_A::C1if1
    }
    #[doc = "Filtered input 2"]
    #[inline(always)]
    pub fn is_c1if2(&self) -> bool {
        *self == STIS_A::C1if2
    }
    #[doc = "External input"]
    #[inline(always)]
    pub fn is_ext(&self) -> bool {
        *self == STIS_A::Ext
    }
}
#[doc = "Field `STIS` writer - Subordinate TMR input selection"]
pub type STIS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, STIS_A, crate::Safe>;
impl<'a, REG> STIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal selection 0"]
    #[inline(always)]
    pub fn is0(self) -> &'a mut crate::W<REG> {
        self.variant(STIS_A::Is0)
    }
    #[doc = "Internal selection 1"]
    #[inline(always)]
    pub fn is1(self) -> &'a mut crate::W<REG> {
        self.variant(STIS_A::Is1)
    }
    #[doc = "Internal selection 2"]
    #[inline(always)]
    pub fn is2(self) -> &'a mut crate::W<REG> {
        self.variant(STIS_A::Is2)
    }
    #[doc = "Internal selection 3"]
    #[inline(always)]
    pub fn is3(self) -> &'a mut crate::W<REG> {
        self.variant(STIS_A::Is3)
    }
    #[doc = "C1IRAW input detector"]
    #[inline(always)]
    pub fn c1inc(self) -> &'a mut crate::W<REG> {
        self.variant(STIS_A::C1inc)
    }
    #[doc = "Filtered input 1"]
    #[inline(always)]
    pub fn c1if1(self) -> &'a mut crate::W<REG> {
        self.variant(STIS_A::C1if1)
    }
    #[doc = "Filtered input 2"]
    #[inline(always)]
    pub fn c1if2(self) -> &'a mut crate::W<REG> {
        self.variant(STIS_A::C1if2)
    }
    #[doc = "External input"]
    #[inline(always)]
    pub fn ext(self) -> &'a mut crate::W<REG> {
        self.variant(STIS_A::Ext)
    }
}
impl R {
    #[doc = "Bits 0:2 - Subordinate TMR mode selection"]
    #[inline(always)]
    pub fn smsel(&self) -> SMSEL_R {
        SMSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Subordinate TMR input selection"]
    #[inline(always)]
    pub fn stis(&self) -> STIS_R {
        STIS_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STCTRL")
            .field("stis", &self.stis())
            .field("smsel", &self.smsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Subordinate TMR mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn smsel(&mut self) -> SMSEL_W<STCTRL_SPEC> {
        SMSEL_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Subordinate TMR input selection"]
    #[inline(always)]
    #[must_use]
    pub fn stis(&mut self) -> STIS_W<STCTRL_SPEC> {
        STIS_W::new(self, 4)
    }
}
#[doc = "Subordinate TMR control register\n\nYou can [`read`](crate::Reg::read) this register and get [`stctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STCTRL_SPEC;
impl crate::RegisterSpec for STCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stctrl::R`](R) reader structure"]
impl crate::Readable for STCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stctrl::W`](W) writer structure"]
impl crate::Writable for STCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCTRL to value 0"]
impl crate::Resettable for STCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
