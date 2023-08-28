#[doc = "Register `CM1_OUTPUT` reader"]
pub type R = crate::R<CM1_OUTPUT_SPEC>;
#[doc = "Register `CM1_OUTPUT` writer"]
pub type W = crate::W<CM1_OUTPUT_SPEC>;
#[doc = "Field `C1C` reader - Channel 1 configure"]
pub type C1C_R = crate::FieldReader;
#[doc = "Field `C1C` writer - Channel 1 configure"]
pub type C1C_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `C1OIEN` reader - Channel 1 output immediately enable"]
pub type C1OIEN_R = crate::BitReader;
#[doc = "Field `C1OIEN` writer - Channel 1 output immediately enable"]
pub type C1OIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C1OBEN` reader - Channel 1 output buffer enable"]
pub type C1OBEN_R = crate::BitReader;
#[doc = "Field `C1OBEN` writer - Channel 1 output buffer enable"]
pub type C1OBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C1OCTRL` reader - Channel 1 output control"]
pub type C1OCTRL_R = crate::FieldReader;
#[doc = "Field `C1OCTRL` writer - Channel 1 output control"]
pub type C1OCTRL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `C2C` reader - Channel 2 configure"]
pub type C2C_R = crate::FieldReader;
#[doc = "Field `C2C` writer - Channel 2 configure"]
pub type C2C_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `C2OIEN` reader - Channel 2 output immediately enable"]
pub type C2OIEN_R = crate::BitReader;
#[doc = "Field `C2OIEN` writer - Channel 2 output immediately enable"]
pub type C2OIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C2OBEN` reader - Channel 2 output buffer enable"]
pub type C2OBEN_R = crate::BitReader;
#[doc = "Field `C2OBEN` writer - Channel 2 output buffer enable"]
pub type C2OBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C2OCTRL` reader - Channel 2 output control"]
pub type C2OCTRL_R = crate::FieldReader;
#[doc = "Field `C2OCTRL` writer - Channel 2 output control"]
pub type C2OCTRL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:1 - Channel 1 configure"]
    #[inline(always)]
    pub fn c1c(&self) -> C1C_R {
        C1C_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Channel 1 output immediately enable"]
    #[inline(always)]
    pub fn c1oien(&self) -> C1OIEN_R {
        C1OIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 1 output buffer enable"]
    #[inline(always)]
    pub fn c1oben(&self) -> C1OBEN_R {
        C1OBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 1 output control"]
    #[inline(always)]
    pub fn c1octrl(&self) -> C1OCTRL_R {
        C1OCTRL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Channel 2 configure"]
    #[inline(always)]
    pub fn c2c(&self) -> C2C_R {
        C2C_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Channel 2 output immediately enable"]
    #[inline(always)]
    pub fn c2oien(&self) -> C2OIEN_R {
        C2OIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 output buffer enable"]
    #[inline(always)]
    pub fn c2oben(&self) -> C2OBEN_R {
        C2OBEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Channel 2 output control"]
    #[inline(always)]
    pub fn c2octrl(&self) -> C2OCTRL_R {
        C2OCTRL_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 1 configure"]
    #[inline(always)]
    #[must_use]
    pub fn c1c(&mut self) -> C1C_W<CM1_OUTPUT_SPEC, 0> {
        C1C_W::new(self)
    }
    #[doc = "Bit 2 - Channel 1 output immediately enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1oien(&mut self) -> C1OIEN_W<CM1_OUTPUT_SPEC, 2> {
        C1OIEN_W::new(self)
    }
    #[doc = "Bit 3 - Channel 1 output buffer enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1oben(&mut self) -> C1OBEN_W<CM1_OUTPUT_SPEC, 3> {
        C1OBEN_W::new(self)
    }
    #[doc = "Bits 4:6 - Channel 1 output control"]
    #[inline(always)]
    #[must_use]
    pub fn c1octrl(&mut self) -> C1OCTRL_W<CM1_OUTPUT_SPEC, 4> {
        C1OCTRL_W::new(self)
    }
    #[doc = "Bits 8:9 - Channel 2 configure"]
    #[inline(always)]
    #[must_use]
    pub fn c2c(&mut self) -> C2C_W<CM1_OUTPUT_SPEC, 8> {
        C2C_W::new(self)
    }
    #[doc = "Bit 10 - Channel 2 output immediately enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2oien(&mut self) -> C2OIEN_W<CM1_OUTPUT_SPEC, 10> {
        C2OIEN_W::new(self)
    }
    #[doc = "Bit 11 - Channel 2 output buffer enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2oben(&mut self) -> C2OBEN_W<CM1_OUTPUT_SPEC, 11> {
        C2OBEN_W::new(self)
    }
    #[doc = "Bits 12:14 - Channel 2 output control"]
    #[inline(always)]
    #[must_use]
    pub fn c2octrl(&mut self) -> C2OCTRL_W<CM1_OUTPUT_SPEC, 12> {
        C2OCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel output mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm1_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm1_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM1_OUTPUT_SPEC;
impl crate::RegisterSpec for CM1_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm1_output::R`](R) reader structure"]
impl crate::Readable for CM1_OUTPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm1_output::W`](W) writer structure"]
impl crate::Writable for CM1_OUTPUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM1_OUTPUT to value 0"]
impl crate::Resettable for CM1_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
