#[doc = "Register `MACA0H` reader"]
pub type R = crate::R<MACA0H_SPEC>;
#[doc = "Register `MACA0H` writer"]
pub type W = crate::W<MACA0H_SPEC>;
#[doc = "Field `MA0H` reader - MAC address0 high"]
pub type MA0H_R = crate::FieldReader<u16>;
#[doc = "Field `MA0H` writer - MAC address0 high"]
pub type MA0H_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `AE` reader - Address enable"]
pub type AE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - MAC address0 high"]
    #[inline(always)]
    pub fn ma0h(&self) -> MA0H_R {
        MA0H_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA0H")
            .field("ma0h", &format_args!("{}", self.ma0h().bits()))
            .field("ae", &format_args!("{}", self.ae().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MACA0H_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address0 high"]
    #[inline(always)]
    #[must_use]
    pub fn ma0h(&mut self) -> MA0H_W<MACA0H_SPEC, 0> {
        MA0H_W::new(self)
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
#[doc = "Ethernet MAC address 0 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA0H_SPEC;
impl crate::RegisterSpec for MACA0H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca0h::R`](R) reader structure"]
impl crate::Readable for MACA0H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca0h::W`](W) writer structure"]
impl crate::Writable for MACA0H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACA0H to value 0x0010_ffff"]
impl crate::Resettable for MACA0H_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_ffff;
}
