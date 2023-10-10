#[doc = "Register `BK3TMGSP` reader"]
pub type R = crate::R<BK3TMGSP_SPEC>;
#[doc = "Register `BK3TMGSP` writer"]
pub type W = crate::W<BK3TMGSP_SPEC>;
#[doc = "Field `SPST` reader - special memory setup time"]
pub type SPST_R = crate::FieldReader;
#[doc = "Field `SPST` writer - special memory setup time"]
pub type SPST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SPWT` reader - special memory wait time"]
pub type SPWT_R = crate::FieldReader;
#[doc = "Field `SPWT` writer - special memory wait time"]
pub type SPWT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SPHT` reader - special memory hold time"]
pub type SPHT_R = crate::FieldReader;
#[doc = "Field `SPHT` writer - special memory hold time"]
pub type SPHT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SPDHIZT` reader - special memory databus High resistance time"]
pub type SPDHIZT_R = crate::FieldReader;
#[doc = "Field `SPDHIZT` writer - special memory databus High resistance time"]
pub type SPDHIZT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
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
        f.debug_struct("BK3TMGSP")
            .field("spdhizt", &format_args!("{}", self.spdhizt().bits()))
            .field("spht", &format_args!("{}", self.spht().bits()))
            .field("spwt", &format_args!("{}", self.spwt().bits()))
            .field("spst", &format_args!("{}", self.spst().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<BK3TMGSP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - special memory setup time"]
    #[inline(always)]
    #[must_use]
    pub fn spst(&mut self) -> SPST_W<BK3TMGSP_SPEC, 0> {
        SPST_W::new(self)
    }
    #[doc = "Bits 8:15 - special memory wait time"]
    #[inline(always)]
    #[must_use]
    pub fn spwt(&mut self) -> SPWT_W<BK3TMGSP_SPEC, 8> {
        SPWT_W::new(self)
    }
    #[doc = "Bits 16:23 - special memory hold time"]
    #[inline(always)]
    #[must_use]
    pub fn spht(&mut self) -> SPHT_W<BK3TMGSP_SPEC, 16> {
        SPHT_W::new(self)
    }
    #[doc = "Bits 24:31 - special memory databus High resistance time"]
    #[inline(always)]
    #[must_use]
    pub fn spdhizt(&mut self) -> SPDHIZT_W<BK3TMGSP_SPEC, 24> {
        SPDHIZT_W::new(self)
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
#[doc = "special memory space timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk3tmgsp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk3tmgsp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BK3TMGSP_SPEC;
impl crate::RegisterSpec for BK3TMGSP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk3tmgsp::R`](R) reader structure"]
impl crate::Readable for BK3TMGSP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bk3tmgsp::W`](W) writer structure"]
impl crate::Writable for BK3TMGSP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BK3TMGSP to value 0xfcfc_fcfc"]
impl crate::Resettable for BK3TMGSP_SPEC {
    const RESET_VALUE: Self::Ux = 0xfcfc_fcfc;
}
