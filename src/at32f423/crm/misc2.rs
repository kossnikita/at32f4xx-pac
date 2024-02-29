#[doc = "Register `MISC2` reader"]
pub type R = crate::R<MISC2_SPEC>;
#[doc = "Register `MISC2` writer"]
pub type W = crate::W<MISC2_SPEC>;
#[doc = "Field `AUTO_STEP_EN` reader - AUTO_STEP_EN"]
pub type AUTO_STEP_EN_R = crate::FieldReader;
#[doc = "Field `AUTO_STEP_EN` writer - AUTO_STEP_EN"]
pub type AUTO_STEP_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USBDIV` reader - USB division"]
pub type USBDIV_R = crate::FieldReader;
#[doc = "Field `USBDIV` writer - USB division"]
pub type USBDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HICK_TO_SCLK_DIV` reader - HICK as system clock frequency division"]
pub type HICK_TO_SCLK_DIV_R = crate::FieldReader;
#[doc = "Field `HICK_TO_SCLK_DIV` writer - HICK as system clock frequency division"]
pub type HICK_TO_SCLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HEXT_TO_SCLK_DIV` reader - HEXT as system clock frequency division"]
pub type HEXT_TO_SCLK_DIV_R = crate::FieldReader;
#[doc = "Field `HEXT_TO_SCLK_DIV` writer - HEXT as system clock frequency division"]
pub type HEXT_TO_SCLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 4:5 - AUTO_STEP_EN"]
    #[inline(always)]
    pub fn auto_step_en(&self) -> AUTO_STEP_EN_R {
        AUTO_STEP_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 12:15 - USB division"]
    #[inline(always)]
    pub fn usbdiv(&self) -> USBDIV_R {
        USBDIV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - HICK as system clock frequency division"]
    #[inline(always)]
    pub fn hick_to_sclk_div(&self) -> HICK_TO_SCLK_DIV_R {
        HICK_TO_SCLK_DIV_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - HEXT as system clock frequency division"]
    #[inline(always)]
    pub fn hext_to_sclk_div(&self) -> HEXT_TO_SCLK_DIV_R {
        HEXT_TO_SCLK_DIV_R::new(((self.bits >> 19) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC2")
            .field(
                "auto_step_en",
                &format_args!("{}", self.auto_step_en().bits()),
            )
            .field("usbdiv", &format_args!("{}", self.usbdiv().bits()))
            .field(
                "hick_to_sclk_div",
                &format_args!("{}", self.hick_to_sclk_div().bits()),
            )
            .field(
                "hext_to_sclk_div",
                &format_args!("{}", self.hext_to_sclk_div().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MISC2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 4:5 - AUTO_STEP_EN"]
    #[inline(always)]
    #[must_use]
    pub fn auto_step_en(&mut self) -> AUTO_STEP_EN_W<MISC2_SPEC> {
        AUTO_STEP_EN_W::new(self, 4)
    }
    #[doc = "Bits 12:15 - USB division"]
    #[inline(always)]
    #[must_use]
    pub fn usbdiv(&mut self) -> USBDIV_W<MISC2_SPEC> {
        USBDIV_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - HICK as system clock frequency division"]
    #[inline(always)]
    #[must_use]
    pub fn hick_to_sclk_div(&mut self) -> HICK_TO_SCLK_DIV_W<MISC2_SPEC> {
        HICK_TO_SCLK_DIV_W::new(self, 16)
    }
    #[doc = "Bits 19:21 - HEXT as system clock frequency division"]
    #[inline(always)]
    #[must_use]
    pub fn hext_to_sclk_div(&mut self) -> HEXT_TO_SCLK_DIV_W<MISC2_SPEC> {
        HEXT_TO_SCLK_DIV_W::new(self, 19)
    }
}
#[doc = "Miscellaneous register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC2_SPEC;
impl crate::RegisterSpec for MISC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc2::R`](R) reader structure"]
impl crate::Readable for MISC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc2::W`](W) writer structure"]
impl crate::Writable for MISC2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISC2 to value 0x0d"]
impl crate::Resettable for MISC2_SPEC {
    const RESET_VALUE: u32 = 0x0d;
}
