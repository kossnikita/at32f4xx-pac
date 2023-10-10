#[doc = "Register `GDIV` reader"]
pub type R = crate::R<GDIV_SPEC>;
#[doc = "Register `GDIV` writer"]
pub type W = crate::W<GDIV_SPEC>;
#[doc = "Field `ISDIV` reader - IrDA/smartcard division value"]
pub type ISDIV_R = crate::FieldReader;
#[doc = "Field `ISDIV` writer - IrDA/smartcard division value"]
pub type ISDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SCGT` reader - Smart card guard time value"]
pub type SCGT_R = crate::FieldReader;
#[doc = "Field `SCGT` writer - Smart card guard time value"]
pub type SCGT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
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
            .field("scgt", &format_args!("{}", self.scgt().bits()))
            .field("isdiv", &format_args!("{}", self.isdiv().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GDIV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - IrDA/smartcard division value"]
    #[inline(always)]
    #[must_use]
    pub fn isdiv(&mut self) -> ISDIV_W<GDIV_SPEC, 0> {
        ISDIV_W::new(self)
    }
    #[doc = "Bits 8:15 - Smart card guard time value"]
    #[inline(always)]
    #[must_use]
    pub fn scgt(&mut self) -> SCGT_W<GDIV_SPEC, 8> {
        SCGT_W::new(self)
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
#[doc = "Guard time and division register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GDIV_SPEC;
impl crate::RegisterSpec for GDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdiv::R`](R) reader structure"]
impl crate::Readable for GDIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gdiv::W`](W) writer structure"]
impl crate::Writable for GDIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GDIV to value 0"]
impl crate::Resettable for GDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
