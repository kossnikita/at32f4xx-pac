#[doc = "Register `MISC3` reader"]
pub type R = crate::R<MISC3_SPEC>;
#[doc = "Register `MISC3` writer"]
pub type W = crate::W<MISC3_SPEC>;
#[doc = "Field `AUTO_STEP_EN` reader - Auto step en"]
pub type AUTO_STEP_EN_R = crate::FieldReader;
#[doc = "Field `AUTO_STEP_EN` writer - Auto step en"]
pub type AUTO_STEP_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HICK_TO_USB` reader - HICK to usb clock"]
pub type HICK_TO_USB_R = crate::BitReader;
#[doc = "Field `HICK_TO_USB` writer - HICK to usb clock"]
pub type HICK_TO_USB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HICK_TO_SCLK` reader - HICK to system clock"]
pub type HICK_TO_SCLK_R = crate::BitReader;
#[doc = "Field `HICK_TO_SCLK` writer - HICK to system clock"]
pub type HICK_TO_SCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:5 - Auto step en"]
    #[inline(always)]
    pub fn auto_step_en(&self) -> AUTO_STEP_EN_R {
        AUTO_STEP_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - HICK to usb clock"]
    #[inline(always)]
    pub fn hick_to_usb(&self) -> HICK_TO_USB_R {
        HICK_TO_USB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HICK to system clock"]
    #[inline(always)]
    pub fn hick_to_sclk(&self) -> HICK_TO_SCLK_R {
        HICK_TO_SCLK_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC3")
            .field("auto_step_en", &self.auto_step_en())
            .field("hick_to_usb", &self.hick_to_usb())
            .field("hick_to_sclk", &self.hick_to_sclk())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:5 - Auto step en"]
    #[inline(always)]
    #[must_use]
    pub fn auto_step_en(&mut self) -> AUTO_STEP_EN_W<MISC3_SPEC> {
        AUTO_STEP_EN_W::new(self, 4)
    }
    #[doc = "Bit 8 - HICK to usb clock"]
    #[inline(always)]
    #[must_use]
    pub fn hick_to_usb(&mut self) -> HICK_TO_USB_W<MISC3_SPEC> {
        HICK_TO_USB_W::new(self, 8)
    }
    #[doc = "Bit 9 - HICK to system clock"]
    #[inline(always)]
    #[must_use]
    pub fn hick_to_sclk(&mut self) -> HICK_TO_SCLK_W<MISC3_SPEC> {
        HICK_TO_SCLK_W::new(self, 9)
    }
}
#[doc = "Miscellaneous register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`misc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC3_SPEC;
impl crate::RegisterSpec for MISC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc3::R`](R) reader structure"]
impl crate::Readable for MISC3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc3::W`](W) writer structure"]
impl crate::Writable for MISC3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISC3 to value 0"]
impl crate::Resettable for MISC3_SPEC {
    const RESET_VALUE: u32 = 0;
}
