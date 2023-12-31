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
            .field("userid", &format_args!("{}", self.userid().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GUID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Product ID field"]
    #[inline(always)]
    #[must_use]
    pub fn userid(&mut self) -> USERID_W<GUID_SPEC> {
        USERID_W::new(self, 0)
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
#[doc = "Product ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`guid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`guid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GUID_SPEC;
impl crate::RegisterSpec for GUID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`guid::R`](R) reader structure"]
impl crate::Readable for GUID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`guid::W`](W) writer structure"]
impl crate::Writable for GUID_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GUID to value 0x1000"]
impl crate::Resettable for GUID_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
