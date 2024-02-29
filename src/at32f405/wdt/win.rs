#[doc = "Register `WIN` reader"]
pub type R = crate::R<WIN_SPEC>;
#[doc = "Field `WIN` reader - Window value"]
pub type WIN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Window value"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIN")
            .field("win", &format_args!("{}", self.win().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<WIN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Window register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIN_SPEC;
impl crate::RegisterSpec for WIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win::R`](R) reader structure"]
impl crate::Readable for WIN_SPEC {}
#[doc = "`reset()` method sets WIN to value 0x0fff"]
impl crate::Resettable for WIN_SPEC {
    const RESET_VALUE: u32 = 0x0fff;
}
