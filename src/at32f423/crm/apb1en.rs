#[doc = "Register `APB1EN` reader"]
pub type R = crate::R<APB1EN_SPEC>;
#[doc = "Register `APB1EN` writer"]
pub type W = crate::W<APB1EN_SPEC>;
#[doc = "Field `TMR2` reader - Timer2 clock enable"]
pub type TMR2_R = crate::BitReader;
#[doc = "Field `TMR2` writer - Timer2 clock enable"]
pub type TMR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR3` reader - Timer3 clock enable"]
pub type TMR3_R = crate::BitReader;
#[doc = "Field `TMR3` writer - Timer3 clock enable"]
pub type TMR3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR4` reader - Timer4 clock enable"]
pub type TMR4_R = crate::BitReader;
#[doc = "Field `TMR4` writer - Timer4 clock enable"]
pub type TMR4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR6` reader - Timer6 clock enable"]
pub type TMR6_R = crate::BitReader;
#[doc = "Field `TMR6` writer - Timer6 clock enable"]
pub type TMR6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR7` reader - Timer7 clock enable"]
pub type TMR7_R = crate::BitReader;
#[doc = "Field `TMR7` writer - Timer7 clock enable"]
pub type TMR7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR12` reader - Timer12 clock enable"]
pub type TMR12_R = crate::BitReader;
#[doc = "Field `TMR12` writer - Timer12 clock enable"]
pub type TMR12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR13` reader - Timer13 clock enable"]
pub type TMR13_R = crate::BitReader;
#[doc = "Field `TMR13` writer - Timer13 clock enable"]
pub type TMR13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR14` reader - Timer14 clock enable"]
pub type TMR14_R = crate::BitReader;
#[doc = "Field `TMR14` writer - Timer14 clock enable"]
pub type TMR14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WWDT` reader - WWDT clock enable"]
pub type WWDT_R = crate::BitReader;
#[doc = "Field `WWDT` writer - WWDT clock enable"]
pub type WWDT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI2` reader - SPI2 clock enable"]
pub type SPI2_R = crate::BitReader;
#[doc = "Field `SPI2` writer - SPI2 clock enable"]
pub type SPI2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI3` reader - SPI3 clock enable"]
pub type SPI3_R = crate::BitReader;
#[doc = "Field `SPI3` writer - SPI3 clock enable"]
pub type SPI3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART2` reader - USART2 clock enable"]
pub type USART2_R = crate::BitReader;
#[doc = "Field `USART2` writer - USART2 clock enable"]
pub type USART2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART3` reader - USART3 clock enable"]
pub type USART3_R = crate::BitReader;
#[doc = "Field `USART3` writer - USART3 clock enable"]
pub type USART3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART4` reader - USART4 clock enable"]
pub type USART4_R = crate::BitReader;
#[doc = "Field `USART4` writer - USART4 clock enable"]
pub type USART4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART5` reader - USART5 clock enable"]
pub type USART5_R = crate::BitReader;
#[doc = "Field `USART5` writer - USART5 clock enable"]
pub type USART5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C1` reader - I2C1 clock enable"]
pub type I2C1_R = crate::BitReader;
#[doc = "Field `I2C1` writer - I2C1 clock enable"]
pub type I2C1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C2` reader - I2C2 clock enable"]
pub type I2C2_R = crate::BitReader;
#[doc = "Field `I2C2` writer - I2C2 clock enable"]
pub type I2C2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C3` reader - I2C3 clock enable"]
pub type I2C3_R = crate::BitReader;
#[doc = "Field `I2C3` writer - I2C3 clock enable"]
pub type I2C3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN1` reader - CAN1 clock enable"]
pub type CAN1_R = crate::BitReader;
#[doc = "Field `CAN1` writer - CAN1 clock enable"]
pub type CAN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN2` reader - CAN2 clock enable"]
pub type CAN2_R = crate::BitReader;
#[doc = "Field `CAN2` writer - CAN2 clock enable"]
pub type CAN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWC` reader - PWC clock enable"]
pub type PWC_R = crate::BitReader;
#[doc = "Field `PWC` writer - PWC clock enable"]
pub type PWC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAC` reader - DAC clock enable"]
pub type DAC_R = crate::BitReader;
#[doc = "Field `DAC` writer - DAC clock enable"]
pub type DAC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART7` reader - USART7 clock enable"]
pub type USART7_R = crate::BitReader;
#[doc = "Field `USART7` writer - USART7 clock enable"]
pub type USART7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART8` reader - USART8 clock enable"]
pub type USART8_R = crate::BitReader;
#[doc = "Field `USART8` writer - USART8 clock enable"]
pub type USART8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Timer2 clock enable"]
    #[inline(always)]
    pub fn tmr2(&self) -> TMR2_R {
        TMR2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer3 clock enable"]
    #[inline(always)]
    pub fn tmr3(&self) -> TMR3_R {
        TMR3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer4 clock enable"]
    #[inline(always)]
    pub fn tmr4(&self) -> TMR4_R {
        TMR4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer6 clock enable"]
    #[inline(always)]
    pub fn tmr6(&self) -> TMR6_R {
        TMR6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer7 clock enable"]
    #[inline(always)]
    pub fn tmr7(&self) -> TMR7_R {
        TMR7_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer12 clock enable"]
    #[inline(always)]
    pub fn tmr12(&self) -> TMR12_R {
        TMR12_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer13 clock enable"]
    #[inline(always)]
    pub fn tmr13(&self) -> TMR13_R {
        TMR13_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer14 clock enable"]
    #[inline(always)]
    pub fn tmr14(&self) -> TMR14_R {
        TMR14_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDT clock enable"]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline(always)]
    pub fn spi3(&self) -> SPI3_R {
        SPI3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 clock enable"]
    #[inline(always)]
    pub fn usart4(&self) -> USART4_R {
        USART4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - USART5 clock enable"]
    #[inline(always)]
    pub fn usart5(&self) -> USART5_R {
        USART5_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable"]
    #[inline(always)]
    pub fn i2c3(&self) -> I2C3_R {
        I2C3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 clock enable"]
    #[inline(always)]
    pub fn can1(&self) -> CAN1_R {
        CAN1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN2 clock enable"]
    #[inline(always)]
    pub fn can2(&self) -> CAN2_R {
        CAN2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - PWC clock enable"]
    #[inline(always)]
    pub fn pwc(&self) -> PWC_R {
        PWC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC clock enable"]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USART7 clock enable"]
    #[inline(always)]
    pub fn usart7(&self) -> USART7_R {
        USART7_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - USART8 clock enable"]
    #[inline(always)]
    pub fn usart8(&self) -> USART8_R {
        USART8_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2(&mut self) -> TMR2_W<APB1EN_SPEC, 0> {
        TMR2_W::new(self)
    }
    #[doc = "Bit 1 - Timer3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3(&mut self) -> TMR3_W<APB1EN_SPEC, 1> {
        TMR3_W::new(self)
    }
    #[doc = "Bit 2 - Timer4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr4(&mut self) -> TMR4_W<APB1EN_SPEC, 2> {
        TMR4_W::new(self)
    }
    #[doc = "Bit 4 - Timer6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr6(&mut self) -> TMR6_W<APB1EN_SPEC, 4> {
        TMR6_W::new(self)
    }
    #[doc = "Bit 5 - Timer7 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr7(&mut self) -> TMR7_W<APB1EN_SPEC, 5> {
        TMR7_W::new(self)
    }
    #[doc = "Bit 6 - Timer12 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr12(&mut self) -> TMR12_W<APB1EN_SPEC, 6> {
        TMR12_W::new(self)
    }
    #[doc = "Bit 7 - Timer13 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr13(&mut self) -> TMR13_W<APB1EN_SPEC, 7> {
        TMR13_W::new(self)
    }
    #[doc = "Bit 8 - Timer14 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr14(&mut self) -> TMR14_W<APB1EN_SPEC, 8> {
        TMR14_W::new(self)
    }
    #[doc = "Bit 11 - WWDT clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt(&mut self) -> WWDT_W<APB1EN_SPEC, 11> {
        WWDT_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2(&mut self) -> SPI2_W<APB1EN_SPEC, 14> {
        SPI2_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi3(&mut self) -> SPI3_W<APB1EN_SPEC, 15> {
        SPI3_W::new(self)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart2(&mut self) -> USART2_W<APB1EN_SPEC, 17> {
        USART2_W::new(self)
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart3(&mut self) -> USART3_W<APB1EN_SPEC, 18> {
        USART3_W::new(self)
    }
    #[doc = "Bit 19 - USART4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart4(&mut self) -> USART4_W<APB1EN_SPEC, 19> {
        USART4_W::new(self)
    }
    #[doc = "Bit 20 - USART5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart5(&mut self) -> USART5_W<APB1EN_SPEC, 20> {
        USART5_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<APB1EN_SPEC, 21> {
        I2C1_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2(&mut self) -> I2C2_W<APB1EN_SPEC, 22> {
        I2C2_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3(&mut self) -> I2C3_W<APB1EN_SPEC, 23> {
        I2C3_W::new(self)
    }
    #[doc = "Bit 25 - CAN1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn can1(&mut self) -> CAN1_W<APB1EN_SPEC, 25> {
        CAN1_W::new(self)
    }
    #[doc = "Bit 26 - CAN2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn can2(&mut self) -> CAN2_W<APB1EN_SPEC, 26> {
        CAN2_W::new(self)
    }
    #[doc = "Bit 28 - PWC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwc(&mut self) -> PWC_W<APB1EN_SPEC, 28> {
        PWC_W::new(self)
    }
    #[doc = "Bit 29 - DAC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac(&mut self) -> DAC_W<APB1EN_SPEC, 29> {
        DAC_W::new(self)
    }
    #[doc = "Bit 30 - USART7 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart7(&mut self) -> USART7_W<APB1EN_SPEC, 30> {
        USART7_W::new(self)
    }
    #[doc = "Bit 31 - USART8 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart8(&mut self) -> USART8_W<APB1EN_SPEC, 31> {
        USART8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "APB1 peripheral clock enable register (CRM_APB1EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1EN_SPEC;
impl crate::RegisterSpec for APB1EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1en::R`](R) reader structure"]
impl crate::Readable for APB1EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1en::W`](W) writer structure"]
impl crate::Writable for APB1EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1EN to value 0"]
impl crate::Resettable for APB1EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
