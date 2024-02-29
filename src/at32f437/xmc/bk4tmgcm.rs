#[doc = "Register `BK4TMGCM` reader"]
pub type R = crate::R<BK4TMGCM_SPEC>;
#[doc = "Register `BK4TMGCM` writer"]
pub type W = crate::W<BK4TMGCM_SPEC>;
#[doc = "Field `CMST` reader - Regular memory setup time"]
pub type CMST_R = crate::FieldReader;
#[doc = "Field `CMST` writer - Regular memory setup time"]
pub type CMST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMWT` reader - Regular memory wait time"]
pub type CMWT_R = crate::FieldReader;
#[doc = "Field `CMWT` writer - Regular memory wait time"]
pub type CMWT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMHT` reader - Regular memory hold time"]
pub type CMHT_R = crate::FieldReader;
#[doc = "Field `CMHT` writer - Regular memory hold time"]
pub type CMHT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMDHIZT` reader - Regular memory databus High resistance time"]
pub type CMDHIZT_R = crate::FieldReader;
#[doc = "Field `CMDHIZT` writer - Regular memory databus High resistance time"]
pub type CMDHIZT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Regular memory setup time"]
    #[inline(always)]
    pub fn cmst(&self) -> CMST_R {
        CMST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Regular memory wait time"]
    #[inline(always)]
    pub fn cmwt(&self) -> CMWT_R {
        CMWT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Regular memory hold time"]
    #[inline(always)]
    pub fn cmht(&self) -> CMHT_R {
        CMHT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Regular memory databus High resistance time"]
    #[inline(always)]
    pub fn cmdhizt(&self) -> CMDHIZT_R {
        CMDHIZT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK4TMGCM")
            .field("cmdhizt", &format_args!("{}", self.cmdhizt().bits()))
            .field("cmht", &format_args!("{}", self.cmht().bits()))
            .field("cmwt", &format_args!("{}", self.cmwt().bits()))
            .field("cmst", &format_args!("{}", self.cmst().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<BK4TMGCM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Regular memory setup time"]
    #[inline(always)]
    #[must_use]
    pub fn cmst(&mut self) -> CMST_W<BK4TMGCM_SPEC> {
        CMST_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Regular memory wait time"]
    #[inline(always)]
    #[must_use]
    pub fn cmwt(&mut self) -> CMWT_W<BK4TMGCM_SPEC> {
        CMWT_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Regular memory hold time"]
    #[inline(always)]
    #[must_use]
    pub fn cmht(&mut self) -> CMHT_W<BK4TMGCM_SPEC> {
        CMHT_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Regular memory databus High resistance time"]
    #[inline(always)]
    #[must_use]
    pub fn cmdhizt(&mut self) -> CMDHIZT_W<BK4TMGCM_SPEC> {
        CMDHIZT_W::new(self, 24)
    }
}
#[doc = "Regular memory space timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4tmgcm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4tmgcm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BK4TMGCM_SPEC;
impl crate::RegisterSpec for BK4TMGCM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk4tmgcm::R`](R) reader structure"]
impl crate::Readable for BK4TMGCM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bk4tmgcm::W`](W) writer structure"]
impl crate::Writable for BK4TMGCM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BK4TMGCM to value 0xfcfc_fcfc"]
impl crate::Resettable for BK4TMGCM_SPEC {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
