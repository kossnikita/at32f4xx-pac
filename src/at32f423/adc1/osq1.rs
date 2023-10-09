#[doc = "Register `OSQ1` reader"]
pub type R = crate::R<OSQ1_SPEC>;
#[doc = "Register `OSQ1` writer"]
pub type W = crate::W<OSQ1_SPEC>;
#[doc = "Field `OSN[13-16]` reader - Number of %sth conversion in ordinary sequence"]
pub type OSN_R = crate::FieldReader;
#[doc = "Field `OSN[13-16]` writer - Number of %sth conversion in ordinary sequence"]
pub type OSN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OCLEN` reader - Ordinary conversion sequence length"]
pub type OCLEN_R = crate::FieldReader;
#[doc = "Field `OCLEN` writer - Ordinary conversion sequence length"]
pub type OCLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Number of [13-16]th conversion in ordinary sequence"]
    #[inline(always)]
    pub unsafe fn osn(&self, n: u8) -> OSN_R {
        OSN_R::new(((self.bits >> ((n - 13) * 5)) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - Number of 13th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn13(&self) -> OSN_R {
        OSN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 14th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn14(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Number of 15th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn15(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 16th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn16(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Ordinary conversion sequence length"]
    #[inline(always)]
    pub fn oclen(&self) -> OCLEN_R {
        OCLEN_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSQ1")
            .field("oclen", &format_args!("{}", self.oclen().bits()))
            .field("osn13", &format_args!("{}", self.osn13().bits()))
            .field("osn14", &format_args!("{}", self.osn14().bits()))
            .field("osn15", &format_args!("{}", self.osn15().bits()))
            .field("osn16", &format_args!("{}", self.osn16().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<OSQ1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Number of [13-16]th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn osn<const O: u8>(&mut self) -> OSN_W<OSQ1_SPEC, O> {
        OSN_W::new(self)
    }
    #[doc = "Bits 0:4 - Number of 13th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn13(&mut self) -> OSN_W<OSQ1_SPEC, 0> {
        OSN_W::new(self)
    }
    #[doc = "Bits 5:9 - Number of 14th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn14(&mut self) -> OSN_W<OSQ1_SPEC, 5> {
        OSN_W::new(self)
    }
    #[doc = "Bits 10:14 - Number of 15th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn15(&mut self) -> OSN_W<OSQ1_SPEC, 10> {
        OSN_W::new(self)
    }
    #[doc = "Bits 15:19 - Number of 16th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn16(&mut self) -> OSN_W<OSQ1_SPEC, 15> {
        OSN_W::new(self)
    }
    #[doc = "Bits 20:24 - Ordinary conversion sequence length"]
    #[inline(always)]
    #[must_use]
    pub fn oclen(&mut self) -> OCLEN_W<OSQ1_SPEC, 20> {
        OCLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ordinary sequence register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osq1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osq1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSQ1_SPEC;
impl crate::RegisterSpec for OSQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq1::R`](R) reader structure"]
impl crate::Readable for OSQ1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osq1::W`](W) writer structure"]
impl crate::Writable for OSQ1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSQ1 to value 0"]
impl crate::Resettable for OSQ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
