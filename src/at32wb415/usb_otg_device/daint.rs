#[doc = "Register `DAINT` reader"]
pub type R = crate::R<DAINT_SPEC>;
#[doc = "Field `INEPTINT` reader - IN endpoint interrupt bits"]
pub type INEPTINT_R = crate::FieldReader<u16>;
#[doc = "Field `OUTEPTINT` reader - OUT endpoint interrupt bits"]
pub type OUTEPTINT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint interrupt bits"]
    #[inline(always)]
    pub fn ineptint(&self) -> INEPTINT_R {
        INEPTINT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn outeptint(&self) -> OUTEPTINT_R {
        OUTEPTINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "OTGFS device all endpoints interrupt register (OTGFS_DAINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAINT_SPEC;
impl crate::RegisterSpec for DAINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daint::R`](R) reader structure"]
impl crate::Readable for DAINT_SPEC {}
#[doc = "`reset()` method sets DAINT to value 0"]
impl crate::Resettable for DAINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
