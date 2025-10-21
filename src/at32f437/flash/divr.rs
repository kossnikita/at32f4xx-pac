#[doc = "Register `DIVR` reader"]
pub type R = crate::R<DIVR_SPEC>;
#[doc = "Register `DIVR` writer"]
pub type W = crate::W<DIVR_SPEC>;
#[doc = "Field `FDIV` reader - Flash divider"]
pub type FDIV_R = crate::FieldReader;
#[doc = "Field `FDIV` writer - Flash divider"]
pub type FDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FDIV_STS` reader - Flash divider status"]
pub type FDIV_STS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Flash divider"]
    #[inline(always)]
    pub fn fdiv(&self) -> FDIV_R {
        FDIV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Flash divider status"]
    #[inline(always)]
    pub fn fdiv_sts(&self) -> FDIV_STS_R {
        FDIV_STS_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIVR")
            .field("fdiv", &self.fdiv())
            .field("fdiv_sts", &self.fdiv_sts())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Flash divider"]
    #[inline(always)]
    pub fn fdiv(&mut self) -> FDIV_W<'_, DIVR_SPEC> {
        FDIV_W::new(self, 0)
    }
}
#[doc = "Flash divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIVR_SPEC;
impl crate::RegisterSpec for DIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divr::R`](R) reader structure"]
impl crate::Readable for DIVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`divr::W`](W) writer structure"]
impl crate::Writable for DIVR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIVR to value 0x22"]
impl crate::Resettable for DIVR_SPEC {
    const RESET_VALUE: u32 = 0x22;
}
