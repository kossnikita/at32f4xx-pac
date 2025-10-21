#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<CLKCTRL_SPEC>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<CLKCTRL_SPEC>;
#[doc = "Field `SPEED` reader - I2C bus speed config"]
pub type SPEED_R = crate::FieldReader<u16>;
#[doc = "Field `SPEED` writer - I2C bus speed config"]
pub type SPEED_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
#[doc = "Fast mode duty cycle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUTYMODE_A {
    #[doc = "0: The ratio of low to high is 2:1"]
    Duty2_1 = 0,
    #[doc = "1: The ratio of low to high is 16:9"]
    Duty16_9 = 1,
}
impl From<DUTYMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DUTYMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUTYMODE` reader - Fast mode duty cycle"]
pub type DUTYMODE_R = crate::BitReader<DUTYMODE_A>;
impl DUTYMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DUTYMODE_A {
        match self.bits {
            false => DUTYMODE_A::Duty2_1,
            true => DUTYMODE_A::Duty16_9,
        }
    }
    #[doc = "The ratio of low to high is 2:1"]
    #[inline(always)]
    pub fn is_duty2_1(&self) -> bool {
        *self == DUTYMODE_A::Duty2_1
    }
    #[doc = "The ratio of low to high is 16:9"]
    #[inline(always)]
    pub fn is_duty16_9(&self) -> bool {
        *self == DUTYMODE_A::Duty16_9
    }
}
#[doc = "Field `DUTYMODE` writer - Fast mode duty cycle"]
pub type DUTYMODE_W<'a, REG> = crate::BitWriter<'a, REG, DUTYMODE_A>;
impl<'a, REG> DUTYMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The ratio of low to high is 2:1"]
    #[inline(always)]
    pub fn duty2_1(self) -> &'a mut crate::W<REG> {
        self.variant(DUTYMODE_A::Duty2_1)
    }
    #[doc = "The ratio of low to high is 16:9"]
    #[inline(always)]
    pub fn duty16_9(self) -> &'a mut crate::W<REG> {
        self.variant(DUTYMODE_A::Duty16_9)
    }
}
#[doc = "Speed mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPEEDMODE_A {
    #[doc = "0: Standard mode (up to 100 kHz)"]
    Standard = 0,
    #[doc = "1: Fast mode (up to 400 kHz)"]
    Fast = 1,
}
impl From<SPEEDMODE_A> for bool {
    #[inline(always)]
    fn from(variant: SPEEDMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPEEDMODE` reader - Speed mode selection"]
pub type SPEEDMODE_R = crate::BitReader<SPEEDMODE_A>;
impl SPEEDMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPEEDMODE_A {
        match self.bits {
            false => SPEEDMODE_A::Standard,
            true => SPEEDMODE_A::Fast,
        }
    }
    #[doc = "Standard mode (up to 100 kHz)"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == SPEEDMODE_A::Standard
    }
    #[doc = "Fast mode (up to 400 kHz)"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == SPEEDMODE_A::Fast
    }
}
#[doc = "Field `SPEEDMODE` writer - Speed mode selection"]
pub type SPEEDMODE_W<'a, REG> = crate::BitWriter<'a, REG, SPEEDMODE_A>;
impl<'a, REG> SPEEDMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard mode (up to 100 kHz)"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(SPEEDMODE_A::Standard)
    }
    #[doc = "Fast mode (up to 400 kHz)"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(SPEEDMODE_A::Fast)
    }
}
impl R {
    #[doc = "Bits 0:11 - I2C bus speed config"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    pub fn dutymode(&self) -> DUTYMODE_R {
        DUTYMODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Speed mode selection"]
    #[inline(always)]
    pub fn speedmode(&self) -> SPEEDMODE_R {
        SPEEDMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKCTRL")
            .field("speedmode", &self.speedmode())
            .field("dutymode", &self.dutymode())
            .field("speed", &self.speed())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - I2C bus speed config"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W<'_, CLKCTRL_SPEC> {
        SPEED_W::new(self, 0)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    pub fn dutymode(&mut self) -> DUTYMODE_W<'_, CLKCTRL_SPEC> {
        DUTYMODE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Speed mode selection"]
    #[inline(always)]
    pub fn speedmode(&mut self) -> SPEEDMODE_W<'_, CLKCTRL_SPEC> {
        SPEEDMODE_W::new(self, 15)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKCTRL_SPEC;
impl crate::RegisterSpec for CLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for CLKCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for CLKCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for CLKCTRL_SPEC {}
