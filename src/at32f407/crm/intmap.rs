#[doc = "Register `INTMAP` reader"]
pub type R = crate::R<INTMAP_SPEC>;
#[doc = "Register `INTMAP` writer"]
pub type W = crate::W<INTMAP_SPEC>;
#[doc = "Field `USB_INT_MAP` reader - USBDEV interrupt remap"]
pub type USB_INT_MAP_R = crate::BitReader;
#[doc = "Field `USB_INT_MAP` writer - USBDEV interrupt remap"]
pub type USB_INT_MAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - USBDEV interrupt remap"]
    #[inline(always)]
    pub fn usb_int_map(&self) -> USB_INT_MAP_R {
        USB_INT_MAP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USBDEV interrupt remap"]
    #[inline(always)]
    #[must_use]
    pub fn usb_int_map(&mut self) -> USB_INT_MAP_W<INTMAP_SPEC, 0> {
        USB_INT_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intmap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intmap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTMAP_SPEC;
impl crate::RegisterSpec for INTMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmap::R`](R) reader structure"]
impl crate::Readable for INTMAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intmap::W`](W) writer structure"]
impl crate::Writable for INTMAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTMAP to value 0"]
impl crate::Resettable for INTMAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}