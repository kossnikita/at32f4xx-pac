#[doc = "Register `OTG_EXTCTRL` reader"]
pub type R = crate::R<OTG_EXTCTRL_SPEC>;
#[doc = "Register `OTG_EXTCTRL` writer"]
pub type W = crate::W<OTG_EXTCTRL_SPEC>;
#[doc = "Field `USBDIV_RST` reader - USB divider reset"]
pub type USBDIV_RST_R = crate::BitReader;
#[doc = "Field `USBDIV_RST` writer - USB divider reset"]
pub type USBDIV_RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EP3_RMPEN` reader - OTGFS end-point 3 remap enable"]
pub type EP3_RMPEN_R = crate::BitReader;
#[doc = "Field `EP3_RMPEN` writer - OTGFS end-point 3 remap enable"]
pub type EP3_RMPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 30 - USB divider reset"]
    #[inline(always)]
    pub fn usbdiv_rst(&self) -> USBDIV_RST_R {
        USBDIV_RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OTGFS end-point 3 remap enable"]
    #[inline(always)]
    pub fn ep3_rmpen(&self) -> EP3_RMPEN_R {
        EP3_RMPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_EXTCTRL")
            .field("usbdiv_rst", &format_args!("{}", self.usbdiv_rst().bit()))
            .field("ep3_rmpen", &format_args!("{}", self.ep3_rmpen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<OTG_EXTCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 30 - USB divider reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbdiv_rst(&mut self) -> USBDIV_RST_W<OTG_EXTCTRL_SPEC, 30> {
        USBDIV_RST_W::new(self)
    }
    #[doc = "Bit 31 - OTGFS end-point 3 remap enable"]
    #[inline(always)]
    #[must_use]
    pub fn ep3_rmpen(&mut self) -> EP3_RMPEN_W<OTG_EXTCTRL_SPEC, 31> {
        EP3_RMPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTGFS external ctrl register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_extctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_extctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_EXTCTRL_SPEC;
impl crate::RegisterSpec for OTG_EXTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_extctrl::R`](R) reader structure"]
impl crate::Readable for OTG_EXTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_extctrl::W`](W) writer structure"]
impl crate::Writable for OTG_EXTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_EXTCTRL to value 0"]
impl crate::Resettable for OTG_EXTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
