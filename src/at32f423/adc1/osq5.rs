#[doc = "Register `OSQ5` reader"]
pub type R = crate::R<OSQ5_SPEC>;
#[doc = "Register `OSQ5` writer"]
pub type W = crate::W<OSQ5_SPEC>;
#[doc = "Field `OSN(23-28)` reader - Number of %sth conversion in ordinary sequence"]
pub type OSN_R = crate::FieldReader;
#[doc = "Field `OSN(23-28)` writer - Number of %sth conversion in ordinary sequence"]
pub type OSN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Number of (23-28)th conversion in ordinary sequence"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OSN23` field.</div>"]
    #[inline(always)]
    pub fn osn(&self, n: u8) -> OSN_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        OSN_R::new(((self.bits >> (n * 5)) & 0x1f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Number of (23-28)th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn_iter(&self) -> impl Iterator<Item = OSN_R> + '_ {
        (0..6).map(move |n| OSN_R::new(((self.bits >> (n * 5)) & 0x1f) as u8))
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
            .field("osn23", &self.osn23())
            .field("osn24", &self.osn24())
            .field("osn25", &self.osn25())
            .field("osn26", &self.osn26())
            .field("osn27", &self.osn27())
            .field("osn28", &self.osn28())
            .finish()
    }
}
impl W {
    #[doc = "Number of (23-28)th conversion in ordinary sequence"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OSN23` field.</div>"]
    #[inline(always)]
    pub fn osn(&mut self, n: u8) -> OSN_W<'_, OSQ5_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        OSN_W::new(self, n * 5)
    }
    #[doc = "Bits 0:4 - Number of 23th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn23(&mut self) -> OSN_W<'_, OSQ5_SPEC> {
        OSN_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Number of 24th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn24(&mut self) -> OSN_W<'_, OSQ5_SPEC> {
        OSN_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Number of 25th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn25(&mut self) -> OSN_W<'_, OSQ5_SPEC> {
        OSN_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Number of 26th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn26(&mut self) -> OSN_W<'_, OSQ5_SPEC> {
        OSN_W::new(self, 15)
    }
    #[doc = "Bits 20:24 - Number of 27th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn27(&mut self) -> OSN_W<'_, OSQ5_SPEC> {
        OSN_W::new(self, 20)
    }
    #[doc = "Bits 25:29 - Number of 28th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn28(&mut self) -> OSN_W<'_, OSQ5_SPEC> {
        OSN_W::new(self, 25)
    }
}
#[doc = "Ordinary sequence register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`osq5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osq5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSQ5_SPEC;
impl crate::RegisterSpec for OSQ5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq5::R`](R) reader structure"]
impl crate::Readable for OSQ5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osq5::W`](W) writer structure"]
impl crate::Writable for OSQ5_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSQ5 to value 0"]
impl crate::Resettable for OSQ5_SPEC {}
