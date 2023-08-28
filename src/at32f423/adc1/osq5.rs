#[doc = "Register `OSQ5` reader"]
pub type R = crate::R<OSQ5_SPEC>;
#[doc = "Register `OSQ5` writer"]
pub type W = crate::W<OSQ5_SPEC>;
#[doc = "Field `OSN23` reader - Number of 23st conversion in ordinary sequence"]
pub type OSN23_R = crate::FieldReader;
#[doc = "Field `OSN23` writer - Number of 23st conversion in ordinary sequence"]
pub type OSN23_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OSN24` reader - Number of 24nd conversion in ordinary sequence"]
pub type OSN24_R = crate::FieldReader;
#[doc = "Field `OSN24` writer - Number of 24nd conversion in ordinary sequence"]
pub type OSN24_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OSN25` reader - number of 25rd conversion in ordinary sequence"]
pub type OSN25_R = crate::FieldReader;
#[doc = "Field `OSN25` writer - number of 25rd conversion in ordinary sequence"]
pub type OSN25_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OSN26` reader - Number of 26th conversion in ordinary sequence"]
pub type OSN26_R = crate::FieldReader;
#[doc = "Field `OSN26` writer - Number of 26th conversion in ordinary sequence"]
pub type OSN26_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OSN27` reader - Number of 27th conversion in ordinary sequence"]
pub type OSN27_R = crate::FieldReader;
#[doc = "Field `OSN27` writer - Number of 27th conversion in ordinary sequence"]
pub type OSN27_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OSN28` reader - Number of 28th conversion in ordinary sequence"]
pub type OSN28_R = crate::FieldReader;
#[doc = "Field `OSN28` writer - Number of 28th conversion in ordinary sequence"]
pub type OSN28_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Number of 23st conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn23(&self) -> OSN23_R {
        OSN23_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 24nd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn24(&self) -> OSN24_R {
        OSN24_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - number of 25rd conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn25(&self) -> OSN25_R {
        OSN25_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 26th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn26(&self) -> OSN26_R {
        OSN26_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Number of 27th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn27(&self) -> OSN27_R {
        OSN27_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Number of 28th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn28(&self) -> OSN28_R {
        OSN28_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of 23st conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn23(&mut self) -> OSN23_W<OSQ5_SPEC, 0> {
        OSN23_W::new(self)
    }
    #[doc = "Bits 5:9 - Number of 24nd conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn24(&mut self) -> OSN24_W<OSQ5_SPEC, 5> {
        OSN24_W::new(self)
    }
    #[doc = "Bits 10:14 - number of 25rd conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn25(&mut self) -> OSN25_W<OSQ5_SPEC, 10> {
        OSN25_W::new(self)
    }
    #[doc = "Bits 15:19 - Number of 26th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn26(&mut self) -> OSN26_W<OSQ5_SPEC, 15> {
        OSN26_W::new(self)
    }
    #[doc = "Bits 20:24 - Number of 27th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn27(&mut self) -> OSN27_W<OSQ5_SPEC, 20> {
        OSN27_W::new(self)
    }
    #[doc = "Bits 25:29 - Number of 28th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn28(&mut self) -> OSN28_W<OSQ5_SPEC, 25> {
        OSN28_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ordinary sequence register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osq5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osq5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSQ5_SPEC;
impl crate::RegisterSpec for OSQ5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq5::R`](R) reader structure"]
impl crate::Readable for OSQ5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osq5::W`](W) writer structure"]
impl crate::Writable for OSQ5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSQ5 to value 0"]
impl crate::Resettable for OSQ5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
