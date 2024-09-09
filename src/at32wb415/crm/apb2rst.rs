#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<APB2RST_SPEC>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<APB2RST_SPEC>;
#[doc = "Field `IOMUX` reader - MUX function I/O reset"]
pub type IOMUX_R = crate::BitReader;
#[doc = "Field `IOMUX` writer - MUX function I/O reset"]
pub type IOMUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXINT` reader - External interrupt reset"]
pub type EXINT_R = crate::BitReader;
#[doc = "Field `EXINT` writer - External interrupt reset"]
pub type EXINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOA` reader - IO port A reset"]
pub type GPIOA_R = crate::BitReader;
#[doc = "Field `GPIOA` writer - IO port A reset"]
pub type GPIOA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOB` reader - IO port B reset"]
pub type GPIOB_R = crate::BitReader;
#[doc = "Field `GPIOB` writer - IO port B reset"]
pub type GPIOB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOC` reader - IO port C reset"]
pub type GPIOC_R = crate::BitReader;
#[doc = "Field `GPIOC` writer - IO port C reset"]
pub type GPIOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOD` reader - IO port D reset"]
pub type GPIOD_R = crate::BitReader;
#[doc = "Field `GPIOD` writer - IO port D reset"]
pub type GPIOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOF` reader - IO port F reset"]
pub type GPIOF_R = crate::BitReader;
#[doc = "Field `GPIOF` writer - IO port F reset"]
pub type GPIOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1` reader - ADC1 reset"]
pub type ADC1_R = crate::BitReader;
#[doc = "Field `ADC1` writer - ADC1 reset"]
pub type ADC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1` reader - Timer1 reset"]
pub type TMR1_R = crate::BitReader;
#[doc = "Field `TMR1` writer - Timer1 reset"]
pub type TMR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1` reader - USART1 reset"]
pub type USART1_R = crate::BitReader;
#[doc = "Field `USART1` writer - USART1 reset"]
pub type USART1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR9` reader - Timer9 reset"]
pub type TMR9_R = crate::BitReader;
#[doc = "Field `TMR9` writer - Timer9 reset"]
pub type TMR9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR10` reader - Timer10 reset"]
pub type TMR10_R = crate::BitReader;
#[doc = "Field `TMR10` writer - Timer10 reset"]
pub type TMR10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR11` reader - Timer11 reset"]
pub type TMR11_R = crate::BitReader;
#[doc = "Field `TMR11` writer - Timer11 reset"]
pub type TMR11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC` reader - ACC reset"]
pub type ACC_R = crate::BitReader;
#[doc = "Field `ACC` writer - ACC reset"]
pub type ACC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MUX function I/O reset"]
    #[inline(always)]
    pub fn iomux(&self) -> IOMUX_R {
        IOMUX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External interrupt reset"]
    #[inline(always)]
    pub fn exint(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port A reset"]
    #[inline(always)]
    pub fn gpioa(&self) -> GPIOA_R {
        GPIOA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port B reset"]
    #[inline(always)]
    pub fn gpiob(&self) -> GPIOB_R {
        GPIOB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port C reset"]
    #[inline(always)]
    pub fn gpioc(&self) -> GPIOC_R {
        GPIOC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port D reset"]
    #[inline(always)]
    pub fn gpiod(&self) -> GPIOD_R {
        GPIOD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port F reset"]
    #[inline(always)]
    pub fn gpiof(&self) -> GPIOF_R {
        GPIOF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 reset"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer1 reset"]
    #[inline(always)]
    pub fn tmr1(&self) -> TMR1_R {
        TMR1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer9 reset"]
    #[inline(always)]
    pub fn tmr9(&self) -> TMR9_R {
        TMR9_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer10 reset"]
    #[inline(always)]
    pub fn tmr10(&self) -> TMR10_R {
        TMR10_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer11 reset"]
    #[inline(always)]
    pub fn tmr11(&self) -> TMR11_R {
        TMR11_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ACC reset"]
    #[inline(always)]
    pub fn acc(&self) -> ACC_R {
        ACC_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RST")
            .field("iomux", &self.iomux())
            .field("exint", &self.exint())
            .field("gpioa", &self.gpioa())
            .field("gpiob", &self.gpiob())
            .field("gpioc", &self.gpioc())
            .field("gpiod", &self.gpiod())
            .field("gpiof", &self.gpiof())
            .field("adc1", &self.adc1())
            .field("tmr1", &self.tmr1())
            .field("usart1", &self.usart1())
            .field("tmr9", &self.tmr9())
            .field("tmr10", &self.tmr10())
            .field("tmr11", &self.tmr11())
            .field("acc", &self.acc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - MUX function I/O reset"]
    #[inline(always)]
    #[must_use]
    pub fn iomux(&mut self) -> IOMUX_W<APB2RST_SPEC> {
        IOMUX_W::new(self, 0)
    }
    #[doc = "Bit 1 - External interrupt reset"]
    #[inline(always)]
    #[must_use]
    pub fn exint(&mut self) -> EXINT_W<APB2RST_SPEC> {
        EXINT_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioa(&mut self) -> GPIOA_W<APB2RST_SPEC> {
        GPIOA_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiob(&mut self) -> GPIOB_W<APB2RST_SPEC> {
        GPIOB_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioc(&mut self) -> GPIOC_W<APB2RST_SPEC> {
        GPIOC_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiod(&mut self) -> GPIOD_W<APB2RST_SPEC> {
        GPIOD_W::new(self, 5)
    }
    #[doc = "Bit 7 - IO port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiof(&mut self) -> GPIOF_W<APB2RST_SPEC> {
        GPIOF_W::new(self, 7)
    }
    #[doc = "Bit 9 - ADC1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc1(&mut self) -> ADC1_W<APB2RST_SPEC> {
        ADC1_W::new(self, 9)
    }
    #[doc = "Bit 11 - Timer1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1(&mut self) -> TMR1_W<APB2RST_SPEC> {
        TMR1_W::new(self, 11)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1(&mut self) -> USART1_W<APB2RST_SPEC> {
        USART1_W::new(self, 14)
    }
    #[doc = "Bit 19 - Timer9 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9(&mut self) -> TMR9_W<APB2RST_SPEC> {
        TMR9_W::new(self, 19)
    }
    #[doc = "Bit 20 - Timer10 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10(&mut self) -> TMR10_W<APB2RST_SPEC> {
        TMR10_W::new(self, 20)
    }
    #[doc = "Bit 21 - Timer11 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11(&mut self) -> TMR11_W<APB2RST_SPEC> {
        TMR11_W::new(self, 21)
    }
    #[doc = "Bit 22 - ACC reset"]
    #[inline(always)]
    #[must_use]
    pub fn acc(&mut self) -> ACC_W<APB2RST_SPEC> {
        ACC_W::new(self, 22)
    }
}
#[doc = "APB2 peripheral reset register (CRM_APB2RST)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2RST_SPEC;
impl crate::RegisterSpec for APB2RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rst::R`](R) reader structure"]
impl crate::Readable for APB2RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2rst::W`](W) writer structure"]
impl crate::Writable for APB2RST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2RST to value 0"]
impl crate::Resettable for APB2RST_SPEC {
    const RESET_VALUE: u32 = 0;
}
