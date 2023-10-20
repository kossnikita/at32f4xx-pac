#[doc = "Register `OSQ4` reader"]
pub type R = crate::R<OSQ4_SPEC>;
#[doc = "Register `OSQ4` writer"]
pub type W = crate::W<OSQ4_SPEC>;
#[doc = "Field `OSN[17-22]` reader - Number of %sth conversion in ordinary sequence"]
pub type OSN_R = crate::FieldReader;
#[doc = "Field `OSN[17-22]` writer - Number of %sth conversion in ordinary sequence"]
pub type OSN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Number of [17-22]th conversion in ordinary sequence\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn osn(&self, n: u8) -> OSN_R {
        assert!(n < 6);
        OSN_R::new(((self.bits >> (n * 5)) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - Number of 17th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn17(&self) -> OSN_R {
        OSN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 18th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn18(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Number of 19th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn19(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 20th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn20(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Number of 21th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn21(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Number of 22th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn22(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSQ4")
            .field("osn17", &format_args!("{}", self.osn17().bits()))
            .field("osn18", &format_args!("{}", self.osn18().bits()))
            .field("osn19", &format_args!("{}", self.osn19().bits()))
            .field("osn20", &format_args!("{}", self.osn20().bits()))
            .field("osn21", &format_args!("{}", self.osn21().bits()))
            .field("osn22", &format_args!("{}", self.osn22().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<OSQ4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Number of [17-22]th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn(&mut self, n: u8) -> OSN_W<OSQ4_SPEC> {
        assert!(n < 6);
        OSN_W::new(self, n * 5)
    }
    #[doc = "Bits 0:4 - Number of 17th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn17(&mut self) -> OSN_W<OSQ4_SPEC> {
        OSN_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Number of 18th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn18(&mut self) -> OSN_W<OSQ4_SPEC> {
        OSN_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Number of 19th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn19(&mut self) -> OSN_W<OSQ4_SPEC> {
        OSN_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Number of 20th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn20(&mut self) -> OSN_W<OSQ4_SPEC> {
        OSN_W::new(self, 15)
    }
    #[doc = "Bits 20:24 - Number of 21th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn21(&mut self) -> OSN_W<OSQ4_SPEC> {
        OSN_W::new(self, 20)
    }
    #[doc = "Bits 25:29 - Number of 22th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn22(&mut self) -> OSN_W<OSQ4_SPEC> {
        OSN_W::new(self, 25)
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
#[doc = "Ordinary sequence register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osq4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osq4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSQ4_SPEC;
impl crate::RegisterSpec for OSQ4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq4::R`](R) reader structure"]
impl crate::Readable for OSQ4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osq4::W`](W) writer structure"]
impl crate::Writable for OSQ4_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSQ4 to value 0"]
impl crate::Resettable for OSQ4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
