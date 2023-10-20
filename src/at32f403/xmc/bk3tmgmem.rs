#[doc = "Register `BK3TMGMEM` reader"]
pub type R = crate::R<BK3TMGMEM_SPEC>;
#[doc = "Register `BK3TMGMEM` writer"]
pub type W = crate::W<BK3TMGMEM_SPEC>;
#[doc = "Field `RGST` reader - Regular memory setup time"]
pub type RGST_R = crate::FieldReader;
#[doc = "Field `RGST` writer - Regular memory setup time"]
pub type RGST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGWT` reader - Regular memory wait time"]
pub type RGWT_R = crate::FieldReader;
#[doc = "Field `RGWT` writer - Regular memory wait time"]
pub type RGWT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGHT` reader - Regular memory hold time"]
pub type RGHT_R = crate::FieldReader;
#[doc = "Field `RGHT` writer - Regular memory hold time"]
pub type RGHT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGDHIZT` reader - Regular memory databus High resistance time"]
pub type RGDHIZT_R = crate::FieldReader;
#[doc = "Field `RGDHIZT` writer - Regular memory databus High resistance time"]
pub type RGDHIZT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK3TMGMEM")
            .field("rgdhizt", &format_args!("{}", self.rgdhizt().bits()))
            .field("rght", &format_args!("{}", self.rght().bits()))
            .field("rgwt", &format_args!("{}", self.rgwt().bits()))
            .field("rgst", &format_args!("{}", self.rgst().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<BK3TMGMEM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Regular memory setup time"]
    #[inline(always)]
    #[must_use]
    pub fn rgst(&mut self) -> RGST_W<BK3TMGMEM_SPEC> {
        RGST_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Regular memory wait time"]
    #[inline(always)]
    #[must_use]
    pub fn rgwt(&mut self) -> RGWT_W<BK3TMGMEM_SPEC> {
        RGWT_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Regular memory hold time"]
    #[inline(always)]
    #[must_use]
    pub fn rght(&mut self) -> RGHT_W<BK3TMGMEM_SPEC> {
        RGHT_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Regular memory databus High resistance time"]
    #[inline(always)]
    #[must_use]
    pub fn rgdhizt(&mut self) -> RGDHIZT_W<BK3TMGMEM_SPEC> {
        RGDHIZT_W::new(self, 24)
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
#[doc = "Regular memory space timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk3tmgmem::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk3tmgmem::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BK3TMGMEM_SPEC;
impl crate::RegisterSpec for BK3TMGMEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk3tmgmem::R`](R) reader structure"]
impl crate::Readable for BK3TMGMEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bk3tmgmem::W`](W) writer structure"]
impl crate::Writable for BK3TMGMEM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BK3TMGMEM to value 0xfcfc_fcfc"]
impl crate::Resettable for BK3TMGMEM_SPEC {
    const RESET_VALUE: Self::Ux = 0xfcfc_fcfc;
}
