#[doc = "Register `IFC` reader"]
pub type R = crate::R<IFC_SPEC>;
#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `RTODFC` reader - Receiver time out detection flag clear"]
pub type RTODFC_R = crate::BitReader;
#[doc = "Field `RTODFC` writer - Receiver time out detection flag clear"]
pub type RTODFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDFC` reader - Character match detection flag clear"]
pub type CMDFC_R = crate::BitReader;
#[doc = "Field `CMDFC` writer - Character match detection flag clear"]
pub type CMDFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 11 - Receiver time out detection flag clear"]
    #[inline(always)]
    pub fn rtodfc(&self) -> RTODFC_R {
        RTODFC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - Character match detection flag clear"]
    #[inline(always)]
    pub fn cmdfc(&self) -> CMDFC_R {
        CMDFC_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Receiver time out detection flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtodfc(&mut self) -> RTODFC_W<IFC_SPEC, 11> {
        RTODFC_W::new(self)
    }
    #[doc = "Bit 17 - Character match detection flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmdfc(&mut self) -> CMDFC_W<IFC_SPEC, 17> {
        CMDFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interruption flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifc::R`](R) reader structure"]
impl crate::Readable for IFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}