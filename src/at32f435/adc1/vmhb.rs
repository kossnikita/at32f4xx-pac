#[doc = "Register `VMHB` reader"]
pub type R = crate::R<VMHB_SPEC>;
#[doc = "Register `VMHB` writer"]
pub type W = crate::W<VMHB_SPEC>;
#[doc = "Field `VMHB` reader - Voltage monitoring high boundary"]
pub type VMHB_R = crate::FieldReader<u16>;
#[doc = "Field `VMHB` writer - Voltage monitoring high boundary"]
pub type VMHB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Voltage monitoring high boundary"]
    #[inline(always)]
    pub fn vmhb(&self) -> VMHB_R {
        VMHB_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Voltage monitoring high boundary"]
    #[inline(always)]
    #[must_use]
    pub fn vmhb(&mut self) -> VMHB_W<VMHB_SPEC, 0> {
        VMHB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Voltage monitoring high boundary register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmhb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmhb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VMHB_SPEC;
impl crate::RegisterSpec for VMHB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmhb::R`](R) reader structure"]
impl crate::Readable for VMHB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vmhb::W`](W) writer structure"]
impl crate::Writable for VMHB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VMHB to value 0x0fff"]
impl crate::Resettable for VMHB_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
