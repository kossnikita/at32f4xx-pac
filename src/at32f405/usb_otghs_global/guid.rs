#[doc = "Register `GUID` reader"]
pub type R = crate::R<GUID_SPEC>;
#[doc = "Register `GUID` writer"]
pub type W = crate::W<GUID_SPEC>;
#[doc = "Field `USERID` reader - Product ID field"]
pub type USERID_R = crate::FieldReader<u32>;
#[doc = "Field `USERID` writer - Product ID field"]
pub type USERID_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Product ID field"]
    #[inline(always)]
    pub fn userid(&self) -> USERID_R {
        USERID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GUID")
            .field("userid", &self.userid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Product ID field"]
    #[inline(always)]
    #[must_use]
    pub fn userid(&mut self) -> USERID_W<GUID_SPEC> {
        USERID_W::new(self, 0)
    }
}
#[doc = "Product ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`guid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`guid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GUID_SPEC;
impl crate::RegisterSpec for GUID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`guid::R`](R) reader structure"]
impl crate::Readable for GUID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`guid::W`](W) writer structure"]
impl crate::Writable for GUID_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GUID to value 0x1000"]
impl crate::Resettable for GUID_SPEC {
    const RESET_VALUE: u32 = 0x1000;
}
