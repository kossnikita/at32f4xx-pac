#[doc = "Register `OSQ4` reader"]
pub type R = crate::R<OSQ4_SPEC>;
#[doc = "Register `OSQ4` writer"]
pub type W = crate::W<OSQ4_SPEC>;
#[doc = "Field `OSN(17-22)` reader - Number of %sth conversion in ordinary sequence"]
pub type OSN_R = crate::FieldReader;
#[doc = "Field `OSN(17-22)` writer - Number of %sth conversion in ordinary sequence"]
pub type OSN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Number of (17-22)th conversion in ordinary sequence"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OSN17` field.</div>"]
    #[inline(always)]
    pub fn osn(&self, n: u8) -> OSN_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        OSN_R::new(((self.bits >> (n * 5)) & 0x1f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Number of (17-22)th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn_iter(&self) -> impl Iterator<Item = OSN_R> + '_ {
        (0..6).map(move |n| OSN_R::new(((self.bits >> (n * 5)) & 0x1f) as u8))
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
            .field("osn17", &self.osn17())
            .field("osn18", &self.osn18())
            .field("osn19", &self.osn19())
            .field("osn20", &self.osn20())
            .field("osn21", &self.osn21())
            .field("osn22", &self.osn22())
            .finish()
    }
}
impl W {
    #[doc = "Number of (17-22)th conversion in ordinary sequence"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OSN17` field.</div>"]
    #[inline(always)]
    pub fn osn(&mut self, n: u8) -> OSN_W<'_, OSQ4_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        OSN_W::new(self, n * 5)
    }
    #[doc = "Bits 0:4 - Number of 17th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn17(&mut self) -> OSN_W<'_, OSQ4_SPEC> {
        OSN_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Number of 18th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn18(&mut self) -> OSN_W<'_, OSQ4_SPEC> {
        OSN_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Number of 19th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn19(&mut self) -> OSN_W<'_, OSQ4_SPEC> {
        OSN_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Number of 20th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn20(&mut self) -> OSN_W<'_, OSQ4_SPEC> {
        OSN_W::new(self, 15)
    }
    #[doc = "Bits 20:24 - Number of 21th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn21(&mut self) -> OSN_W<'_, OSQ4_SPEC> {
        OSN_W::new(self, 20)
    }
    #[doc = "Bits 25:29 - Number of 22th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn22(&mut self) -> OSN_W<'_, OSQ4_SPEC> {
        OSN_W::new(self, 25)
    }
}
#[doc = "Ordinary sequence register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`osq4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osq4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSQ4_SPEC;
impl crate::RegisterSpec for OSQ4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq4::R`](R) reader structure"]
impl crate::Readable for OSQ4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osq4::W`](W) writer structure"]
impl crate::Writable for OSQ4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSQ4 to value 0"]
impl crate::Resettable for OSQ4_SPEC {}
