#[doc = "Register `BK3TMGATT` reader"]
pub type R = crate::R<BK3TMGATT_SPEC>;
#[doc = "Register `BK3TMGATT` writer"]
pub type W = crate::W<BK3TMGATT_SPEC>;
#[doc = "Field `SPST` reader - special memory setup time"]
pub type SPST_R = crate::FieldReader;
#[doc = "Field `SPST` writer - special memory setup time"]
pub type SPST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPWT` reader - special memory wait time"]
pub type SPWT_R = crate::FieldReader;
#[doc = "Field `SPWT` writer - special memory wait time"]
pub type SPWT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPHT` reader - special memory hold time"]
pub type SPHT_R = crate::FieldReader;
#[doc = "Field `SPHT` writer - special memory hold time"]
pub type SPHT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPDHIZT` reader - special memory databus High resistance time"]
pub type SPDHIZT_R = crate::FieldReader;
#[doc = "Field `SPDHIZT` writer - special memory databus High resistance time"]
pub type SPDHIZT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - special memory setup time"]
    #[inline(always)]
    pub fn spst(&self) -> SPST_R {
        SPST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - special memory wait time"]
    #[inline(always)]
    pub fn spwt(&self) -> SPWT_R {
        SPWT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - special memory hold time"]
    #[inline(always)]
    pub fn spht(&self) -> SPHT_R {
        SPHT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - special memory databus High resistance time"]
    #[inline(always)]
    pub fn spdhizt(&self) -> SPDHIZT_R {
        SPDHIZT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK3TMGATT")
            .field("spdhizt", &self.spdhizt())
            .field("spht", &self.spht())
            .field("spwt", &self.spwt())
            .field("spst", &self.spst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - special memory setup time"]
    #[inline(always)]
    #[must_use]
    pub fn spst(&mut self) -> SPST_W<BK3TMGATT_SPEC> {
        SPST_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - special memory wait time"]
    #[inline(always)]
    #[must_use]
    pub fn spwt(&mut self) -> SPWT_W<BK3TMGATT_SPEC> {
        SPWT_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - special memory hold time"]
    #[inline(always)]
    #[must_use]
    pub fn spht(&mut self) -> SPHT_W<BK3TMGATT_SPEC> {
        SPHT_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - special memory databus High resistance time"]
    #[inline(always)]
    #[must_use]
    pub fn spdhizt(&mut self) -> SPDHIZT_W<BK3TMGATT_SPEC> {
        SPDHIZT_W::new(self, 24)
    }
}
#[doc = "special memory space timing register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`bk3tmgatt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk3tmgatt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BK3TMGATT_SPEC;
impl crate::RegisterSpec for BK3TMGATT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk3tmgatt::R`](R) reader structure"]
impl crate::Readable for BK3TMGATT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bk3tmgatt::W`](W) writer structure"]
impl crate::Writable for BK3TMGATT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BK3TMGATT to value 0xfcfc_fcfc"]
impl crate::Resettable for BK3TMGATT_SPEC {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
