#[doc = "Register `REMAP6` reader"]
pub type R = crate::R<REMAP6_SPEC>;
#[doc = "Register `REMAP6` writer"]
pub type W = crate::W<REMAP6_SPEC>;
#[doc = "Field `CAN1_GMUX` reader - CAN1 muxing"]
pub type CAN1_GMUX_R = crate::FieldReader;
#[doc = "Field `CAN1_GMUX` writer - CAN1 muxing"]
pub type CAN1_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USART1_GMUX` reader - USART1 muxing"]
pub type USART1_GMUX_R = crate::FieldReader;
#[doc = "Field `USART1_GMUX` writer - USART1 muxing"]
pub type USART1_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USART3_GMUX` reader - USART3 muxing"]
pub type USART3_GMUX_R = crate::FieldReader;
#[doc = "Field `USART3_GMUX` writer - USART3 muxing"]
pub type USART3_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - CAN1 muxing"]
    #[inline(always)]
    pub fn can1_gmux(&self) -> CAN1_GMUX_R {
        CAN1_GMUX_R::new((self.bits & 0x0f) as u8)
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP6")
            .field(
                "usart3_gmux",
                &format_args!("{}", self.usart3_gmux().bits()),
            )
            .field(
                "usart1_gmux",
                &format_args!("{}", self.usart1_gmux().bits()),
            )
            .field("can1_gmux", &format_args!("{}", self.can1_gmux().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<REMAP6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - CAN1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn can1_gmux(&mut self) -> CAN1_GMUX_W<REMAP6_SPEC> {
        CAN1_GMUX_W::new(self, 0)
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
#[doc = "IO MUX remap register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP6_SPEC;
impl crate::RegisterSpec for REMAP6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap6::R`](R) reader structure"]
impl crate::Readable for REMAP6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap6::W`](W) writer structure"]
impl crate::Writable for REMAP6_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REMAP6 to value 0"]
impl crate::Resettable for REMAP6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
