#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Primary TMR output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PTOS_A {
    #[doc = "0: Reset"]
    Reset = 0,
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "2: Update"]
    Update = 2,
    #[doc = "3: Compare pulse"]
    ComparePulse = 3,
    #[doc = "4: C1ORAW signal"]
    C1oraw = 4,
    #[doc = "5: C2ORAW signal"]
    C2oraw = 5,
    #[doc = "6: C3ORAW signal"]
    C3oraw = 6,
    #[doc = "7: C4ORAW signal"]
    C4oraw = 7,
}
impl From<PTOS_A> for u8 {
    #[inline(always)]
    fn from(variant: PTOS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PTOS_A {
    type Ux = u8;
}
impl crate::IsEnum for PTOS_A {}
#[doc = "Field `PTOS` reader - Primary TMR output selection"]
pub type PTOS_R = crate::FieldReader<PTOS_A>;
impl PTOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PTOS_A {
        match self.bits {
            0 => PTOS_A::Reset,
            1 => PTOS_A::Enable,
            2 => PTOS_A::Update,
            3 => PTOS_A::ComparePulse,
            4 => PTOS_A::C1oraw,
            5 => PTOS_A::C2oraw,
            6 => PTOS_A::C3oraw,
            7 => PTOS_A::C4oraw,
            _ => unreachable!(),
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PTOS_A::Reset
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PTOS_A::Enable
    }
    #[doc = "Update"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == PTOS_A::Update
    }
    #[doc = "Compare pulse"]
    #[inline(always)]
    pub fn is_compare_pulse(&self) -> bool {
        *self == PTOS_A::ComparePulse
    }
    #[doc = "C1ORAW signal"]
    #[inline(always)]
    pub fn is_c1oraw(&self) -> bool {
        *self == PTOS_A::C1oraw
    }
    #[doc = "C2ORAW signal"]
    #[inline(always)]
    pub fn is_c2oraw(&self) -> bool {
        *self == PTOS_A::C2oraw
    }
    #[doc = "C3ORAW signal"]
    #[inline(always)]
    pub fn is_c3oraw(&self) -> bool {
        *self == PTOS_A::C3oraw
    }
    #[doc = "C4ORAW signal"]
    #[inline(always)]
    pub fn is_c4oraw(&self) -> bool {
        *self == PTOS_A::C4oraw
    }
}
#[doc = "Field `PTOS` writer - Primary TMR output selection"]
pub type PTOS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PTOS_A, crate::Safe>;
impl<'a, REG> PTOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PTOS_A::Reset)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PTOS_A::Enable)
    }
    #[doc = "Update"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(PTOS_A::Update)
    }
    #[doc = "Compare pulse"]
    #[inline(always)]
    pub fn compare_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(PTOS_A::ComparePulse)
    }
    #[doc = "C1ORAW signal"]
    #[inline(always)]
    pub fn c1oraw(self) -> &'a mut crate::W<REG> {
        self.variant(PTOS_A::C1oraw)
    }
    #[doc = "C2ORAW signal"]
    #[inline(always)]
    pub fn c2oraw(self) -> &'a mut crate::W<REG> {
        self.variant(PTOS_A::C2oraw)
    }
    #[doc = "C3ORAW signal"]
    #[inline(always)]
    pub fn c3oraw(self) -> &'a mut crate::W<REG> {
        self.variant(PTOS_A::C3oraw)
    }
    #[doc = "C4ORAW signal"]
    #[inline(always)]
    pub fn c4oraw(self) -> &'a mut crate::W<REG> {
        self.variant(PTOS_A::C4oraw)
    }
}
impl R {
    #[doc = "Bits 4:6 - Primary TMR output selection"]
    #[inline(always)]
    pub fn ptos(&self) -> PTOS_R {
        PTOS_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2").field("ptos", &self.ptos()).finish()
    }
}
impl W {
    #[doc = "Bits 4:6 - Primary TMR output selection"]
    #[inline(always)]
    #[must_use]
    pub fn ptos(&mut self) -> PTOS_W<CTRL2_SPEC> {
        PTOS_W::new(self, 4)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: u32 = 0;
}
