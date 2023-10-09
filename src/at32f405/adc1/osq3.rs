#[doc = "Register `OSQ3` reader"]
pub type R = crate::R<OSQ3_SPEC>;
#[doc = "Register `OSQ3` writer"]
pub type W = crate::W<OSQ3_SPEC>;
#[doc = "Field `OSN[1-6]` reader - Number of %sth conversion in ordinary sequence"]
pub type OSN_R = crate::FieldReader;
#[doc = "Field `OSN[1-6]` writer - Number of %sth conversion in ordinary sequence"]
pub type OSN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Number of [1-6]th conversion in ordinary sequence"]
    #[inline(always)]
    pub unsafe fn osn(&self, n: u8) -> OSN_R {
        OSN_R::new(((self.bits >> ((n - 1) * 5)) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - Number of 1th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn1(&self) -> OSN_R {
        OSN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 2th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn2(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Number of 3th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn3(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 4th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn4(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Number of 5th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn5(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Number of 6th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn6(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSQ3")
            .field("osn1", &format_args!("{}", self.osn1().bits()))
            .field("osn2", &format_args!("{}", self.osn2().bits()))
            .field("osn3", &format_args!("{}", self.osn3().bits()))
            .field("osn4", &format_args!("{}", self.osn4().bits()))
            .field("osn5", &format_args!("{}", self.osn5().bits()))
            .field("osn6", &format_args!("{}", self.osn6().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<OSQ3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Number of [1-6]th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn osn<const O: u8>(&mut self) -> OSN_W<OSQ3_SPEC, O> {
        OSN_W::new(self)
    }
    #[doc = "Bits 0:4 - Number of 1th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn1(&mut self) -> OSN_W<OSQ3_SPEC, 0> {
        OSN_W::new(self)
    }
    #[doc = "Bits 5:9 - Number of 2th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn2(&mut self) -> OSN_W<OSQ3_SPEC, 5> {
        OSN_W::new(self)
    }
    #[doc = "Bits 10:14 - Number of 3th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn3(&mut self) -> OSN_W<OSQ3_SPEC, 10> {
        OSN_W::new(self)
    }
    #[doc = "Bits 15:19 - Number of 4th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn4(&mut self) -> OSN_W<OSQ3_SPEC, 15> {
        OSN_W::new(self)
    }
    #[doc = "Bits 20:24 - Number of 5th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn5(&mut self) -> OSN_W<OSQ3_SPEC, 20> {
        OSN_W::new(self)
    }
    #[doc = "Bits 25:29 - Number of 6th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn6(&mut self) -> OSN_W<OSQ3_SPEC, 25> {
        OSN_W::new(self)
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
