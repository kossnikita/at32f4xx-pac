#[doc = "Register `DAINTMSK` reader"]
pub type R = crate::R<DAINTMSK_SPEC>;
#[doc = "Register `DAINTMSK` writer"]
pub type W = crate::W<DAINTMSK_SPEC>;
#[doc = "Field `INEPTMSK` reader - IN EP interrupt mask bits"]
pub type INEPTMSK_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTMSK` writer - IN EP interrupt mask bits"]
pub type INEPTMSK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `OUTEPTMSK` reader - OUT endpoint interrupt bits"]
pub type OUTEPTMSK_R = crate::FieldReader<u16>;
#[doc = "Field `OUTEPTMSK` writer - OUT endpoint interrupt bits"]
pub type OUTEPTMSK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - IN EP interrupt mask bits"]
    #[inline(always)]
    pub fn ineptmsk(&self) -> INEPTMSK_R {
        INEPTMSK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn outeptmsk(&self) -> OUTEPTMSK_R {
        OUTEPTMSK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAINTMSK")
            .field("ineptmsk", &format_args!("{}", self.ineptmsk().bits()))
            .field("outeptmsk", &format_args!("{}", self.outeptmsk().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DAINTMSK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP interrupt mask bits"]
    #[inline(always)]
    #[must_use]
    pub fn ineptmsk(&mut self) -> INEPTMSK_W<DAINTMSK_SPEC, 0> {
        INEPTMSK_W::new(self)
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    #[must_use]
    pub fn outeptmsk(&mut self) -> OUTEPTMSK_W<DAINTMSK_SPEC, 16> {
        OUTEPTMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTGFS all endpoints interrupt mask register (OTGFS_DAINTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAINTMSK_SPEC;
impl crate::RegisterSpec for DAINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daintmsk::R`](R) reader structure"]
impl crate::Readable for DAINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`daintmsk::W`](W) writer structure"]
impl crate::Writable for DAINTMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAINTMSK to value 0"]
impl crate::Resettable for DAINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
