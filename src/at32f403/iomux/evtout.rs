#[doc = "Register `EVTOUT` reader"]
pub type R = crate::R<EVTOUT_SPEC>;
#[doc = "Register `EVTOUT` writer"]
pub type W = crate::W<EVTOUT_SPEC>;
#[doc = "Field `SELPIN` reader - Select pin"]
pub type SELPIN_R = crate::FieldReader;
#[doc = "Field `SELPIN` writer - Select pin"]
pub type SELPIN_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 4, O>;
#[doc = "Field `SELPORT` reader - Select port"]
pub type SELPORT_R = crate::FieldReader<SELPORT_A>;
#[doc = "Select port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELPORT_A {
    #[doc = "0: GPIOA"]
    Gpioa = 0,
    #[doc = "1: GPIOB"]
    Gpiob = 1,
    #[doc = "2: GPIOC"]
    Gpioc = 2,
    #[doc = "3: GPIOD"]
    Gpiod = 3,
    #[doc = "5: GPIOE"]
    Gpioe = 5,
    #[doc = "6: GPIOF"]
    Gpiof = 6,
    #[doc = "7: GPIOG"]
    Gpiog = 7,
}
impl From<SELPORT_A> for u8 {
    #[inline(always)]
    fn from(variant: SELPORT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SELPORT_A {
    type Ux = u8;
}
impl SELPORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SELPORT_A> {
        match self.bits {
            0 => Some(SELPORT_A::Gpioa),
            1 => Some(SELPORT_A::Gpiob),
            2 => Some(SELPORT_A::Gpioc),
            3 => Some(SELPORT_A::Gpiod),
            5 => Some(SELPORT_A::Gpioe),
            6 => Some(SELPORT_A::Gpiof),
            7 => Some(SELPORT_A::Gpiog),
            _ => None,
        }
    }
    #[doc = "GPIOA"]
    #[inline(always)]
    pub fn is_gpioa(&self) -> bool {
        *self == SELPORT_A::Gpioa
    }
    #[doc = "GPIOB"]
    #[inline(always)]
    pub fn is_gpiob(&self) -> bool {
        *self == SELPORT_A::Gpiob
    }
    #[doc = "GPIOC"]
    #[inline(always)]
    pub fn is_gpioc(&self) -> bool {
        *self == SELPORT_A::Gpioc
    }
    #[doc = "GPIOD"]
    #[inline(always)]
    pub fn is_gpiod(&self) -> bool {
        *self == SELPORT_A::Gpiod
    }
    #[doc = "GPIOE"]
    #[inline(always)]
    pub fn is_gpioe(&self) -> bool {
        *self == SELPORT_A::Gpioe
    }
    #[doc = "GPIOF"]
    #[inline(always)]
    pub fn is_gpiof(&self) -> bool {
        *self == SELPORT_A::Gpiof
    }
    #[doc = "GPIOG"]
    #[inline(always)]
    pub fn is_gpiog(&self) -> bool {
        *self == SELPORT_A::Gpiog
    }
}
#[doc = "Field `SELPORT` writer - Select port"]
pub type SELPORT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SELPORT_A>;
impl<'a, REG, const O: u8> SELPORT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIOA"]
    #[inline(always)]
    pub fn gpioa(self) -> &'a mut crate::W<REG> {
        self.variant(SELPORT_A::Gpioa)
    }
    #[doc = "GPIOB"]
    #[inline(always)]
    pub fn gpiob(self) -> &'a mut crate::W<REG> {
        self.variant(SELPORT_A::Gpiob)
    }
    #[doc = "GPIOC"]
    #[inline(always)]
    pub fn gpioc(self) -> &'a mut crate::W<REG> {
        self.variant(SELPORT_A::Gpioc)
    }
    #[doc = "GPIOD"]
    #[inline(always)]
    pub fn gpiod(self) -> &'a mut crate::W<REG> {
        self.variant(SELPORT_A::Gpiod)
    }
    #[doc = "GPIOE"]
    #[inline(always)]
    pub fn gpioe(self) -> &'a mut crate::W<REG> {
        self.variant(SELPORT_A::Gpioe)
    }
    #[doc = "GPIOF"]
    #[inline(always)]
    pub fn gpiof(self) -> &'a mut crate::W<REG> {
        self.variant(SELPORT_A::Gpiof)
    }
    #[doc = "GPIOG"]
    #[inline(always)]
    pub fn gpiog(self) -> &'a mut crate::W<REG> {
        self.variant(SELPORT_A::Gpiog)
    }
}
#[doc = "Field `EVOEN` reader - Event output enable"]
pub type EVOEN_R = crate::BitReader<EVOEN_A>;
#[doc = "Event output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVOEN_A {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<EVOEN_A> for bool {
    #[inline(always)]
    fn from(variant: EVOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl EVOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EVOEN_A {
        match self.bits {
            false => EVOEN_A::Disable,
            true => EVOEN_A::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EVOEN_A::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EVOEN_A::Enable
    }
}
#[doc = "Field `EVOEN` writer - Event output enable"]
pub type EVOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EVOEN_A>;
impl<'a, REG, const O: u8> EVOEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EVOEN_A::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EVOEN_A::Enable)
    }
}
impl R {
    #[doc = "Bits 0:3 - Select pin"]
    #[inline(always)]
    pub fn selpin(&self) -> SELPIN_R {
        SELPIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Select port"]
    #[inline(always)]
    pub fn selport(&self) -> SELPORT_R {
        SELPORT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Event output enable"]
    #[inline(always)]
    pub fn evoen(&self) -> EVOEN_R {
        EVOEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVTOUT")
            .field("selpin", &format_args!("{}", self.selpin().bits()))
            .field("selport", &format_args!("{}", self.selport().bits()))
            .field("evoen", &format_args!("{}", self.evoen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<EVTOUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select pin"]
    #[inline(always)]
    #[must_use]
    pub fn selpin(&mut self) -> SELPIN_W<EVTOUT_SPEC, 0> {
        SELPIN_W::new(self)
    }
    #[doc = "Bits 4:6 - Select port"]
    #[inline(always)]
    #[must_use]
    pub fn selport(&mut self) -> SELPORT_W<EVTOUT_SPEC, 4> {
        SELPORT_W::new(self)
    }
    #[doc = "Bit 7 - Event output enable"]
    #[inline(always)]
    #[must_use]
    pub fn evoen(&mut self) -> EVOEN_W<EVTOUT_SPEC, 7> {
        EVOEN_W::new(self)
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
#[doc = "Event output register (IOMUX_EVTOUT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVTOUT_SPEC;
impl crate::RegisterSpec for EVTOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evtout::R`](R) reader structure"]
impl crate::Readable for EVTOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evtout::W`](W) writer structure"]
impl crate::Writable for EVTOUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVTOUT to value 0"]
impl crate::Resettable for EVTOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
