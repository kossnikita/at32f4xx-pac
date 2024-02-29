#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Tamper pin enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpenr {
    #[doc = "0: TAMPER pin is disabled"]
    Disabled = 0,
    #[doc = "1: TAMPER pin is enabled"]
    Enabled = 1,
}
impl From<Tpenr> for bool {
    #[inline(always)]
    fn from(variant: Tpenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPEN` reader - Tamper pin enable"]
pub type TPEN_R = crate::BitReader<Tpenr>;
impl TPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpenr {
        match self.bits {
            false => Tpenr::Disabled,
            true => Tpenr::Enabled,
        }
    }
    #[doc = "TAMPER pin is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tpenr::Disabled
    }
    #[doc = "TAMPER pin is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tpenr::Enabled
    }
}
#[doc = "Tamper pin enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TpenwWO {
    #[doc = "0: TAMPER pin disable"]
    Disable = 0,
    #[doc = "1: TAMPER pin enable"]
    Enable = 1,
}
impl From<TpenwWO> for bool {
    #[inline(always)]
    fn from(variant: TpenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPEN` writer - Tamper pin enable"]
pub type TPEN_W<'a, REG> = crate::BitWriter<'a, REG, TpenwWO>;
impl<'a, REG> TPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TAMPER pin disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TpenwWO::Disable)
    }
    #[doc = "TAMPER pin enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TpenwWO::Enable)
    }
}
#[doc = "TAMPER pin polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPP_A {
    #[doc = "0: Active high"]
    High = 0,
    #[doc = "1: Active low"]
    Low = 1,
}
impl From<TPP_A> for bool {
    #[inline(always)]
    fn from(variant: TPP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPP` reader - TAMPER pin polarity"]
pub type TPP_R = crate::BitReader<TPP_A>;
impl TPP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TPP_A {
        match self.bits {
            false => TPP_A::High,
            true => TPP_A::Low,
        }
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TPP_A::High
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TPP_A::Low
    }
}
#[doc = "Field `TPP` writer - TAMPER pin polarity"]
pub type TPP_W<'a, REG> = crate::BitWriter<'a, REG, TPP_A>;
impl<'a, REG> TPP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(TPP_A::High)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(TPP_A::Low)
    }
}
impl R {
    #[doc = "Bit 0 - Tamper pin enable"]
    #[inline(always)]
    pub fn tpen(&self) -> TPEN_R {
        TPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMPER pin polarity"]
    #[inline(always)]
    pub fn tpp(&self) -> TPP_R {
        TPP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("tpen", &format_args!("{}", self.tpen().bit()))
            .field("tpp", &format_args!("{}", self.tpp().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpen(&mut self) -> TPEN_W<CTRL_SPEC> {
        TPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TAMPER pin polarity"]
    #[inline(always)]
    #[must_use]
    pub fn tpp(&mut self) -> TPP_W<CTRL_SPEC> {
        TPP_W::new(self, 1)
    }
}
#[doc = "BPR control register (BPR_CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
