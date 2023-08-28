#[doc = "Register `OSQ6` reader"]
pub type R = crate::R<OSQ6_SPEC>;
#[doc = "Register `OSQ6` writer"]
pub type W = crate::W<OSQ6_SPEC>;
#[doc = "Field `OSN29` reader - Number of 29st conversion in ordinary sequence"]
pub type OSN29_R = crate::FieldReader;
#[doc = "Field `OSN29` writer - Number of 29st conversion in ordinary sequence"]
pub type OSN29_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OSN30` reader - Number of 30nd conversion in ordinary sequence"]
pub type OSN30_R = crate::FieldReader;
#[doc = "Field `OSN30` writer - Number of 30nd conversion in ordinary sequence"]
pub type OSN30_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OSN31` reader - number of 31rd conversion in ordinary sequence"]
pub type OSN31_R = crate::FieldReader;
#[doc = "Field `OSN31` writer - number of 31rd conversion in ordinary sequence"]
pub type OSN31_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OSN32` reader - Number of 32th conversion in ordinary sequence"]
pub type OSN32_R = crate::FieldReader;
#[doc = "Field `OSN32` writer - Number of 32th conversion in ordinary sequence"]
pub type OSN32_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Number of 29st conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn29(&self) -> OSN29_R {
        OSN29_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 30nd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn30(&self) -> OSN30_R {
        OSN30_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - number of 31rd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn31(&self) -> OSN31_R {
        OSN31_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 32th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn32(&self) -> OSN32_R {
        OSN32_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of 29st conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn29(&mut self) -> OSN29_W<OSQ6_SPEC, 0> {
        OSN29_W::new(self)
    }
    #[doc = "Bits 5:9 - Number of 30nd conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn30(&mut self) -> OSN30_W<OSQ6_SPEC, 5> {
        OSN30_W::new(self)
    }
    #[doc = "Bits 10:14 - number of 31rd conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn31(&mut self) -> OSN31_W<OSQ6_SPEC, 10> {
        OSN31_W::new(self)
    }
    #[doc = "Bits 15:19 - Number of 32th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn32(&mut self) -> OSN32_W<OSQ6_SPEC, 15> {
        OSN32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ordinary sequence register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osq6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osq6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSQ6_SPEC;
impl crate::RegisterSpec for OSQ6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq6::R`](R) reader structure"]
impl crate::Readable for OSQ6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osq6::W`](W) writer structure"]
impl crate::Writable for OSQ6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSQ6 to value 0"]
impl crate::Resettable for OSQ6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
