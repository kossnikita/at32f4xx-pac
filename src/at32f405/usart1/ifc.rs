#[doc = "Register `IFC` reader"]
pub type R = crate::R<IFC_SPEC>;
#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `RTODFC` reader - Receiver time out detection flag clear"]
pub type RTODFC_R = crate::BitReader;
#[doc = "Field `RTODFC` writer - Receiver time out detection flag clear"]
pub type RTODFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDFC` reader - Character match detection flag clear"]
pub type CMDFC_R = crate::BitReader;
#[doc = "Field `CMDFC` writer - Character match detection flag clear"]
pub type CMDFC_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IFC")
            .field("cmdfc", &self.cmdfc())
            .field("rtodfc", &self.rtodfc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 11 - Receiver time out detection flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtodfc(&mut self) -> RTODFC_W<IFC_SPEC> {
        RTODFC_W::new(self, 11)
    }
    #[doc = "Bit 17 - Character match detection flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmdfc(&mut self) -> CMDFC_W<IFC_SPEC> {
        CMDFC_W::new(self, 17)
    }
}
#[doc = "Interruption flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifc::R`](R) reader structure"]
impl crate::Readable for IFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: u32 = 0;
}
