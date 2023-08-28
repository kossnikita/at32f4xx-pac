#[doc = "Register `OSQ3` reader"]
pub type R = crate::R<OSQ3_SPEC>;
#[doc = "Register `OSQ3` writer"]
pub type W = crate::W<OSQ3_SPEC>;
#[doc = "Field `OSN1` reader - Number of 1st conversion in ordinary sequence"]
pub type OSN1_R = crate::FieldReader;
#[doc = "Field `OSN1` writer - Number of 1st conversion in ordinary sequence"]
pub type OSN1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OSN2` reader - Number of 2nd conversion in ordinary sequence"]
pub type OSN2_R = crate::FieldReader;
#[doc = "Field `OSN2` writer - Number of 2nd conversion in ordinary sequence"]
pub type OSN2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OSN3` reader - number of 3rd conversion in ordinary sequence"]
pub type OSN3_R = crate::FieldReader;
#[doc = "Field `OSN3` writer - number of 3rd conversion in ordinary sequence"]
pub type OSN3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OSN4` reader - Number of 4th conversion in ordinary sequence"]
pub type OSN4_R = crate::FieldReader;
#[doc = "Field `OSN4` writer - Number of 4th conversion in ordinary sequence"]
pub type OSN4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OSN5` reader - Number of 5th conversion in ordinary sequence"]
pub type OSN5_R = crate::FieldReader;
#[doc = "Field `OSN5` writer - Number of 5th conversion in ordinary sequence"]
pub type OSN5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OSN6` reader - Number of 6th conversion in ordinary sequence"]
pub type OSN6_R = crate::FieldReader;
#[doc = "Field `OSN6` writer - Number of 6th conversion in ordinary sequence"]
pub type OSN6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Number of 1st conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn1(&self) -> OSN1_R {
        OSN1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 2nd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn2(&self) -> OSN2_R {
        OSN2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - number of 3rd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn3(&self) -> OSN3_R {
        OSN3_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 4th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn4(&self) -> OSN4_R {
        OSN4_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Number of 5th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn5(&self) -> OSN5_R {
        OSN5_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Number of 6th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn6(&self) -> OSN6_R {
        OSN6_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of 1st conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn1(&mut self) -> OSN1_W<OSQ3_SPEC, 0> {
        OSN1_W::new(self)
    }
    #[doc = "Bits 5:9 - Number of 2nd conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn2(&mut self) -> OSN2_W<OSQ3_SPEC, 5> {
        OSN2_W::new(self)
    }
    #[doc = "Bits 10:14 - number of 3rd conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn3(&mut self) -> OSN3_W<OSQ3_SPEC, 10> {
        OSN3_W::new(self)
    }
    #[doc = "Bits 15:19 - Number of 4th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn4(&mut self) -> OSN4_W<OSQ3_SPEC, 15> {
        OSN4_W::new(self)
    }
    #[doc = "Bits 20:24 - Number of 5th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn5(&mut self) -> OSN5_W<OSQ3_SPEC, 20> {
        OSN5_W::new(self)
    }
    #[doc = "Bits 25:29 - Number of 6th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn6(&mut self) -> OSN6_W<OSQ3_SPEC, 25> {
        OSN6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ordinary sequence register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osq3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osq3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSQ3_SPEC;
impl crate::RegisterSpec for OSQ3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq3::R`](R) reader structure"]
impl crate::Readable for OSQ3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osq3::W`](W) writer structure"]
impl crate::Writable for OSQ3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSQ3 to value 0"]
impl crate::Resettable for OSQ3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
