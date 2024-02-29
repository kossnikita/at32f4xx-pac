#[doc = "Register `RSP2` reader"]
pub type R = crate::R<RSP2_SPEC>;
#[doc = "Field `CARDSTS2` reader - CARDSTATUS2"]
pub type CARDSTS2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS2"]
    #[inline(always)]
    pub fn cardsts2(&self) -> CARDSTS2_R {
        CARDSTS2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSP2")
            .field("cardsts2", &format_args!("{}", self.cardsts2().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<RSP2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Bits 31:0 = CARDSTATUS2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSP2_SPEC;
impl crate::RegisterSpec for RSP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsp2::R`](R) reader structure"]
impl crate::Readable for RSP2_SPEC {}
#[doc = "`reset()` method sets RSP2 to value 0"]
impl crate::Resettable for RSP2_SPEC {
    const RESET_VALUE: u32 = 0;
}
