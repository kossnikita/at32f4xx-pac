#[doc = "Register `GDIV` reader"]
pub type R = crate::R<GDIV_SPEC>;
#[doc = "Register `GDIV` writer"]
pub type W = crate::W<GDIV_SPEC>;
#[doc = "Field `ISDIV` reader - IrDA/smartcard division value"]
pub type ISDIV_R = crate::FieldReader;
#[doc = "Field `ISDIV` writer - IrDA/smartcard division value"]
pub type ISDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
#[doc = "Field `SCGT` reader - Smart card guard time value"]
pub type SCGT_R = crate::FieldReader;
#[doc = "Field `SCGT` writer - Smart card guard time value"]
pub type SCGT_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - IrDA/smartcard division value"]
    #[inline(always)]
    pub fn isdiv(&self) -> ISDIV_R {
        ISDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Smart card guard time value"]
    #[inline(always)]
    pub fn scgt(&self) -> SCGT_R {
        SCGT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GDIV")
            .field("scgt", &self.scgt())
            .field("isdiv", &self.isdiv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - IrDA/smartcard division value"]
    #[inline(always)]
    #[must_use]
    pub fn isdiv(&mut self) -> ISDIV_W<GDIV_SPEC> {
        ISDIV_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Smart card guard time value"]
    #[inline(always)]
    #[must_use]
    pub fn scgt(&mut self) -> SCGT_W<GDIV_SPEC> {
        SCGT_W::new(self, 8)
    }
}
#[doc = "Guard time and division register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GDIV_SPEC;
impl crate::RegisterSpec for GDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdiv::R`](R) reader structure"]
impl crate::Readable for GDIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gdiv::W`](W) writer structure"]
impl crate::Writable for GDIV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GDIV to value 0"]
impl crate::Resettable for GDIV_SPEC {
    const RESET_VALUE: u32 = 0;
}
