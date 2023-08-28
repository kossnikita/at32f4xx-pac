#[doc = "Register `BK4TMGMEM` reader"]
pub type R = crate::R<BK4TMGMEM_SPEC>;
#[doc = "Register `BK4TMGMEM` writer"]
pub type W = crate::W<BK4TMGMEM_SPEC>;
#[doc = "Field `RGST` reader - Regular memory setup time"]
pub type RGST_R = crate::FieldReader;
#[doc = "Field `RGST` writer - Regular memory setup time"]
pub type RGST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `RGWT` reader - Regular memory wait time"]
pub type RGWT_R = crate::FieldReader;
#[doc = "Field `RGWT` writer - Regular memory wait time"]
pub type RGWT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `RGHT` reader - Regular memory hold time"]
pub type RGHT_R = crate::FieldReader;
#[doc = "Field `RGHT` writer - Regular memory hold time"]
pub type RGHT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `RGDHIZT` reader - Regular memory databus High resistance time"]
pub type RGDHIZT_R = crate::FieldReader;
#[doc = "Field `RGDHIZT` writer - Regular memory databus High resistance time"]
pub type RGDHIZT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Regular memory setup time"]
    #[inline(always)]
    pub fn rgst(&self) -> RGST_R {
        RGST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Regular memory wait time"]
    #[inline(always)]
    pub fn rgwt(&self) -> RGWT_R {
        RGWT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Regular memory hold time"]
    #[inline(always)]
    pub fn rght(&self) -> RGHT_R {
        RGHT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Regular memory databus High resistance time"]
    #[inline(always)]
    pub fn rgdhizt(&self) -> RGDHIZT_R {
        RGDHIZT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Regular memory setup time"]
    #[inline(always)]
    #[must_use]
    pub fn rgst(&mut self) -> RGST_W<BK4TMGMEM_SPEC, 0> {
        RGST_W::new(self)
    }
    #[doc = "Bits 8:15 - Regular memory wait time"]
    #[inline(always)]
    #[must_use]
    pub fn rgwt(&mut self) -> RGWT_W<BK4TMGMEM_SPEC, 8> {
        RGWT_W::new(self)
    }
    #[doc = "Bits 16:23 - Regular memory hold time"]
    #[inline(always)]
    #[must_use]
    pub fn rght(&mut self) -> RGHT_W<BK4TMGMEM_SPEC, 16> {
        RGHT_W::new(self)
    }
    #[doc = "Bits 24:31 - Regular memory databus High resistance time"]
    #[inline(always)]
    #[must_use]
    pub fn rgdhizt(&mut self) -> RGDHIZT_W<BK4TMGMEM_SPEC, 24> {
        RGDHIZT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Regular memory space timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4tmgmem::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4tmgmem::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BK4TMGMEM_SPEC;
impl crate::RegisterSpec for BK4TMGMEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk4tmgmem::R`](R) reader structure"]
impl crate::Readable for BK4TMGMEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bk4tmgmem::W`](W) writer structure"]
impl crate::Writable for BK4TMGMEM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BK4TMGMEM to value 0xfcfc_fcfc"]
impl crate::Resettable for BK4TMGMEM_SPEC {
    const RESET_VALUE: Self::Ux = 0xfcfc_fcfc;
}
