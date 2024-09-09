#[doc = "Register `REMAP6` reader"]
pub type R = crate::R<REMAP6_SPEC>;
#[doc = "Register `REMAP6` writer"]
pub type W = crate::W<REMAP6_SPEC>;
#[doc = "Field `CAN1_GMUX` reader - CAN1 muxing"]
pub type CAN1_GMUX_R = crate::FieldReader;
#[doc = "Field `CAN1_GMUX` writer - CAN1 muxing"]
pub type CAN1_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CAN2_GMUX` reader - CAN2 muxing"]
pub type CAN2_GMUX_R = crate::FieldReader;
#[doc = "Field `CAN2_GMUX` writer - CAN2 muxing"]
pub type CAN2_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDIO1_GMUX` reader - SDIO1 muxing"]
pub type SDIO1_GMUX_R = crate::FieldReader;
#[doc = "Field `SDIO1_GMUX` writer - SDIO1 muxing"]
pub type SDIO1_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USART1_GMUX` reader - USART1 muxing"]
pub type USART1_GMUX_R = crate::FieldReader;
#[doc = "Field `USART1_GMUX` writer - USART1 muxing"]
pub type USART1_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USART3_GMUX` reader - USART3 muxing"]
pub type USART3_GMUX_R = crate::FieldReader;
#[doc = "Field `USART3_GMUX` writer - USART3 muxing"]
pub type USART3_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UART4_GMUX` reader - UART4 muxing"]
pub type UART4_GMUX_R = crate::FieldReader;
#[doc = "Field `UART4_GMUX` writer - UART4 muxing"]
pub type UART4_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - CAN1 muxing"]
    #[inline(always)]
    pub fn can1_gmux(&self) -> CAN1_GMUX_R {
        CAN1_GMUX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CAN2 muxing"]
    #[inline(always)]
    pub fn can2_gmux(&self) -> CAN2_GMUX_R {
        CAN2_GMUX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SDIO1 muxing"]
    #[inline(always)]
    pub fn sdio1_gmux(&self) -> SDIO1_GMUX_R {
        SDIO1_GMUX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - USART1 muxing"]
    #[inline(always)]
    pub fn usart1_gmux(&self) -> USART1_GMUX_R {
        USART1_GMUX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - USART3 muxing"]
    #[inline(always)]
    pub fn usart3_gmux(&self) -> USART3_GMUX_R {
        USART3_GMUX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - UART4 muxing"]
    #[inline(always)]
    pub fn uart4_gmux(&self) -> UART4_GMUX_R {
        UART4_GMUX_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP6")
            .field("can1_gmux", &self.can1_gmux())
            .field("can2_gmux", &self.can2_gmux())
            .field("sdio1_gmux", &self.sdio1_gmux())
            .field("usart1_gmux", &self.usart1_gmux())
            .field("usart3_gmux", &self.usart3_gmux())
            .field("uart4_gmux", &self.uart4_gmux())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - CAN1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn can1_gmux(&mut self) -> CAN1_GMUX_W<REMAP6_SPEC> {
        CAN1_GMUX_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - CAN2 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn can2_gmux(&mut self) -> CAN2_GMUX_W<REMAP6_SPEC> {
        CAN2_GMUX_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - SDIO1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1_gmux(&mut self) -> SDIO1_GMUX_W<REMAP6_SPEC> {
        SDIO1_GMUX_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - USART1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn usart1_gmux(&mut self) -> USART1_GMUX_W<REMAP6_SPEC> {
        USART1_GMUX_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - USART3 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn usart3_gmux(&mut self) -> USART3_GMUX_W<REMAP6_SPEC> {
        USART3_GMUX_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - UART4 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn uart4_gmux(&mut self) -> UART4_GMUX_W<REMAP6_SPEC> {
        UART4_GMUX_W::new(self, 28)
    }
}
#[doc = "IO MUX remap register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`remap6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP6_SPEC;
impl crate::RegisterSpec for REMAP6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap6::R`](R) reader structure"]
impl crate::Readable for REMAP6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap6::W`](W) writer structure"]
impl crate::Writable for REMAP6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP6 to value 0"]
impl crate::Resettable for REMAP6_SPEC {
    const RESET_VALUE: u32 = 0;
}
