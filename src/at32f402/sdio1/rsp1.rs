#[doc = "Register `RSP1` reader"]
pub type R = crate::R<RSP1_SPEC>;
#[doc = "Field `CARDSTS1` reader - CARDSTATUS1"]
pub type CARDSTS1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS1"]
    #[inline(always)]
    pub fn cardsts1(&self) -> CARDSTS1_R {
        CARDSTS1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSP1")
            .field("cardsts1", &format_args!("{}", self.cardsts1().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<RSP1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Bits 31:0 = CARDSTATUS1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSP1_SPEC;
impl crate::RegisterSpec for RSP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsp1::R`](R) reader structure"]
impl crate::Readable for RSP1_SPEC {}
#[doc = "`reset()` method sets RSP1 to value 0"]
impl crate::Resettable for RSP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
