#[doc = "Register `OSQ6` reader"]
pub type R = crate::R<OSQ6_SPEC>;
#[doc = "Register `OSQ6` writer"]
pub type W = crate::W<OSQ6_SPEC>;
#[doc = "Field `OSN(29-32)` reader - Number of %sth conversion in ordinary sequence"]
pub type OSN_R = crate::FieldReader;
#[doc = "Field `OSN(29-32)` writer - Number of %sth conversion in ordinary sequence"]
pub type OSN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Number of (29-32)th conversion in ordinary sequence"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OSN29` field.</div>"]
    #[inline(always)]
    pub fn osn(&self, n: u8) -> OSN_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        OSN_R::new(((self.bits >> (n * 5)) & 0x1f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Number of (29-32)th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn_iter(&self) -> impl Iterator<Item = OSN_R> + '_ {
        (0..4).map(move |n| OSN_R::new(((self.bits >> (n * 5)) & 0x1f) as u8))
    }
    #[doc = "Bits 0:4 - Number of 29th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn29(&self) -> OSN_R {
        OSN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 30th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn30(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Number of 31th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn31(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 32th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn32(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSQ6")
            .field("osn29", &self.osn29())
            .field("osn30", &self.osn30())
            .field("osn31", &self.osn31())
            .field("osn32", &self.osn32())
            .finish()
    }
}
impl W {
    #[doc = "Number of (29-32)th conversion in ordinary sequence"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OSN29` field.</div>"]
    #[inline(always)]
    pub fn osn(&mut self, n: u8) -> OSN_W<'_, OSQ6_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        OSN_W::new(self, n * 5)
    }
    #[doc = "Bits 0:4 - Number of 29th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn29(&mut self) -> OSN_W<'_, OSQ6_SPEC> {
        OSN_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Number of 30th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn30(&mut self) -> OSN_W<'_, OSQ6_SPEC> {
        OSN_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Number of 31th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn31(&mut self) -> OSN_W<'_, OSQ6_SPEC> {
        OSN_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Number of 32th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn32(&mut self) -> OSN_W<'_, OSQ6_SPEC> {
        OSN_W::new(self, 15)
    }
}
#[doc = "Ordinary sequence register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`osq6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osq6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSQ6_SPEC;
impl crate::RegisterSpec for OSQ6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq6::R`](R) reader structure"]
impl crate::Readable for OSQ6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osq6::W`](W) writer structure"]
impl crate::Writable for OSQ6_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSQ6 to value 0"]
impl crate::Resettable for OSQ6_SPEC {}
