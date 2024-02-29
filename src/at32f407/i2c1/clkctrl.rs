#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<CLKCTRL_SPEC>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<CLKCTRL_SPEC>;
#[doc = "Field `SPEED` reader - I2C bus speed config"]
pub type SPEED_R = crate::FieldReader<u16>;
#[doc = "Field `SPEED` writer - I2C bus speed config"]
pub type SPEED_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DUTYMODE` reader - Fast mode duty cycle"]
pub type DUTYMODE_R = crate::BitReader;
#[doc = "Field `DUTYMODE` writer - Fast mode duty cycle"]
pub type DUTYMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPEEDMODE` reader - Speed mode selection"]
pub type SPEEDMODE_R = crate::BitReader;
#[doc = "Field `SPEEDMODE` writer - Speed mode selection"]
pub type SPEEDMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("speedmode", &format_args!("{}", self.speedmode().bit()))
            .field("dutymode", &format_args!("{}", self.dutymode().bit()))
            .field("speed", &format_args!("{}", self.speed().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CLKCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:11 - I2C bus speed config"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<CLKCTRL_SPEC> {
        SPEED_W::new(self, 0)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    #[must_use]
    pub fn dutymode(&mut self) -> DUTYMODE_W<CLKCTRL_SPEC> {
        DUTYMODE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Speed mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn speedmode(&mut self) -> SPEEDMODE_W<CLKCTRL_SPEC> {
        SPEEDMODE_W::new(self, 15)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKCTRL_SPEC;
impl crate::RegisterSpec for CLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for CLKCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for CLKCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for CLKCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
