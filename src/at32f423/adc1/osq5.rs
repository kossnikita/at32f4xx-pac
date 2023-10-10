#[doc = "Register `OSQ5` reader"]
pub type R = crate::R<OSQ5_SPEC>;
#[doc = "Register `OSQ5` writer"]
pub type W = crate::W<OSQ5_SPEC>;
#[doc = "Field `OSN[23-28]` reader - Number of %sth conversion in ordinary sequence"]
pub type OSN_R = crate::FieldReader;
#[doc = "Field `OSN[23-28]` writer - Number of %sth conversion in ordinary sequence"]
pub type OSN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Number of [23-28]th conversion in ordinary sequence"]
    #[inline(always)]
    pub unsafe fn osn(&self, n: u8) -> OSN_R {
        OSN_R::new(((self.bits >> ((n - 23) * 5)) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - Number of 23th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn23(&self) -> OSN_R {
        OSN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 24th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn24(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Number of 25th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn25(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 26th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn26(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Number of 27th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn27(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Number of 28th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn28(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSQ5")
            .field("osn23", &format_args!("{}", self.osn23().bits()))
            .field("osn24", &format_args!("{}", self.osn24().bits()))
            .field("osn25", &format_args!("{}", self.osn25().bits()))
            .field("osn26", &format_args!("{}", self.osn26().bits()))
            .field("osn27", &format_args!("{}", self.osn27().bits()))
            .field("osn28", &format_args!("{}", self.osn28().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<OSQ5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Number of [23-28]th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn osn<const O: u8>(&mut self) -> OSN_W<OSQ5_SPEC, O> {
        OSN_W::new(self)
    }
    #[doc = "Bits 0:4 - Number of 23th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn23(&mut self) -> OSN_W<OSQ5_SPEC, 0> {
        OSN_W::new(self)
    }
    #[doc = "Bits 5:9 - Number of 24th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn24(&mut self) -> OSN_W<OSQ5_SPEC, 5> {
        OSN_W::new(self)
    }
    #[doc = "Bits 10:14 - Number of 25th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn25(&mut self) -> OSN_W<OSQ5_SPEC, 10> {
        OSN_W::new(self)
    }
    #[doc = "Bits 15:19 - Number of 26th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn26(&mut self) -> OSN_W<OSQ5_SPEC, 15> {
        OSN_W::new(self)
    }
    #[doc = "Bits 20:24 - Number of 27th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn27(&mut self) -> OSN_W<OSQ5_SPEC, 20> {
        OSN_W::new(self)
    }
    #[doc = "Bits 25:29 - Number of 28th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn28(&mut self) -> OSN_W<OSQ5_SPEC, 25> {
        OSN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
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
