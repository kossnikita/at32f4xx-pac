#[doc = "Register `MISC2` reader"]
pub type R = crate::R<MISC2_SPEC>;
#[doc = "Register `MISC2` writer"]
pub type W = crate::W<MISC2_SPEC>;
#[doc = "Field `HICK_TO_USB` reader - HICK_TO_USB"]
pub type HICK_TO_USB_R = crate::BitReader;
#[doc = "Field `HICK_TO_USB` writer - HICK_TO_USB"]
pub type HICK_TO_USB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HICK_TO_SCLK` reader - HICK to system clock"]
pub type HICK_TO_SCLK_R = crate::BitReader;
#[doc = "Field `HICK_TO_SCLK` writer - HICK to system clock"]
pub type HICK_TO_SCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - HICK_TO_USB"]
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
        f.debug_struct("MISC2")
            .field("hick_to_usb", &self.hick_to_usb())
            .field("hick_to_sclk", &self.hick_to_sclk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - HICK_TO_USB"]
    #[inline(always)]
    #[must_use]
    pub fn hick_to_usb(&mut self) -> HICK_TO_USB_W<MISC2_SPEC> {
        HICK_TO_USB_W::new(self, 8)
    }
    #[doc = "Bit 9 - HICK to system clock"]
    #[inline(always)]
    #[must_use]
    pub fn hick_to_sclk(&mut self) -> HICK_TO_SCLK_W<MISC2_SPEC> {
        HICK_TO_SCLK_W::new(self, 9)
    }
}
#[doc = "Miscellaneous register2\n\nYou can [`read`](crate::Reg::read) this register and get [`misc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
