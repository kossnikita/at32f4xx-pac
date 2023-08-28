#[doc = "Register `RSP3` reader"]
pub type R = crate::R<RSP3_SPEC>;
#[doc = "Field `CARDSTS3` reader - CARDSTATUS3"]
pub type CARDSTS3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS3"]
    #[inline(always)]
    pub fn cardsts3(&self) -> CARDSTS3_R {
        CARDSTS3_R::new(self.bits)
    }
}
#[doc = "Bits 31:0 = CARDSTATUS3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSP3_SPEC;
impl crate::RegisterSpec for RSP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsp3::R`](R) reader structure"]
impl crate::Readable for RSP3_SPEC {}
#[doc = "`reset()` method sets RSP3 to value 0"]
impl crate::Resettable for RSP3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
