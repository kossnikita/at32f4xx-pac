#[doc = "Register `OTG_EXTCTRL` reader"]
pub type R = crate::R<OTG_EXTCTRL_SPEC>;
#[doc = "Register `OTG_EXTCTRL` writer"]
pub type W = crate::W<OTG_EXTCTRL_SPEC>;
#[doc = "Field `USBDIV_RST` reader - USB divider reset"]
pub type USBDIV_RST_R = crate::BitReader;
#[doc = "Field `USBDIV_RST` writer - USB divider reset"]
pub type USBDIV_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3_RMPEN` reader - OTGFS end-point 3 remap enable"]
pub type EP3_RMPEN_R = crate::BitReader;
#[doc = "Field `EP3_RMPEN` writer - OTGFS end-point 3 remap enable"]
pub type EP3_RMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("usbdiv_rst", &self.usbdiv_rst())
            .field("ep3_rmpen", &self.ep3_rmpen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - USB divider reset"]
    #[inline(always)]
    pub fn usbdiv_rst(&mut self) -> USBDIV_RST_W<'_, OTG_EXTCTRL_SPEC> {
        USBDIV_RST_W::new(self, 30)
    }
    #[doc = "Bit 31 - OTGFS end-point 3 remap enable"]
    #[inline(always)]
    pub fn ep3_rmpen(&mut self) -> EP3_RMPEN_W<'_, OTG_EXTCTRL_SPEC> {
        EP3_RMPEN_W::new(self, 31)
    }
}
#[doc = "OTGFS external ctrl register1\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_extctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_extctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_EXTCTRL_SPEC;
impl crate::RegisterSpec for OTG_EXTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_extctrl::R`](R) reader structure"]
impl crate::Readable for OTG_EXTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_extctrl::W`](W) writer structure"]
impl crate::Writable for OTG_EXTCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_EXTCTRL to value 0"]
impl crate::Resettable for OTG_EXTCTRL_SPEC {}
