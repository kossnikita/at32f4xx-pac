#[doc = "Register `DEVADDR` reader"]
pub type R = crate::R<DEVADDR_SPEC>;
#[doc = "Register `DEVADDR` writer"]
pub type W = crate::W<DEVADDR_SPEC>;
#[doc = "Field `ADDR` reader - Host assign device address"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - Host assign device address"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `CEN` reader - USB core enable"]
pub type CEN_R = crate::BitReader;
#[doc = "Field `CEN` writer - USB core enable"]
pub type CEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - Host assign device address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - USB core enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVADDR")
            .field("addr", &format_args!("{}", self.addr().bits()))
            .field("cen", &format_args!("{}", self.cen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DEVADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Host assign device address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<DEVADDR_SPEC, 0> {
        ADDR_W::new(self)
    }
    #[doc = "Bit 7 - USB core enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<DEVADDR_SPEC, 7> {
        CEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "device address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVADDR_SPEC;
impl crate::RegisterSpec for DEVADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devaddr::R`](R) reader structure"]
impl crate::Readable for DEVADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`devaddr::W`](W) writer structure"]
impl crate::Writable for DEVADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVADDR to value 0"]
impl crate::Resettable for DEVADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
