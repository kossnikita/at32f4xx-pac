#[doc = "Register `OTGHS` reader"]
pub type R = crate::R<OTGHS_SPEC>;
#[doc = "Register `OTGHS` writer"]
pub type W = crate::W<OTGHS_SPEC>;
#[doc = "Field `USBHS_PHY12_SEL` reader - USBHS phy12 select value"]
pub type USBHS_PHY12_SEL_R = crate::FieldReader;
#[doc = "Field `USBHS_PHY12_SEL` writer - USBHS phy12 select value"]
pub type USBHS_PHY12_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 3:4 - USBHS phy12 select value"]
    #[inline(always)]
    pub fn usbhs_phy12_sel(&self) -> USBHS_PHY12_SEL_R {
        USBHS_PHY12_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTGHS")
            .field(
                "usbhs_phy12_sel",
                &format_args!("{}", self.usbhs_phy12_sel().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<OTGHS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 3:4 - USBHS phy12 select value"]
    #[inline(always)]
    #[must_use]
    pub fn usbhs_phy12_sel(&mut self) -> USBHS_PHY12_SEL_W<OTGHS_SPEC, 3> {
        USBHS_PHY12_SEL_W::new(self)
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
#[doc = "OTGHS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otghs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otghs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTGHS_SPEC;
impl crate::RegisterSpec for OTGHS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otghs::R`](R) reader structure"]
impl crate::Readable for OTGHS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otghs::W`](W) writer structure"]
impl crate::Writable for OTGHS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTGHS to value 0"]
impl crate::Resettable for OTGHS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
