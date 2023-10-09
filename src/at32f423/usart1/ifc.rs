#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `RTODFC` writer - Receiver time out detection flag clear"]
pub type RTODFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDFC` writer - Character match flag clear"]
pub type CMDFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPWUFC` writer - Low power wake up flag clear"]
pub type LPWUFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl core::fmt::Debug for crate::generic::Reg<IFC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 11 - Receiver time out detection flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtodfc(&mut self) -> RTODFC_W<IFC_SPEC, 11> {
        RTODFC_W::new(self)
    }
    #[doc = "Bit 17 - Character match flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmdfc(&mut self) -> CMDFC_W<IFC_SPEC, 17> {
        CMDFC_W::new(self)
    }
    #[doc = "Bit 20 - Low power wake up flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn lpwufc(&mut self) -> LPWUFC_W<IFC_SPEC, 20> {
        LPWUFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
