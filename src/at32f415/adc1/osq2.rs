#[doc = "Register `OSQ2` reader"]
pub type R = crate::R<OSQ2_SPEC>;
#[doc = "Register `OSQ2` writer"]
pub type W = crate::W<OSQ2_SPEC>;
#[doc = "Field `OSN7` reader - Number of 13th conversion in ordinary sequence"]
pub type OSN7_R = crate::FieldReader;
#[doc = "Field `OSN7` writer - Number of 13th conversion in ordinary sequence"]
pub type OSN7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OSN8` reader - Number of 7th conversion in ordinary sequence"]
pub type OSN8_R = crate::FieldReader;
#[doc = "Field `OSN8` writer - Number of 7th conversion in ordinary sequence"]
pub type OSN8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OSN9` reader - Number of 8th conversion in ordinary sequence"]
pub type OSN9_R = crate::FieldReader;
#[doc = "Field `OSN9` writer - Number of 8th conversion in ordinary sequence"]
pub type OSN9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OSN10` reader - Number of 10th conversion in ordinary sequence"]
pub type OSN10_R = crate::FieldReader;
#[doc = "Field `OSN10` writer - Number of 10th conversion in ordinary sequence"]
pub type OSN10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OSN11` reader - Number of 11th conversion in ordinary sequence"]
pub type OSN11_R = crate::FieldReader;
#[doc = "Field `OSN11` writer - Number of 11th conversion in ordinary sequence"]
pub type OSN11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OSN12` reader - Number of 12th conversion in ordinary sequence"]
pub type OSN12_R = crate::FieldReader;
#[doc = "Field `OSN12` writer - Number of 12th conversion in ordinary sequence"]
pub type OSN12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Number of 13th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn7(&self) -> OSN7_R {
        OSN7_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 7th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn8(&self) -> OSN8_R {
        OSN8_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Number of 8th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn9(&self) -> OSN9_R {
        OSN9_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 10th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn10(&self) -> OSN10_R {
        OSN10_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Number of 11th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn11(&self) -> OSN11_R {
        OSN11_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Number of 12th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn12(&self) -> OSN12_R {
        OSN12_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of 13th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn7(&mut self) -> OSN7_W<OSQ2_SPEC, 0> {
        OSN7_W::new(self)
    }
    #[doc = "Bits 5:9 - Number of 7th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn8(&mut self) -> OSN8_W<OSQ2_SPEC, 5> {
        OSN8_W::new(self)
    }
    #[doc = "Bits 10:14 - Number of 8th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn9(&mut self) -> OSN9_W<OSQ2_SPEC, 10> {
        OSN9_W::new(self)
    }
    #[doc = "Bits 15:19 - Number of 10th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn10(&mut self) -> OSN10_W<OSQ2_SPEC, 15> {
        OSN10_W::new(self)
    }
    #[doc = "Bits 20:24 - Number of 11th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn11(&mut self) -> OSN11_W<OSQ2_SPEC, 20> {
        OSN11_W::new(self)
    }
    #[doc = "Bits 25:29 - Number of 12th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn12(&mut self) -> OSN12_W<OSQ2_SPEC, 25> {
        OSN12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ordinary sequence register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osq2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osq2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSQ2_SPEC;
impl crate::RegisterSpec for OSQ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq2::R`](R) reader structure"]
impl crate::Readable for OSQ2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osq2::W`](W) writer structure"]
impl crate::Writable for OSQ2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSQ2 to value 0"]
impl crate::Resettable for OSQ2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
