#[doc = "Register `EVTOUT` reader"]
pub type R = crate::R<EVTOUT_SPEC>;
#[doc = "Register `EVTOUT` writer"]
pub type W = crate::W<EVTOUT_SPEC>;
#[doc = "Field `SELPIN` reader - Select pin"]
pub type SELPIN_R = crate::FieldReader;
#[doc = "Field `SELPIN` writer - Select pin"]
pub type SELPIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
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
    #[doc = "6: GPIOF"]
    Gpiof = 6,
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
impl crate::IsEnum for SELPORT_A {}
#[doc = "Field `SELPORT` reader - Select port"]
pub type SELPORT_R = crate::FieldReader<SELPORT_A>;
impl SELPORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SELPORT_A> {
        match self.bits {
            0 => Some(SELPORT_A::Gpioa),
            1 => Some(SELPORT_A::Gpiob),
            2 => Some(SELPORT_A::Gpioc),
            3 => Some(SELPORT_A::Gpiod),
            6 => Some(SELPORT_A::Gpiof),
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
    #[doc = "GPIOF"]
    #[inline(always)]
    pub fn is_gpiof(&self) -> bool {
        *self == SELPORT_A::Gpiof
    }
}
#[doc = "Field `SELPORT` writer - Select port"]
pub type SELPORT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SELPORT_A>;
impl<'a, REG> SELPORT_W<'a, REG>
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
    #[doc = "GPIOF"]
    #[inline(always)]
    pub fn gpiof(self) -> &'a mut crate::W<REG> {
        self.variant(SELPORT_A::Gpiof)
    }
}
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
#[doc = "Field `EVOEN` reader - Event output enable"]
pub type EVOEN_R = crate::BitReader<EVOEN_A>;
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
pub type EVOEN_W<'a, REG> = crate::BitWriter<'a, REG, EVOEN_A>;
impl<'a, REG> EVOEN_W<'a, REG>
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
            .field("selpin", &self.selpin())
            .field("selport", &self.selport())
            .field("evoen", &self.evoen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Select pin"]
    #[inline(always)]
    pub fn selpin(&mut self) -> SELPIN_W<'_, EVTOUT_SPEC> {
        SELPIN_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Select port"]
    #[inline(always)]
    pub fn selport(&mut self) -> SELPORT_W<'_, EVTOUT_SPEC> {
        SELPORT_W::new(self, 4)
    }
    #[doc = "Bit 7 - Event output enable"]
    #[inline(always)]
    pub fn evoen(&mut self) -> EVOEN_W<'_, EVTOUT_SPEC> {
        EVOEN_W::new(self, 7)
    }
}
#[doc = "Event output register\n\nYou can [`read`](crate::Reg::read) this register and get [`evtout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evtout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVTOUT_SPEC;
impl crate::RegisterSpec for EVTOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evtout::R`](R) reader structure"]
impl crate::Readable for EVTOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evtout::W`](W) writer structure"]
impl crate::Writable for EVTOUT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVTOUT to value 0"]
impl crate::Resettable for EVTOUT_SPEC {}
