#[doc = "Register `RSP4` reader"]
pub type R = crate::R<RSP4_SPEC>;
#[doc = "Field `CARDSTS4` reader - CARDSTATUS4"]
pub type CARDSTS4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS4"]
    #[inline(always)]
    pub fn cardsts4(&self) -> CARDSTS4_R {
        CARDSTS4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSP4")
            .field("cardsts4", &format_args!("{}", self.cardsts4().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<RSP4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Bits 31:0 = CARDSTATUS4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSP4_SPEC;
impl crate::RegisterSpec for RSP4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsp4::R`](R) reader structure"]
impl crate::Readable for RSP4_SPEC {}
#[doc = "`reset()` method sets RSP4 to value 0"]
impl crate::Resettable for RSP4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
