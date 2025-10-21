#[doc = "Register `OSQ1` reader"]
pub type R = crate::R<OSQ1_SPEC>;
#[doc = "Register `OSQ1` writer"]
pub type W = crate::W<OSQ1_SPEC>;
#[doc = "Field `OSN(13-16)` reader - Number of %sth conversion in ordinary sequence"]
pub type OSN_R = crate::FieldReader;
#[doc = "Field `OSN(13-16)` writer - Number of %sth conversion in ordinary sequence"]
pub type OSN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OCLEN` reader - Ordinary conversion sequence length"]
pub type OCLEN_R = crate::FieldReader;
#[doc = "Field `OCLEN` writer - Ordinary conversion sequence length"]
pub type OCLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Number of (13-16)th conversion in ordinary sequence"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OSN13` field.</div>"]
    #[inline(always)]
    pub fn osn(&self, n: u8) -> OSN_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        OSN_R::new(((self.bits >> (n * 5)) & 0x1f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Number of (13-16)th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn_iter(&self) -> impl Iterator<Item = OSN_R> + '_ {
        (0..4).map(move |n| OSN_R::new(((self.bits >> (n * 5)) & 0x1f) as u8))
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
            .field("oclen", &self.oclen())
            .field("osn13", &self.osn13())
            .field("osn14", &self.osn14())
            .field("osn15", &self.osn15())
            .field("osn16", &self.osn16())
            .finish()
    }
}
impl W {
    #[doc = "Number of (13-16)th conversion in ordinary sequence"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OSN13` field.</div>"]
    #[inline(always)]
    pub fn osn(&mut self, n: u8) -> OSN_W<'_, OSQ1_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        OSN_W::new(self, n * 5)
    }
    #[doc = "Bits 0:4 - Number of 13th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn13(&mut self) -> OSN_W<'_, OSQ1_SPEC> {
        OSN_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Number of 14th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn14(&mut self) -> OSN_W<'_, OSQ1_SPEC> {
        OSN_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Number of 15th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn15(&mut self) -> OSN_W<'_, OSQ1_SPEC> {
        OSN_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Number of 16th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn16(&mut self) -> OSN_W<'_, OSQ1_SPEC> {
        OSN_W::new(self, 15)
    }
    #[doc = "Bits 20:24 - Ordinary conversion sequence length"]
    #[inline(always)]
    pub fn oclen(&mut self) -> OCLEN_W<'_, OSQ1_SPEC> {
        OCLEN_W::new(self, 20)
    }
}
#[doc = "Ordinary sequence register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`osq1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osq1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSQ1_SPEC;
impl crate::RegisterSpec for OSQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq1::R`](R) reader structure"]
impl crate::Readable for OSQ1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osq1::W`](W) writer structure"]
impl crate::Writable for OSQ1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSQ1 to value 0"]
impl crate::Resettable for OSQ1_SPEC {}
