#[doc = "Register `APB2_PAUSE` reader"]
pub type R = crate::R<APB2_PAUSE_SPEC>;
#[doc = "Register `APB2_PAUSE` writer"]
pub type W = crate::W<APB2_PAUSE_SPEC>;
#[doc = "Field `TMR1_PAUSE` reader - TMR1_PAUSE"]
pub type TMR1_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR1_PAUSE` writer - TMR1_PAUSE"]
pub type TMR1_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR8_PAUSE` reader - TMR8_PAUSE"]
pub type TMR8_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR8_PAUSE` writer - TMR8_PAUSE"]
pub type TMR8_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR20_PAUSE` reader - TIM20_PAUSE"]
pub type TMR20_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR20_PAUSE` writer - TIM20_PAUSE"]
pub type TMR20_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR9_PAUSE` reader - TMR9_PAUSE"]
pub type TMR9_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR9_PAUSE` writer - TMR9_PAUSE"]
pub type TMR9_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR10_PAUSE` reader - TMR10_PAUSE"]
pub type TMR10_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR10_PAUSE` writer - TMR10_PAUSE"]
pub type TMR10_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR11_PAUSE` reader - TMR11_PAUSE"]
pub type TMR11_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR11_PAUSE` writer - TMR11_PAUSE"]
pub type TMR11_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TMR1_PAUSE"]
    #[inline(always)]
    pub fn tmr1_pause(&self) -> TMR1_PAUSE_R {
        TMR1_PAUSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TMR8_PAUSE"]
    #[inline(always)]
    pub fn tmr8_pause(&self) -> TMR8_PAUSE_R {
        TMR8_PAUSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM20_PAUSE"]
    #[inline(always)]
    pub fn tmr20_pause(&self) -> TMR20_PAUSE_R {
        TMR20_PAUSE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - TMR9_PAUSE"]
    #[inline(always)]
    pub fn tmr9_pause(&self) -> TMR9_PAUSE_R {
        TMR9_PAUSE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TMR10_PAUSE"]
    #[inline(always)]
    pub fn tmr10_pause(&self) -> TMR10_PAUSE_R {
        TMR10_PAUSE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TMR11_PAUSE"]
    #[inline(always)]
    pub fn tmr11_pause(&self) -> TMR11_PAUSE_R {
        TMR11_PAUSE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2_PAUSE")
            .field("tmr1_pause", &self.tmr1_pause())
            .field("tmr8_pause", &self.tmr8_pause())
            .field("tmr20_pause", &self.tmr20_pause())
            .field("tmr9_pause", &self.tmr9_pause())
            .field("tmr10_pause", &self.tmr10_pause())
            .field("tmr11_pause", &self.tmr11_pause())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - TMR1_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1_pause(&mut self) -> TMR1_PAUSE_W<APB2_PAUSE_SPEC> {
        TMR1_PAUSE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TMR8_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr8_pause(&mut self) -> TMR8_PAUSE_W<APB2_PAUSE_SPEC> {
        TMR8_PAUSE_W::new(self, 1)
    }
    #[doc = "Bit 6 - TIM20_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr20_pause(&mut self) -> TMR20_PAUSE_W<APB2_PAUSE_SPEC> {
        TMR20_PAUSE_W::new(self, 6)
    }
    #[doc = "Bit 16 - TMR9_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9_pause(&mut self) -> TMR9_PAUSE_W<APB2_PAUSE_SPEC> {
        TMR9_PAUSE_W::new(self, 16)
    }
    #[doc = "Bit 17 - TMR10_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10_pause(&mut self) -> TMR10_PAUSE_W<APB2_PAUSE_SPEC> {
        TMR10_PAUSE_W::new(self, 17)
    }
    #[doc = "Bit 18 - TMR11_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11_pause(&mut self) -> TMR11_PAUSE_W<APB2_PAUSE_SPEC> {
        TMR11_PAUSE_W::new(self, 18)
    }
}
#[doc = "DEBUG APB2 PAUSE\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2_pause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2_pause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2_PAUSE_SPEC;
impl crate::RegisterSpec for APB2_PAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2_pause::R`](R) reader structure"]
impl crate::Readable for APB2_PAUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2_pause::W`](W) writer structure"]
impl crate::Writable for APB2_PAUSE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2_PAUSE to value 0"]
impl crate::Resettable for APB2_PAUSE_SPEC {
    const RESET_VALUE: u32 = 0;
}
