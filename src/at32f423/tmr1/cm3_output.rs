#[doc = "Register `CM3_OUTPUT` reader"]
pub type R = crate::R<CM3_OUTPUT_SPEC>;
#[doc = "Register `CM3_OUTPUT` writer"]
pub type W = crate::W<CM3_OUTPUT_SPEC>;
#[doc = "Field `C5OIEN` reader - Channel 5 output immediately enable"]
pub type C5OIEN_R = crate::BitReader;
#[doc = "Field `C5OIEN` writer - Channel 5 output immediately enable"]
pub type C5OIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C5OBEN` reader - Channel 5 output buffer enable"]
pub type C5OBEN_R = crate::BitReader;
#[doc = "Field `C5OBEN` writer - Channel 5 output buffer enable"]
pub type C5OBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C5OCTRL` reader - Channel 5 output control"]
pub type C5OCTRL_R = crate::FieldReader;
#[doc = "Field `C5OCTRL` writer - Channel 5 output control"]
pub type C5OCTRL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `C5OSEN` reader - Channel 5 output switch enable"]
pub type C5OSEN_R = crate::BitReader;
#[doc = "Field `C5OSEN` writer - Channel 5 output switch enable"]
pub type C5OSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - Channel 5 output immediately enable"]
    #[inline(always)]
    pub fn c5oien(&self) -> C5OIEN_R {
        C5OIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 5 output buffer enable"]
    #[inline(always)]
    pub fn c5oben(&self) -> C5OBEN_R {
        C5OBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 5 output control"]
    #[inline(always)]
    pub fn c5octrl(&self) -> C5OCTRL_R {
        C5OCTRL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Channel 5 output switch enable"]
    #[inline(always)]
    pub fn c5osen(&self) -> C5OSEN_R {
        C5OSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Channel 5 output immediately enable"]
    #[inline(always)]
    #[must_use]
    pub fn c5oien(&mut self) -> C5OIEN_W<CM3_OUTPUT_SPEC, 2> {
        C5OIEN_W::new(self)
    }
    #[doc = "Bit 3 - Channel 5 output buffer enable"]
    #[inline(always)]
    #[must_use]
    pub fn c5oben(&mut self) -> C5OBEN_W<CM3_OUTPUT_SPEC, 3> {
        C5OBEN_W::new(self)
    }
    #[doc = "Bits 4:6 - Channel 5 output control"]
    #[inline(always)]
    #[must_use]
    pub fn c5octrl(&mut self) -> C5OCTRL_W<CM3_OUTPUT_SPEC, 4> {
        C5OCTRL_W::new(self)
    }
    #[doc = "Bit 7 - Channel 5 output switch enable"]
    #[inline(always)]
    #[must_use]
    pub fn c5osen(&mut self) -> C5OSEN_W<CM3_OUTPUT_SPEC, 7> {
        C5OSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel output mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm3_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm3_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM3_OUTPUT_SPEC;
impl crate::RegisterSpec for CM3_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm3_output::R`](R) reader structure"]
impl crate::Readable for CM3_OUTPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm3_output::W`](W) writer structure"]
impl crate::Writable for CM3_OUTPUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM3_OUTPUT to value 0"]
impl crate::Resettable for CM3_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
