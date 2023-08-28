#[doc = "Register `VMLB` reader"]
pub type R = crate::R<VMLB_SPEC>;
#[doc = "Register `VMLB` writer"]
pub type W = crate::W<VMLB_SPEC>;
#[doc = "Field `VMLB` reader - Voltage monitoring low boundary"]
pub type VMLB_R = crate::FieldReader<u16>;
#[doc = "Field `VMLB` writer - Voltage monitoring low boundary"]
pub type VMLB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Voltage monitoring low boundary"]
    #[inline(always)]
    pub fn vmlb(&self) -> VMLB_R {
        VMLB_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Voltage monitoring low boundary"]
    #[inline(always)]
    #[must_use]
    pub fn vmlb(&mut self) -> VMLB_W<VMLB_SPEC, 0> {
        VMLB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Voltage monitoring low boundary register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VMLB_SPEC;
impl crate::RegisterSpec for VMLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmlb::R`](R) reader structure"]
impl crate::Readable for VMLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vmlb::W`](W) writer structure"]
impl crate::Writable for VMLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VMLB to value 0"]
impl crate::Resettable for VMLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
