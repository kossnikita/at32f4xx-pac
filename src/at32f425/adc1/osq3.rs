#[doc = "Register `OSQ3` reader"]
pub type R = crate::R<OSQ3_SPEC>;
#[doc = "Register `OSQ3` writer"]
pub type W = crate::W<OSQ3_SPEC>;
#[doc = "Field `OSN(1-6)` reader - Number of %sth conversion in ordinary sequence"]
pub type OSN_R = crate::FieldReader;
#[doc = "Field `OSN(1-6)` writer - Number of %sth conversion in ordinary sequence"]
pub type OSN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Number of (1-6)th conversion in ordinary sequence"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OSN1` field.</div>"]
    #[inline(always)]
    pub fn osn(&self, n: u8) -> OSN_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        OSN_R::new(((self.bits >> (n * 5)) & 0x1f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Number of (1-6)th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn_iter(&self) -> impl Iterator<Item = OSN_R> + '_ {
        (0..6).map(move |n| OSN_R::new(((self.bits >> (n * 5)) & 0x1f) as u8))
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
            .field("osn1", &self.osn1())
            .field("osn2", &self.osn2())
            .field("osn3", &self.osn3())
            .field("osn4", &self.osn4())
            .field("osn5", &self.osn5())
            .field("osn6", &self.osn6())
            .finish()
    }
}
impl W {
    #[doc = "Number of (1-6)th conversion in ordinary sequence"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OSN1` field.</div>"]
    #[inline(always)]
    pub fn osn(&mut self, n: u8) -> OSN_W<'_, OSQ3_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        OSN_W::new(self, n * 5)
    }
    #[doc = "Bits 0:4 - Number of 1th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn1(&mut self) -> OSN_W<'_, OSQ3_SPEC> {
        OSN_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Number of 2th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn2(&mut self) -> OSN_W<'_, OSQ3_SPEC> {
        OSN_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Number of 3th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn3(&mut self) -> OSN_W<'_, OSQ3_SPEC> {
        OSN_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Number of 4th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn4(&mut self) -> OSN_W<'_, OSQ3_SPEC> {
        OSN_W::new(self, 15)
    }
    #[doc = "Bits 20:24 - Number of 5th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn5(&mut self) -> OSN_W<'_, OSQ3_SPEC> {
        OSN_W::new(self, 20)
    }
    #[doc = "Bits 25:29 - Number of 6th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn6(&mut self) -> OSN_W<'_, OSQ3_SPEC> {
        OSN_W::new(self, 25)
    }
}
#[doc = "Ordinary sequence register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`osq3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osq3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSQ3_SPEC;
impl crate::RegisterSpec for OSQ3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq3::R`](R) reader structure"]
impl crate::Readable for OSQ3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osq3::W`](W) writer structure"]
impl crate::Writable for OSQ3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSQ3 to value 0"]
impl crate::Resettable for OSQ3_SPEC {}
