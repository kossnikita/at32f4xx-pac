#[doc = "Register `APB1EN` reader"]
pub type R = crate::R<APB1EN_SPEC>;
#[doc = "Register `APB1EN` writer"]
pub type W = crate::W<APB1EN_SPEC>;
#[doc = "Field `TMR2EN` reader - Timer2 clock enable"]
pub type TMR2EN_R = crate::BitReader;
#[doc = "Field `TMR2EN` writer - Timer2 clock enable"]
pub type TMR2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR3EN` reader - Timer3 clock enable"]
pub type TMR3EN_R = crate::BitReader;
#[doc = "Field `TMR3EN` writer - Timer3 clock enable"]
pub type TMR3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR4EN` reader - Timer4 clock enable"]
pub type TMR4EN_R = crate::BitReader;
#[doc = "Field `TMR4EN` writer - Timer4 clock enable"]
pub type TMR4EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR5EN` reader - Timer5 clock enable"]
pub type TMR5EN_R = crate::BitReader;
#[doc = "Field `TMR5EN` writer - Timer5 clock enable"]
pub type TMR5EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMPEN` reader - Comparator clock enable"]
pub type CMPEN_R = crate::BitReader;
#[doc = "Field `CMPEN` writer - Comparator clock enable"]
pub type CMPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WWDTEN` reader - Window watchdog timer clock enable"]
pub type WWDTEN_R = crate::BitReader;
#[doc = "Field `WWDTEN` writer - Window watchdog timer clock enable"]
pub type WWDTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable"]
pub type SPI2EN_R = crate::BitReader;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable"]
pub type SPI2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART2EN` reader - USART2 clock enable"]
pub type USART2EN_R = crate::BitReader;
#[doc = "Field `USART2EN` writer - USART2 clock enable"]
pub type USART2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART3EN` reader - USART3 clock enable"]
pub type USART3EN_R = crate::BitReader;
#[doc = "Field `USART3EN` writer - USART3 clock enable"]
pub type USART3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART5EN` reader - UART5 clock enable"]
pub type UART5EN_R = crate::BitReader;
#[doc = "Field `UART5EN` writer - UART5 clock enable"]
pub type UART5EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable"]
pub type I2C1EN_R = crate::BitReader;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable"]
pub type I2C1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN1EN` reader - CAN1 clock enable"]
pub type CAN1EN_R = crate::BitReader;
#[doc = "Field `CAN1EN` writer - CAN1 clock enable"]
pub type CAN1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWCEN` reader - Power clock enable"]
pub type PWCEN_R = crate::BitReader;
#[doc = "Field `PWCEN` writer - Power clock enable"]
pub type PWCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Timer2 clock enable"]
    #[inline(always)]
    pub fn tmr2en(&self) -> TMR2EN_R {
        TMR2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer3 clock enable"]
    #[inline(always)]
    pub fn tmr3en(&self) -> TMR3EN_R {
        TMR3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer4 clock enable"]
    #[inline(always)]
    pub fn tmr4en(&self) -> TMR4EN_R {
        TMR4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer5 clock enable"]
    #[inline(always)]
    pub fn tmr5en(&self) -> TMR5EN_R {
        TMR5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator clock enable"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    pub fn wwdten(&self) -> WWDTEN_R {
        WWDTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 clock enable"]
    #[inline(always)]
    pub fn uart5en(&self) -> UART5EN_R {
        UART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 clock enable"]
    #[inline(always)]
    pub fn can1en(&self) -> CAN1EN_R {
        CAN1EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Power clock enable"]
    #[inline(always)]
    pub fn pwcen(&self) -> PWCEN_R {
        PWCEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2en(&mut self) -> TMR2EN_W<APB1EN_SPEC, 0> {
        TMR2EN_W::new(self)
    }
    #[doc = "Bit 1 - Timer3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3en(&mut self) -> TMR3EN_W<APB1EN_SPEC, 1> {
        TMR3EN_W::new(self)
    }
    #[doc = "Bit 2 - Timer4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr4en(&mut self) -> TMR4EN_W<APB1EN_SPEC, 2> {
        TMR4EN_W::new(self)
    }
    #[doc = "Bit 3 - Timer5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr5en(&mut self) -> TMR5EN_W<APB1EN_SPEC, 3> {
        TMR5EN_W::new(self)
    }
    #[doc = "Bit 9 - Comparator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen(&mut self) -> CMPEN_W<APB1EN_SPEC, 9> {
        CMPEN_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdten(&mut self) -> WWDTEN_W<APB1EN_SPEC, 11> {
        WWDTEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<APB1EN_SPEC, 14> {
        SPI2EN_W::new(self)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<APB1EN_SPEC, 17> {
        USART2EN_W::new(self)
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> USART3EN_W<APB1EN_SPEC, 18> {
        USART3EN_W::new(self)
    }
    #[doc = "Bit 20 - UART5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart5en(&mut self) -> UART5EN_W<APB1EN_SPEC, 20> {
        UART5EN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<APB1EN_SPEC, 21> {
        I2C1EN_W::new(self)
    }
    #[doc = "Bit 25 - CAN1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn can1en(&mut self) -> CAN1EN_W<APB1EN_SPEC, 25> {
        CAN1EN_W::new(self)
    }
    #[doc = "Bit 28 - Power clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwcen(&mut self) -> PWCEN_W<APB1EN_SPEC, 28> {
        PWCEN_W::new(self)
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
