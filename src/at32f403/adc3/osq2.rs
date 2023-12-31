#[doc = "Register `OSQ2` reader"]
pub type R = crate::R<OSQ2_SPEC>;
#[doc = "Register `OSQ2` writer"]
pub type W = crate::W<OSQ2_SPEC>;
#[doc = "Field `OSN[7-12]` reader - Number of %sth conversion in ordinary sequence"]
pub type OSN_R = crate::FieldReader;
#[doc = "Field `OSN[7-12]` writer - Number of %sth conversion in ordinary sequence"]
pub type OSN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Number of [7-12]th conversion in ordinary sequence\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn osn(&self, n: u8) -> OSN_R {
        assert!(n < 6);
        OSN_R::new(((self.bits >> (n * 5)) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - Number of 7th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn7(&self) -> OSN_R {
        OSN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 8th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn8(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Number of 9th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn9(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 10th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn10(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Number of 11th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn11(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Number of 12th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn12(&self) -> OSN_R {
        OSN_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSQ2")
            .field("osn7", &format_args!("{}", self.osn7().bits()))
            .field("osn8", &format_args!("{}", self.osn8().bits()))
            .field("osn9", &format_args!("{}", self.osn9().bits()))
            .field("osn10", &format_args!("{}", self.osn10().bits()))
            .field("osn11", &format_args!("{}", self.osn11().bits()))
            .field("osn12", &format_args!("{}", self.osn12().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<OSQ2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Number of [7-12]th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn(&mut self, n: u8) -> OSN_W<OSQ2_SPEC> {
        assert!(n < 6);
        OSN_W::new(self, n * 5)
    }
    #[doc = "Bits 0:4 - Number of 7th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn7(&mut self) -> OSN_W<OSQ2_SPEC> {
        OSN_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Number of 8th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn8(&mut self) -> OSN_W<OSQ2_SPEC> {
        OSN_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Number of 9th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn9(&mut self) -> OSN_W<OSQ2_SPEC> {
        OSN_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Number of 10th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn10(&mut self) -> OSN_W<OSQ2_SPEC> {
        OSN_W::new(self, 15)
    }
    #[doc = "Bits 20:24 - Number of 11th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn11(&mut self) -> OSN_W<OSQ2_SPEC> {
        OSN_W::new(self, 20)
    }
    #[doc = "Bits 25:29 - Number of 12th conversion in ordinary sequence"]
    #[inline(always)]
    #[must_use]
    pub fn osn12(&mut self) -> OSN_W<OSQ2_SPEC> {
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
#[doc = "Ordinary sequence register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osq2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osq2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSQ2_SPEC;
impl crate::RegisterSpec for OSQ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq2::R`](R) reader structure"]
impl crate::Readable for OSQ2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osq2::W`](W) writer structure"]
impl crate::Writable for OSQ2_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSQ2 to value 0"]
impl crate::Resettable for OSQ2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
