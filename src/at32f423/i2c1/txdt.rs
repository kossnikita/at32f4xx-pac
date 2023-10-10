#[doc = "Register `TXDT` reader"]
pub type R = crate::R<TXDT_SPEC>;
#[doc = "Register `TXDT` writer"]
pub type W = crate::W<TXDT_SPEC>;
#[doc = "Field `DT` reader - Transmit data register"]
pub type DT_R = crate::FieldReader;
#[doc = "Field `DT` writer - Transmit data register"]
pub type DT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Transmit data register"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXDT")
            .field("dt", &format_args!("{}", self.dt().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXDT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit data register"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<TXDT_SPEC, 0> {
        DT_W::new(self)
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
#[doc = "Transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDT_SPEC;
impl crate::RegisterSpec for TXDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdt::R`](R) reader structure"]
impl crate::Readable for TXDT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txdt::W`](W) writer structure"]
impl crate::Writable for TXDT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDT to value 0"]
impl crate::Resettable for TXDT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
