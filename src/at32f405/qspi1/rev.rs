#[doc = "Register `REV` reader"]
pub type R = crate::R<REV_SPEC>;
#[doc = "Register `REV` writer"]
pub type W = crate::W<REV_SPEC>;
#[doc = "Field `REVISION` reader - Revision number"]
pub type REVISION_R = crate::FieldReader<u32>;
#[doc = "Field `REVISION` writer - Revision number"]
pub type REVISION_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 31, O, u32>;
impl R {
    #[doc = "Bits 0:30 - Revision number"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(self.bits & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REV")
            .field("revision", &format_args!("{}", self.revision().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<REV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:30 - Revision number"]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> REVISION_W<REV_SPEC, 0> {
        REVISION_W::new(self)
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
#[doc = "Revision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rev::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rev::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REV_SPEC;
impl crate::RegisterSpec for REV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rev::R`](R) reader structure"]
impl crate::Readable for REV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rev::W`](W) writer structure"]
impl crate::Writable for REV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REV to value 0x0001_0500"]
impl crate::Resettable for REV_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0500;
}
