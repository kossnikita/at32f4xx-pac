#[doc = "Register `BK4TMGAT` reader"]
pub type R = crate::R<BK4TMGAT_SPEC>;
#[doc = "Register `BK4TMGAT` writer"]
pub type W = crate::W<BK4TMGAT_SPEC>;
#[doc = "Field `ATST` reader - special memory setup time"]
pub type ATST_R = crate::FieldReader;
#[doc = "Field `ATST` writer - special memory setup time"]
pub type ATST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ATWT` reader - special memory wait time"]
pub type ATWT_R = crate::FieldReader;
#[doc = "Field `ATWT` writer - special memory wait time"]
pub type ATWT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ATHT` reader - special memory hold time"]
pub type ATHT_R = crate::FieldReader;
#[doc = "Field `ATHT` writer - special memory hold time"]
pub type ATHT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ATDHIZT` reader - special memory databus High resistance time"]
pub type ATDHIZT_R = crate::FieldReader;
#[doc = "Field `ATDHIZT` writer - special memory databus High resistance time"]
pub type ATDHIZT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - special memory setup time"]
    #[inline(always)]
    pub fn atst(&self) -> ATST_R {
        ATST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - special memory wait time"]
    #[inline(always)]
    pub fn atwt(&self) -> ATWT_R {
        ATWT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - special memory hold time"]
    #[inline(always)]
    pub fn atht(&self) -> ATHT_R {
        ATHT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - special memory databus High resistance time"]
    #[inline(always)]
    pub fn atdhizt(&self) -> ATDHIZT_R {
        ATDHIZT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - special memory setup time"]
    #[inline(always)]
    #[must_use]
    pub fn atst(&mut self) -> ATST_W<BK4TMGAT_SPEC, 0> {
        ATST_W::new(self)
    }
    #[doc = "Bits 8:15 - special memory wait time"]
    #[inline(always)]
    #[must_use]
    pub fn atwt(&mut self) -> ATWT_W<BK4TMGAT_SPEC, 8> {
        ATWT_W::new(self)
    }
    #[doc = "Bits 16:23 - special memory hold time"]
    #[inline(always)]
    #[must_use]
    pub fn atht(&mut self) -> ATHT_W<BK4TMGAT_SPEC, 16> {
        ATHT_W::new(self)
    }
    #[doc = "Bits 24:31 - special memory databus High resistance time"]
    #[inline(always)]
    #[must_use]
    pub fn atdhizt(&mut self) -> ATDHIZT_W<BK4TMGAT_SPEC, 24> {
        ATDHIZT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "special memory space timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4tmgat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4tmgat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BK4TMGAT_SPEC;
impl crate::RegisterSpec for BK4TMGAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk4tmgat::R`](R) reader structure"]
impl crate::Readable for BK4TMGAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bk4tmgat::W`](W) writer structure"]
impl crate::Writable for BK4TMGAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BK4TMGAT to value 0xfcfc_fcfc"]
impl crate::Resettable for BK4TMGAT_SPEC {
    const RESET_VALUE: Self::Ux = 0xfcfc_fcfc;
}